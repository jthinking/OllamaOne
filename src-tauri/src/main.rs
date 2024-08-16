#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app_config;
mod fetch;

use crate::fetch::fetch_stream;
use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::service::ContainerSummary;
use bollard::Docker;
use entity::{chat, config};
use futures_util::stream;
use futures_util::stream::StreamExt;
use migration::{Migrator, MigratorTrait};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use service::sea_orm::TryIntoModel;
use service::{
    sea_orm::{Database, DatabaseConnection},
    ChatMutation as ChatMutationCore, ChatQuery as ChatQueryCore,
    ConfigMutation as ConfigMutationCore, ConfigQuery as ConfigQueryCore,
};
use std::collections::HashMap;
use std::{env, fs};
use tauri::{AppHandle, Manager};
use tiktoken_rs::{p50k_base, CoreBPE};

#[macro_use]
extern crate lazy_static;

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    //let _ = _test_docker().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            APP_HANDLE.get_or_init(|| app.handle().clone());

            tauri::async_runtime::spawn(async move {
                let app_handle = APP_HANDLE.get().unwrap();
                let home_dir = app_handle
                    .path()
                    .home_dir()
                    .expect("failed to get home dir");
                let data_dir = home_dir.join(".ollamaone/data");
                if let Err(_) = fs::metadata(&data_dir) {
                    fs::create_dir_all(&data_dir).expect("Could not create data directory");
                }
                let db_url =
                    "sqlite://".to_string() + data_dir.to_str().unwrap() + "/db.sqlite?mode=rwc";

                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");
                Migrator::up(&conn, None).await.unwrap();

                let state = AppState { conn };

                app_handle.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_machine_uid,
            get_mac_address,
            sha2_256,
            device_id,
            md5,
            count_token,
            fetch_stream,
            create_config,
            update_config,
            delete_config,
            list_configs,
            get_config,
            create_chat,
            update_chat,
            delete_chat,
            list_chats,
            get_chat,
            crawl_url,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                println!("window is closing");
                api.prevent_close();

                tauri::async_runtime::spawn(async move {
                    let app_handle = APP_HANDLE.get().unwrap();
                    let state: tauri::State<'_, AppState> = app_handle.state();
                    let _ = state.conn.clone().close().await;
                    app_handle.exit(0);
                });
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    static ref BPE: CoreBPE = p50k_base().unwrap();
}

#[tauri::command]
fn count_token(text: &str) -> usize {
    let tokens = BPE.encode_with_special_tokens(text);
    tokens.len()
}

#[tauri::command]
fn get_machine_uid() -> String {
    let result = machine_uid::get();
    if result.is_err() {
        return "".to_string();
    }
    return result.unwrap();
}

#[tauri::command]
fn get_mac_address() -> String {
    match mac_address::get_mac_address() {
        Ok(Some(ma)) => ma.to_string(),
        Ok(None) => "".to_string(),
        Err(e) => {
            println!("{:?}", e);
            "".to_string()
        }
    }
}

#[tauri::command]
fn device_id() -> String {
    let uid = get_machine_uid();
    if uid == "" {
        md5(get_mac_address())
    } else {
        md5(uid)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductActivation {
    pub product_key: String,
    pub device_id: Option<String>,
    pub status: i32,
    pub activation_time: Option<i64>,
    pub expiration_time: Option<i64>,
    pub validity_period: i64,
}

#[tauri::command]
fn sha2_256(value: String) -> String {
    use sha2::{Digest, Sha256};

    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(value);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    // convert to hex string
    format!("{:x}", result)
}

#[tauri::command]
fn md5(value: String) -> String {
    use md5::{Digest, Md5};

    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    // process input message
    hasher.update(value);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    // convert to hex string
    format!("{:x}", result)
}

async fn _test_docker() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("status", vec!["running"]);

    let containers = &docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: list_container_filters,
            ..Default::default()
        }))
        .await?;

    let docker_stream = stream::repeat(docker);
    docker_stream
        .zip(stream::iter(containers))
        .for_each_concurrent(2, _conc)
        .await;

    Ok(())
}

async fn _conc(arg: (Docker, &ContainerSummary)) {
    let (docker, container) = arg;
    println!(
        "{:?}",
        docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<InspectContainerOptions>
            )
            .await
            .unwrap()
    )
}

#[tauri::command]
async fn crawl_url(url: &str) -> Result<String, ()> {
    // let (mut browser, mut handler) = Browser::launch(
    //     BrowserConfigBuilder::default()
    //         .request_timeout(Duration::from_secs(5))
    //         .build()
    //         .unwrap(),
    // )
    // .await
    // .unwrap();

    // let handle = task::spawn(async move {
    //     while let Some(h) = handler.next().await {
    //         h.unwrap();
    //     }
    // });

    // let page = browser.new_page(url).await.unwrap();
    // let html = page
    //     .wait_for_navigation()
    //     .await
    //     .unwrap()
    //     .content()
    //     .await
    //     .unwrap();
    // println!("loaded page {:?}", html);

    // browser.close().await.unwrap();
    // handle.await.unwrap();

    let result = get_url_body(url).await;
    if result.is_err() {
        return Ok("".to_string());
    }
    return Ok(result.unwrap());
}

async fn get_url_body(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tauri::command]
async fn create_config(
    state: tauri::State<'_, AppState>,
    form: config::Model,
) -> Result<FlashData, ()> {
    let _ = &state.conn;

    ConfigMutationCore::create_config(&state.conn, form)
        .await
        .expect("could not insert config");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Config succcessfully added".to_owned(),
    };

    Ok(data)
}

#[tauri::command]
async fn update_config(
    state: tauri::State<'_, AppState>,
    id: String,
    form: config::Model,
) -> Result<FlashData, ()> {
    ConfigMutationCore::update_config_by_id(&state.conn, id, form)
        .await
        .expect("could not edit config");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Config succcessfully updated".to_owned(),
    };

    Ok(data)
}

#[tauri::command]
async fn delete_config(state: tauri::State<'_, AppState>, id: String) -> Result<FlashData, ()> {
    ConfigMutationCore::delete_config(&state.conn, id)
        .await
        .expect("could not delete config");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Config succcessfully deleted".to_owned(),
    };

    Ok(data)
}

#[tauri::command]
async fn list_configs(
    state: tauri::State<'_, AppState>,
    params: Params,
) -> Result<Vec<config::Model>, ()> {
    let page = params.page.unwrap_or(1);
    let configs_per_page = params.configs_per_page.unwrap_or(5);

    let (configs, num_pages) =
        ConfigQueryCore::find_configs_in_page(&state.conn, page, configs_per_page)
            .await
            .expect("Cannot find configs in page");

    println!("num_pages: {}", num_pages);

    Ok(configs)
}

#[tauri::command]
async fn get_config(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<config::Model>, ()> {
    let config = ConfigQueryCore::find_config_by_id(&state.conn, id)
        .await
        .expect("Cannot find configs in page");
    Ok(config)
}

// -- chats

#[tauri::command]
async fn create_chat(state: tauri::State<'_, AppState>, form: chat::Model) -> Result<ChatRes, ()> {
    let _ = &state.conn;

    let result = ChatMutationCore::create_chat(&state.conn, form)
        .await
        .expect("could not insert config");
    println!("result: {:?}", result);
    let data = ChatRes {
        kind: "success".to_owned(),
        message: "Created chat successfully!".to_owned(),
        data: Some(result.try_into_model().unwrap()),
    };

    Ok(data)
}

#[tauri::command]
async fn update_chat(
    state: tauri::State<'_, AppState>,
    id: i32,
    form: chat::Model,
) -> Result<FlashData, ()> {
    ChatMutationCore::update_chat_by_id(&state.conn, id, form)
        .await
        .expect("could not edit config");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Chat succcessfully updated".to_owned(),
    };

    Ok(data)
}

#[tauri::command]
async fn delete_chat(state: tauri::State<'_, AppState>, id: i32) -> Result<FlashData, ()> {
    ChatMutationCore::delete_chat(&state.conn, id)
        .await
        .expect("could not delete config");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Config succcessfully deleted".to_owned(),
    };

    Ok(data)
}

#[tauri::command]
async fn list_chats(
    state: tauri::State<'_, AppState>,
    params: Params,
) -> Result<Vec<chat::Model>, ()> {
    let page = params.page.unwrap_or(1);
    let chats_per_page = params.configs_per_page.unwrap_or(5);

    let (configs, num_pages) = ChatQueryCore::find_chats_in_page(&state.conn, page, chats_per_page)
        .await
        .expect("Cannot find chats in page");

    println!("num_pages: {}", num_pages);

    Ok(configs)
}

#[tauri::command]
async fn get_chat(state: tauri::State<'_, AppState>, id: i32) -> Result<Option<chat::Model>, ()> {
    let config = ChatQueryCore::find_chat_by_id(&state.conn, id)
        .await
        .expect("Cannot find chat in page");
    Ok(config)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ChatRes {
    kind: String,
    message: String,
    data: Option<chat::Model>,
}

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    configs_per_page: Option<u64>,
}
