<script lang="ts">
    import type { Task } from "$lib/stores/agendaStore.svelte";
    import { agendaStore } from "$lib/stores/agendaStore.svelte";
    const { updateTask, deleteTask, reindexTasks } = agendaStore;

    const { tasks, date } = $props();

    const dateString = $derived(date.toDateString());

    const filterTasks = (tasks: Task[], dateString: string) => {        
        const filteredTasks = tasks.filter((t: Task) => t.date_string === dateString);
        return filteredTasks;
    }
    const filteredTasks = $derived(filterTasks(tasks, dateString));

    let targetTaskIndex: number | boolean = $state(false);

    const toggleTaskCompleted = (task: Task) => {
        const updatedTask: Task = {
            ...task,
            completed: !task.completed
        }
        updateTask(updatedTask);
    }

    const handleDelete = (task: Task) => {
        deleteTask(task.id);
    }

    // get data of task
    // in case user clicks child elements
    const getTaskDataset = (node: any) => {
        if (!node.dataset.index) {
            return getTaskDataset(node.parentNode);
        } else {
            return { ...node.dataset } as Task;
        }
    }

    const handleDragStart = (e: DragEvent) => {
        const sourceTaskData = getTaskDataset(e.target);
        e.dataTransfer?.setData("sourceIndex", sourceTaskData.index.toString());
    }

    const handleDragEnd = (e: DragEvent) => {
        targetTaskIndex = false;
    }

    const handleDragOver = (e: DragEvent, index: number) => {
        e.preventDefault();
        // @ts-ignore
        const rect = e.currentTarget.getBoundingClientRect();
        const offset = e.clientY - rect.top;
        const midpoint = rect.height / 2;

        if (offset < midpoint) {
            targetTaskIndex = index;
        } else {
            targetTaskIndex = index + 1;
        }
    }

    const handleDropZoneDragOver = (e: DragEvent) => {
        e.preventDefault();
    }

    const handleContainerDragOver = (e: DragEvent) => {
        e.preventDefault();
        if (filteredTasks.length === 0) {
            targetTaskIndex = getPlaceholderTask().index;
        }
    }

    const handleDragLeave = (e: DragEvent) => {
        // @ts-ignore
        if (!e.currentTarget.contains(e.relatedTarget)) {
            targetTaskIndex = false;
        }
    }

    const handleDrop = (e: DragEvent) => {
        e.preventDefault();

        // @ts-ignore
        const sourceIndex = parseInt(e.dataTransfer.getData("sourceIndex"));

        // @ts-ignore
        if (sourceIndex < targetTaskIndex) { // @ts-ignore
            targetTaskIndex = targetTaskIndex - 1;
        }

        const [t] = tasks.splice(sourceIndex, 1);
        // @ts-ignore
        t.date_string = e.currentTarget.dataset.date_string;
        tasks.splice(targetTaskIndex, 0, t);
        reindexTasks();
        targetTaskIndex = false;
    }

    // get the new index and date_string of the task being moved if filteredtasks is empty
    // or the last task
    const getPlaceholderTask = () => {
        if (filteredTasks.length > 0) {
            const lastTask = filteredTasks[filteredTasks.length - 1];
            return {
                index: lastTask.index + 1,
            }
        } else {
            let index = tasks.findIndex((t: Task) => Date.parse(t.date_string) > date); // assumes tasks are sorted by date, which they should be
            if (index === -1) { index = tasks.length };
            return {
                index,
            }
        }
    }
</script>

<ul 
    class="tasks"
    data-date_string={dateString}
    ondragover={handleContainerDragOver}
    ondragleave={handleDragLeave}
    ondrop={e => {if (filteredTasks.length === 0) {handleDrop(e)}}}
>
    {#each filteredTasks as task (task.id)}
        {#if targetTaskIndex === task.index}
            <li class="task-dropzone"
                data-index={task.index}
                data-date_string={task.date_string}
                ondragover={handleDropZoneDragOver}
                ondrop={handleDrop}
            >
                <!-- TODO: Icons -->
                <span>move task here</span>
            </li>
        {/if}
        <li 
            class="task"
            class:over={targetTaskIndex === task.index}
            draggable="true"
            data-index={task.index}
            data-date_string={task.date_string}
            ondragstart={handleDragStart}
            ondragend={handleDragEnd}
            ondragover={(e) => {handleDragOver(e, task.index)}}
            ondrop={handleDrop}
        >
            <button class="task-completed" onclick={() => toggleTaskCompleted(task)}>
                <!-- TODO: Replace with icons -->
                {#if task.completed}
                    Y
                {:else}
                    N
                {/if}
            </button>
            <span class="task-title">{task.title}</span>
            <button class="task-delete" onclick={() => handleDelete(task)}>
                <!-- TODO: Replace with icons -->
                D
            </button>
        </li>
    {/each}
    {#if targetTaskIndex === getPlaceholderTask().index}
        <!-- TODO: Fix this to use the correct task id: not the last task, but the last task for that date -->
        <li class="task-dropzone"
            data-date_string={dateString}
            ondragover={handleDropZoneDragOver} 
            ondrop={handleDrop}
        >
            <!-- TODO: Icons -->
            <span>move task here</span>
        </li>
    {/if}
</ul>

<style>
    .tasks {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        row-gap: 2px;
        width: 100%;
        height: 100%;
        padding: 0;
        margin: 0;
    }

    .task {
        display: flex;
        flex-direction: row;
        column-gap: 10px;
        padding: 10px;
        border: 1px solid var(--color-neutral-light);
        border-radius: 10px;
        list-style-type: none;
        color: var(--color-neutral-dark);
        background-color: var(--color-neutral-light-background);
        box-sizing: border-box;
    }

    .over {
        border: 1px solid var(--color-neutral);
        background-color: var(--color-neutral-background);
    }

    .task-title {
        flex-grow: 1;
        overflow-x: hidden;
    }

    .task-dropzone {
        width: 100%;
        padding: 10px;
        border: 1px dashed var(--color-neutral-light);
        border-radius: 10px;
        list-style-type: none;
        color: var(--color-neutral-dark);
        background-color: var(--color-neutral-background);
        text-align: center;
        box-sizing: border-box;
    }
</style>