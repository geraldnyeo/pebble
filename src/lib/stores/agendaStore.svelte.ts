import { invoke } from "@tauri-apps/api/core"

export interface Task {
    // basic configurations
    index: number,
    id: string,
    title: string,
    description: string,
    completed: boolean,
    date_string: string,

    // subtask data
    parent: string | null,
    subtasks: string[],
}

export const agendaStore = (() => {
    let tasks = $state<Task[]>([]);

    // return the tasks state
    const getTasks = () => {
        return tasks;
    }

    // read from persistent storage
    const readTasks = async () => {
        tasks = await invoke("read_tasks");
    }

    // sync with persistent storage
    const syncTasks = async () => {
        const res = await invoke("sync_tasks", { "updatedTasks": tasks });
        console.log(res); // for testing
    }

    // add a task
    const addTask = (task: Task) => {
        for (let i = 0; i < tasks.length; i++) {
            if (tasks[i].id == task.id) {
                throw new Error("Failed to add task, task id already exists.");
            }
        }

        tasks.push(task);
    }

    // update a task
    const updateTask = (task: Task) => {
        for (let i = 0; i < tasks.length; i++) {
            if (tasks[i].id == task.id) {
                tasks[i] = task;
            }
        }
    }

    // delete a task
    const deleteTask = (task_id: string) => {
        tasks = tasks.filter((t: Task) => t.id != task_id);
        reindexTasks();
    }

    // reindex tasks
    const reindexTasks = () => {
        const newTasks = [...tasks];

        for (let i = 0; i < newTasks.length; i++) {
            newTasks[i].index = i;
        }

        tasks = newTasks;
    }

    return {
        getTasks,
        readTasks,
        syncTasks,
        addTask,
        updateTask,
        deleteTask,
        reindexTasks,
    }
})();