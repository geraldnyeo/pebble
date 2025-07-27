<script lang="ts">  
    import { ctrlSaveTasks } from "$lib/lifecycle/ctrlSave.svelte";

    import TaskItems from "$lib/components/TaskItems.svelte";
    import TaskAdd from "$lib/components/TaskAdd.svelte";

    import type { Task } from "$lib/stores/agendaStore.svelte";
    import { agendaStore } from "$lib/stores/agendaStore.svelte";
    const { getTasks } = agendaStore;
    
    let tasks = $derived(getTasks());
    let taskAddModal = $state(false);

    let currentDate = $state(new Date());
    let dateString = $derived(currentDate.toDateString());

    const filterTasks = (tasks: Task[], dateString: string) => {        
        const filteredTasks = tasks.filter((t: Task) => t.date_string === dateString);
        return filteredTasks;
    }
    const filteredTasks = $derived(filterTasks(tasks, dateString));

    const weekdays = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
    const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    const toggleTaskAddModal = () => {
        taskAddModal = !taskAddModal;
    }

    const handlePrevDay = () => {
        const newDate = new Date(currentDate);
        newDate.setDate(newDate.getDate() - 1);
        currentDate = newDate;
    }

    const handleNextDay = () => {
        const newDate = new Date(currentDate);
        newDate.setDate(newDate.getDate() + 1);
        currentDate = newDate;
    }

    $effect(() => {
        ctrlSaveTasks();
    })
</script>

<div class="content">
    <h1 class="task-header">
        {weekdays[currentDate.getDay()]}, {currentDate.getDate()} {months[currentDate.getMonth()]}
    </h1>

    <div class="task-items-wrapper">
        <TaskItems {tasks} date={currentDate} />
    </div>

    <div class="task-add-prompt">
        <!-- TODO: Edit so it filters based on completion status -->
        <!-- instead of using task length -->
        {#if filteredTasks.length == 0} 
            <div class="task-add-prompt-nothing">
                <p>No tasks left today! Hooray!</p>
                <p>Enjoy your day, or add a task...</p>
            </div>
        {/if}

        <button class="task-add-button" onclick={toggleTaskAddModal}>+</button>
        <p class="task-add-button-description">Add a task</p>
    </div>

    <div class="calendar-nav">
        <button onclick={handlePrevDay}>&lt;</button>
        <button onclick={handleNextDay}>&gt;</button>
    </div>
</div>

{#if taskAddModal}
    <TaskAdd {toggleTaskAddModal} {dateString} />
{/if}

<style>
    .content {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        row-gap: 20px;
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 40%;
        height: 70%;
    }

    .task-header {
        margin: 20px 0 20px 0;
        box-sizing: border-box;
        color: var(--color-neutral-dark);
    }

    .task-items-wrapper {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        overflow-y: auto;
    }

    .task-add-prompt {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        row-gap: 2px;
        width: 100%;
        padding: 20px;
        box-sizing: border-box;
        border: 1px dashed var(--color-neutral);
        border-radius: 20px;
        text-align: center;
        color: var(--color-neutral-dark);
    }

    .task-add-button {
        width: 30px;
        height: 30px;
        border: none;
        border-radius: 50%;
        text-align: center;
        color: var(--color-neutral-dark);
        background-color: var(--color-primary);
    }

    .task-add-button:hover {
        background-color: var(--color-primary-dark);
    }

    .task-add-button-description {
        color: rgb(130, 130, 130);
        font-size: 12px;
        line-height: 0;
        color: var(--color-neutral-dark);
    }

    .calendar-nav {
        position: absolute;
        bottom: 30px;
        left: 50%;
        transform: translateX(-50%);
    }

    .calendar-nav button {
        width: 25px;
        height: 25px;
        border: none;
        border-radius: 50%;
        text-align: center;
        color: var(--color-neutral-dark);
        background-color: var(--color-neutral-light);
    }
</style>

