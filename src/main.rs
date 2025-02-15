#![allow(non_snake_case)]
use axum::{response::Html, routing::get, Router};
use tokio::signal;
use shtml::{html, Elements, Component, Render};
use tower_http::services::ServeDir;

#[derive(Clone,Copy)]
enum TokenType{
    Player,
    Monster,
    Item,
    None
}

#[derive(Clone, Copy)]
struct Token{
    pub token_type : TokenType
}

#[derive(Clone,Copy)]
enum DoorState{
    Open,
    ClosedLocked,
    Closed,
    Broken
}

#[derive(Clone, Copy)]
struct TileStack{
    pub stack : [Token; 100]
}

impl TileStack{
    pub fn new() -> TileStack{
        TileStack{
            stack : [Token {token_type : TokenType::None} ; 100]
        }
    }
}

#[derive(Clone, Copy)]
enum Tile{
    Wall,
    Floor(TileStack),
    Door(DoorState, TileStack),
    None
}

#[derive(Clone)]
struct GameBoard{
    pub game_board : [[Tile ; 50] ; 50]
}

impl GameBoard {
    pub fn new_empty() -> GameBoard {
        let board : [[Tile ; 50] ; 50] = [[Tile::None ; 50] ; 50];
        GameBoard{
            game_board : board
        }  
    }
}

#[derive(Clone)]
struct AppState{
    game_board : GameBoard
}

impl AppState {
    pub fn new_server_state() -> AppState{
        AppState{
            game_board : GameBoard::new_empty()
        }
    }

    pub fn new_game_board(self) -> AppState {
        // Draw the upper wall
        let mut upper_wall_row = self.game_board.game_board[0];
        for y in 0 .. 10 {
            upper_wall_row[y] = Tile::Wall;
        }
        // Draw the lower wall
        let mut lower_wall_row = self.game_board.game_board[9];
        for y in 0 .. 10 {
            lower_wall_row[y] = Tile::Wall;
        }
        // Fill the floor
        for x in 1 .. 9 {
            let mut row = self.game_board.game_board[x];
            for y in 1 .. 9{
                row[y] = Tile::Floor(TileStack::new());
            }
        }
        self
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Rat HTTP");
    let state = AppState::new_server_state().new_game_board();
    let app = Router::new()
        .route("/", get(handler))
        .route("/new_game", get(new_game_handler))
        .route("/move_right", get(move_right_handler))
        .route("/move_left", get(move_left_handler))
        .route("/move_down", get(move_down_handler))
        .route("/move_up", get(move_up_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Listing on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn handler() -> Html<String> {
    println!("Main Menu Accessed");
    let output = html!{
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <script src="/static/htmx/htmx.min.js"></script>
            </head>
            <body>
                <h1>RAT.HTML</h1>
                <h6>An HTML Only Rogue - Like  by Patrick Phillips</h6>
                <h6>Check out the source on github!</h6>
                <button 
                    hx-get="/new_game"
                    hx-target="#game-holding"
                    hx-swap="outerHTML"
                    id="game-holding">
                    Click Me!
                    
                </button>
                <div>
                    <p>Powered by the SHAT STACK!</p>
                </div>
            </body>
        </html>
    }.to_string();
    Html(output)
}


fn PlayerInputBlock() -> Component {
    html!{
        <div>
            <div hx-get="/move_right" hx-trigger="keyup[key=='ArrowRight'] from:body once" hx-target="#game-target"></div>
            <div hx-get="/move_left" hx-trigger="keyup[key=='ArrowLeft'] from:body once" hx-target="#game-target"></div>
            <div hx-get="/move_down" hx-trigger="keyup[key=='ArrowDown'] from:body once" hx-target="#game-target"></div>
            <div hx-get="/move_up" hx-trigger="keyup[key=='ArrowUp'] from:body once" hx-target="#game-target"></div>
        </div>
    }
}

async fn move_right_handler() -> Html<String>{
    println!("move right");
    let output = html!{
        <PlayerInputBlock/>
        <div id="game-target">
            <h1>You pressed right</h1>
        </div>
    }.to_string();
    Html(output)
}

async fn move_left_handler() -> Html<String>{
    println!("move left");
    let output = html!{
        <PlayerInputBlock/>
        <div id="game-target">
            <h1>You pressed left</h1>
        </div>
    }.to_string();
    Html(output)
}

async fn move_down_handler() -> Html<String>{
    println!("move down");

    let output = html!{
        <PlayerInputBlock/>
        <div id="game-target">
            <h1>You pressed down</h1>
        </div>
    }.to_string();
    Html(output)
}

async fn move_up_handler() -> Html<String>{
    println!("move up");
    
    let output = html!{
        <PlayerInputBlock/>
        <div id="game-target">
            <h1>You pressed up</h1>
        </div>
    }.to_string();
    Html(output)
}

async fn new_game_handler() ->Html<String>{
    println!("new game started");
    let output = html!{
        <PlayerInputBlock/>
        <div id="game-target">
            <table>
                <tr>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>@</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>.</td>
                    <td>#</td>
                </tr>
                <tr>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                    <td>#</td>
                </tr>
            </table>
        </div>
    }.to_string();
    return Html(output)
}

async fn shutdown_signal(){
    let ctrl_c = async {
        signal::ctrl_c()
        .await
        .expect("Failed to get ctrl-c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to install signal handler")
        .recv()
        .await;
    };

    tokio::select! {
        _ =ctrl_c => {},
        _ = terminate => {},
    }
    println!("Graceful shutdown initiated");
}
