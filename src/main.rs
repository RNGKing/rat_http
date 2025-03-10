#![allow(non_snake_case)]
use axum::{response::Html, routing::get, Router, extract::State};
use tokio::signal;
use shtml::{html, Elements, Component, Render};
use tower_http::services::ServeDir;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
        let mut input_stack : [Token ; 100] = [Token {token_type : TokenType::None} ; 100];
        for x in 0 .. 100 {
            input_stack[x] = Token{token_type : TokenType::None};
        }
        TileStack{
            stack : input_stack
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

enum InputDirection{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct GameBoard{
    pub game_board : [[Tile ; 100] ; 100]
}

impl GameBoard {
    pub fn new_empty() -> GameBoard {
        let board : [[Tile ; 100] ; 100] = [[Tile::None ; 100] ; 100];
        GameBoard{
            game_board : board
        }  
    }

    pub fn update_game_state(&mut self, direction : InputDirection){
        println!("Being called from the input");
        
        match direction {
            InputDirection::UP => println!("move up"),
            InputDirection::DOWN => println!("move down"),
            InputDirection::LEFT => println!("move left"),
            InputDirection::RIGHT => println!("move right"),
        }
    }
}

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

#[derive(Clone)]
struct ApplicationState(Arc<Mutex<AppState>>);

#[tokio::main]
async fn main() {
    println!("Starting Rat HTTP");
    let state = ApplicationState(Arc::new(Mutex::new(AppState::new_server_state())));
    let app = Router::new()
        .route("/", get(index_handler))
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

async fn index_handler() -> Html<String> {
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
                <h6>Check out the source on <a href="https://github.com/RNGKing/rat_http/tree/main"> github!</a></h6>
                <button 
                    hx-get="/new_game"
                    hx-target="#game-holding"
                    hx-swap="outerHTML"
                    id="game-holding">
                    New Game
                </button>
                <div>
                    <p>Powered by the <a href="https://github.com/ChristianPavilonis/shat-stack"> SHAT STACK!</a></p>
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

fn RenderGameBoard(game_board : &mut GameBoard) -> Component {
    html!{
        <svg width="500" height="500">
            // upper wall
            <text style="font-family: monospace, monospace;" x="15" y="15" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="30" y="15" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="45" y="15" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="60" y="15" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="75" y="15" fill="red">#</text>
            // lower wall
            <text style="font-family: monospace, monospace;" x="15" y="90" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="30" y="90" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="45" y="90" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="60" y="90" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="75" y="90" fill="red">#</text>
            // left wall
            <text style="font-family: monospace, monospace;" x="15" y="30" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="15" y="45" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="15" y="60" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="15" y="75" fill="red">#</text>
            // right wall
            <text style="font-family: monospace, monospace;" x="75" y="30" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="75" y="45" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="75" y="60" fill="red">#</text>
            <text style="font-family: monospace, monospace;" x="75" y="75" fill="red">#</text>
            
        </svg>
    }
}

fn TestOutputBlock() -> Component{
    html!{
        <svg width="100" height="100">
            <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
        </svg>
    }
}

async fn move_right_handler(State(state) : State<ApplicationState>) -> Html<String>{
    let output : String = match state.0.lock(){
        Ok(mut app_state) => {
            let board = &mut app_state.game_board;
            board.update_game_state(InputDirection::RIGHT);
            html!{
                <PlayerInputBlock/>
                <div id="game-target">
                    <RenderGameBoard game_board=board/>
                </div>
            }.to_string()
        },
        Err(_) => {
            html!{
                <div>
                    <h1>Major error when pressing right</h1>
                </div>
            }.to_string()
        }
    };
    Html(output)
}

async fn move_left_handler(State(state) : State<ApplicationState>) -> Html<String>{
    let output : String = match state.0.lock(){
        Ok(mut app_state) => {
            let board = &mut app_state.game_board;
            board.update_game_state(InputDirection::LEFT);
            html!{
                <PlayerInputBlock/>
                <div id="game-target">
                    <h1>You pressed left</h1>
                </div>
            }.to_string()
        },
        Err(_) => {
            html!{
                <div>
                    <h1>Major error when pressing left</h1>
                </div>
            }.to_string()
        }
    };
    Html(output)
}

async fn move_down_handler(State(state) : State<ApplicationState>) -> Html<String>{
    let output : String = match state.0.lock(){
        Ok(mut app_state) => {
            let board = &mut app_state.game_board;
            board.update_game_state(InputDirection::DOWN);
            html!{
                <PlayerInputBlock/>
                <div id="game-target">
                    <h1>You pressed down</h1>
                </div>
            }.to_string()
        },
        Err(_) => {
            html!{
                <div>
                    <h1>Major error when pressing down</h1>
                </div>
            }.to_string()
        }
    };
    Html(output)
}

async fn move_up_handler(State(state) : State<ApplicationState>) -> Html<String>{
    let output : String = match state.0.lock(){
        Ok(mut app_state) => {
            let board = &mut app_state.game_board;
            board.update_game_state(InputDirection::UP);
            html!{
                <PlayerInputBlock/>
                <div id="game-target">
                    <h1>You pressed up</h1>
                </div>
            }.to_string()
        },
        Err(_) => {
            html!{
                <div>
                    <h1>Major error when pressing up</h1>
                </div>
            }.to_string()
        }
    };
    Html(output)
}

async fn new_game_handler() -> Html<String>{
    println!("new game started");
    let output = html!{
        <div id="game-target">
            <PlayerInputBlock/>
            <TestOutputBlock/>
        </div>
        //</div>
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