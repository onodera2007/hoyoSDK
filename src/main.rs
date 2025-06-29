use std::{
    process::ExitCode,
    sync::{LazyLock, OnceLock},
};

use axum::Router;
use config::SdkConfig;
use database::DbContext;
use handlers::{combo_granter, mdk_shield_api, register, risky_api};
use tokio::net::TcpListener;
use tracing::error;

mod config;
mod database;
mod handlers;
mod util;

use std::thread;
use std::time::Duration;

struct AppState {
    db: DbContext,
    #[expect(dead_code)]
    config: &'static SdkConfig,
}

type AppStateRef = &'static AppState;

#[tokio::main]
async fn main() -> ExitCode {
    static CONFIG: LazyLock<SdkConfig> =
        LazyLock::new(|| config::load_or_create("sdk_server.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();
    println!("这个dsk是由");
     thread::sleep(Duration::from_secs(1));
    println!(r#"
          _____                    _____                    _____                    _____          
         /\    \                  /\    \                  /\    \                  /\    \         
        /::\    \                /::\    \                /::\____\                /::\    \        
       /::::\    \              /::::\    \              /::::|   |               /::::\    \       
      /::::::\    \            /::::::\    \            /:::::|   |              /::::::\    \      
     /:::/\:::\    \          /:::/\:::\    \          /::::::|   |             /:::/\:::\    \     
    /:::/  \:::\    \        /:::/__\:::\    \        /:::/|::|   |            /:::/  \:::\    \    
   /:::/    \:::\    \      /::::\   \:::\    \      /:::/ |::|   |           /:::/    \:::\    \   
  /:::/    / \:::\    \    /::::::\   \:::\    \    /:::/  |::|   | _____    /:::/    / \:::\    \  
 /:::/    /   \:::\ ___\  /:::/\:::\   \:::\    \  /:::/   |::|   |/\    \  /:::/    /   \:::\ ___\ 
/:::/____/  ___\:::|    |/:::/__\:::\   \:::\____\/:: /    |::|   /::\____\/:::/____/  ___\:::|    |
\:::\    \ /\  /:::|____|\:::\   \:::\   \::/    /\::/    /|::|  /:::/    /\:::\    \ /\  /:::|____|
 \:::\    /::\ \::/    /  \:::\   \:::\   \/____/  \/____/ |::| /:::/    /  \:::\    /::\ \::/    / 
  \:::\   \:::\ \/____/    \:::\   \:::\    \              |::|/:::/    /    \:::\   \:::\ \/____/  
   \:::\   \:::\____\       \:::\   \:::\____\             |::::::/    /      \:::\   \:::\____\    
    \:::\  /:::/    /        \:::\   \::/    /             |:::::/    /        \:::\  /:::/    /    
     \:::\/:::/    /          \:::\   \/____/              |::::/    /          \:::\/:::/    /     
      \::::::/    /            \:::\    \                  /:::/    /            \::::::/    /      
       \::::/    /              \:::\____\                /:::/    /              \::::/    /       
        \::/____/                \::/    /                \::/    /                \::/____/        
                                  \/____/                  \/____/                                  
"#);
thread::sleep(Duration::from_secs(1));
    println!(r#"
      _____                   _______                   _____          
     |\    \                 /::\    \                 /\    \         
     |:\____\               /::::\    \               /::\____\        
     |::|   |              /::::::\    \             /:::/    /        
     |::|   |             /::::::::\    \           /:::/    /         
     |::|   |            /:::/~~\:::\    \         /:::/    /          
     |::|   |           /:::/    \:::\    \       /:::/    /           
     |::|   |          /:::/    / \:::\    \     /:::/    /            
     |::|___|______   /:::/____/   \:::\____\   /:::/    /      _____  
     /::::::::\    \ |:::|    |     |:::|    | /:::/____/      /\    \ 
    /::::::::::\____\|:::|____|     |:::|    ||:::|    /      /::\____\
   /:::/~~~~/~~       \:::\    \   /:::/    / |:::|____\     /:::/    /
  /:::/    /           \:::\    \ /:::/    /   \:::\    \   /:::/    / 
 /:::/    /             \:::\    /:::/    /     \:::\    \ /:::/    /  
/:::/    /               \:::\__/:::/    /       \:::\    /:::/    /   
\::/    /                 \::::::::/    /         \:::\__/:::/    /    
 \/____/                   \::::::/    /           \::::::::/    /     
                            \::::/    /             \::::::/    /      
                             \::/____/               \::::/    /       
                              ~~                      \::/____/        
                                                       ~~              
    "#);
    thread::sleep(Duration::from_secs(1));
    println!(r#"
          _____                   _______         
         /\    \                 /::\    \        
        /::\    \               /::::\    \       
       /::::\    \             /::::::\    \      
      /::::::\    \           /::::::::\    \     
     /:::/\:::\    \         /:::/~~\:::\    \    
    /:::/__\:::\    \       /:::/    \:::\    \   
   /::::\   \:::\    \     /:::/    / \:::\    \  
  /::::::\   \:::\    \   /:::/____/   \:::\____\ 
 /:::/\:::\   \:::\ ___\ |:::|    |     |:::|    |
/:::/__\:::\   \:::|    ||:::|____|     |:::|    |
\:::\   \:::\  /:::|____| \:::\    \   /:::/    / 
 \:::\   \:::\/:::/    /   \:::\    \ /:::/    /  
  \:::\   \::::::/    /     \:::\    /:::/    /   
   \:::\   \::::/    /       \:::\__/:::/    /    
    \:::\  /:::/    /         \::::::::/    /     
     \:::\/:::/    /           \::::::/    /      
      \::::::/    /             \::::/    /       
       \::::/    /               \::/____/        
        \::/____/                 ~~              
         ~~                                     
    "#);
    thread::sleep(Duration::from_secs(1));
    println!("修改，如果可以请支持");
    println!("
  _____                                  _ _____                           \n |  __ \\                                | |  __ \\                          \n | |__) |_____   _____ _ __ ___  ___  __| | |__) |___   ___  _ __ ___  ___ \n |  _  // _ \\ \\ / / _ \\ '__/ __|/ _ \\/ _` |  _  // _ \\ / _ \\| '_ ` _ \\/ __|\n | | \\ \\  __/\\ V /  __/ |  \\__ \\  __/ (_| | | \\ \\ (_) | (_) | | | | | \\__ \\\n |_|  \\_\\___| \\_/ \\___|_|  |___/\\___|\\__,_|_|  \\_\\___/ \\___/|_| |_| |_|___/");
  thread::sleep(Duration::from_secs(1));
    println!("启动后访问http://127.0.0.1:20100/account/register 后随便输入账号密码即可注册一个账号");

    init_tracing();
    let db = match DbContext::connect(&CONFIG.db_file).await {
        Ok(db) => db,
        Err(err) => {
            error!("Failed to open SQLite database. Error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let _ = STATE.set(AppState {
        db,
        config: &CONFIG,
    });

    let router = Router::new()
        .merge(risky_api::routes())
        .merge(register::routes())
        .merge(mdk_shield_api::routes())
        .merge(combo_granter::routes())
        .with_state(STATE.get().unwrap());

    let listener = TcpListener::bind(&CONFIG.http_addr)
        .await
        .expect("TcpListener::bind failed. Is another instance of this server already running?");

    axum::serve(listener, router).await.unwrap();

    ExitCode::SUCCESS
}

fn init_tracing() {
    tracing_subscriber::fmt().without_time().init();
}
