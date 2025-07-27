// Imports
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tauri::{Manager, AppHandle, Wry};
use tauri_plugin_store::{StoreExt};

// Task struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    // basic configurations
    pub id: String,
    pub index: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub date_string: String,

    // subtask data
    pub parent: Option<String>,
    pub subtasks: Vec<String>
}

// Configuration
const TASK_STORE_FILE_NAME: &str = "app_tasks.dat";
const ALL_TASK_IDS_KEY: &str = "task_ids";
const TASK_PREFIX: &str = "task_";

// Get path to store file
pub fn get_store_path(app_handle: &AppHandle<Wry>, store_file: &str) -> Result<PathBuf, String> {
    // Get the path directory as a PathBuf
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    // Create directory if it doesn't already exist
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    // Combine app data directory with filename
    Ok(app_data_dir.join(store_file))
}

// Read Tasks
#[tauri::command]
pub async fn read_tasks(app_handle: AppHandle<Wry>) -> Result<Vec<Task>, String> {
    // Get instance of store
    let store_path = get_store_path(&app_handle, TASK_STORE_FILE_NAME)?;
    let store = app_handle.store(store_path).map_err(|e| e.to_string())?;

    // Get list of task_ids from store
    let task_ids: Vec<String> = if let Some(value) = store.get(ALL_TASK_IDS_KEY) {
        serde_json::from_value(value)
            .map_err(|e| format!("Failed to deserialize tasks: {}", e))?
    } else {
        Vec::new()
    };

    // Get list of tasks from store
    let mut tasks: Vec<Task> = Vec::with_capacity(task_ids.len());
    for id in task_ids {
        let task_key = format!("{}{}", TASK_PREFIX, id);
        
        if let Some(value) = store.get(&task_key) {
            match serde_json::from_value::<Task>(value) {
                Ok(task) => tasks.push(task),
                Err(e) => eprintln!("Warning: Failed to deserialize task with key {}: {}", task_key, e),
            }
        } else {
            eprintln!("Warning: Task data missing for ID: {}", id);
        }
    }

    // Return
    Ok(tasks)
}

// Write Tasks
#[tauri::command]
pub async fn sync_tasks(app_handle: AppHandle<Wry>, updated_tasks: Vec<Task>) -> Result<(), String> {
    // Get instance of store
    let store_path = get_store_path(&app_handle, TASK_STORE_FILE_NAME)?;
    let store = app_handle.store(store_path).map_err(|e| e.to_string())?;

    // Get (outdated) list of task_ids from store
    let outdated_task_ids: Vec<String> = if let Some(value) = store.get(ALL_TASK_IDS_KEY) {
        serde_json::from_value(value)
            .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };

    // Get (outdated) hashmap of task_id -> task from store
    let mut outdated_tasks: HashMap<String, Task> = HashMap::with_capacity(outdated_task_ids.len());
    for id in &outdated_task_ids {
        let task_key = format!("{}{}", TASK_PREFIX, id);
        
        if let Some(value) = store.get(&task_key) {
            match serde_json::from_value::<Task>(value) {
                Ok(task) => {
                    outdated_tasks.insert(task.id.clone(), task);
                },
                Err(e) => eprintln!("Warning: Failed to deserialize task with key {}: {}", task_key, e),
            }
        }
    }

    // Get (updated) list of task_ids
    let mut sorted_updated_tasks = updated_tasks.clone();
    sorted_updated_tasks.sort_by(|t1, t2| t1.index.cmp(&t2.index));
    let updated_task_ids:Vec<String> = sorted_updated_tasks.iter().map(|task| task.id.clone()).collect();

    // 1: Process additions and updates
    for task in updated_tasks {
        let task_key = format!("{}{}", TASK_PREFIX, task.id);
        let updated_task = serde_json::to_value(&task)
            .map_err(|e| format!("Failed to serialize task {}: {}", task.id, e))?;

        if let Some(outdated_task) = outdated_tasks.get(&task.id) { // if task already exists
            // check if any changes; update if necessary
            if *outdated_task != task {
                store.set(&task_key, updated_task);
                println!("Updated task: {}", task.id);
            }
        } else { // if task does not exist yet; add
            store.set(&task_key, updated_task);
            println!("Added task: {}", task.id);
        }
    }

    // 2: Process deletions
    let outdated_task_ids_set: HashSet<String> = outdated_task_ids.into_iter().collect();
    let updated_task_ids_set: HashSet<String> = updated_task_ids.iter().map(|id| id.clone()).collect();
    for id in outdated_task_ids_set.difference(&updated_task_ids_set) {
        let task_key = format!("{}{}", TASK_PREFIX, id);
        store.delete(&task_key);
        println!("Deleted task: {}", id);
    }

    // Update task_id master list
    let updated_task_ids_serde: Value = Value::Array(updated_task_ids.iter().map(|id| Value::String(id.clone())).collect());
    store.set(ALL_TASK_IDS_KEY, updated_task_ids_serde);

    // Save all updates
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}


// TODO
// Goals