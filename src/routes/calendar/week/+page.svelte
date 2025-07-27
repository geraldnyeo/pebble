<script lang="ts">
    import { ctrlSaveTasks } from "$lib/lifecycle/ctrlSave.svelte";

    import TaskItems from "$lib/components/TaskItems.svelte";
    import TaskAdd from "$lib/components/TaskAdd.svelte";
    import ContextMenu from "$lib/components/ContextMenu.svelte";
    import ContextMenuOption from "$lib/components/ContextMenuOption.svelte";
    
    import { agendaStore } from "$lib/stores/agendaStore.svelte";
    const { getTasks } = agendaStore;
    
    const tasks = $derived(getTasks());

    let taskAddModal = $state(false);

    const today = new Date();

    let startDate = $state(today);
    let weekDates = $derived((() => {
        const dates: Date[] = [];
        for (let i = 0; i < 7; i++) {
            const date = new Date(startDate);
            date.setDate(date.getDate() + i);
            dates.push(date);
        }
        return dates;
    })())
    
    let selectedDateString = $derived(startDate.toDateString());

    const toggleTaskAddModal = () => {
        taskAddModal = !taskAddModal;
    }

    const handleMenuAdd = (date: Date) => {
        selectedDateString = date.toDateString();
        toggleTaskAddModal();
    }

    const handlePrevDay = () => {
        const newDate = new Date(startDate);
        newDate.setDate(newDate.getDate() - 1);
        startDate = newDate;
    }

    const handleNextDay = () => {
        const newDate = new Date(startDate);
        newDate.setDate(newDate.getDate() + 1);
        startDate = newDate;
    }

    const handlePrevWeek = () => {
        const newDate = new Date(startDate);
        newDate.setDate(newDate.getDate() - 7);
        startDate = newDate;
    }

    const handleNextWeek = () => {
        const newDate = new Date(startDate);
        newDate.setDate(newDate.getDate() + 7);
        startDate = newDate;
    }

    $effect(() => {
        ctrlSaveTasks();
    })
</script>

<div class="content">
    <div class="calendar-week">
        {#each weekDates as date, index}
            {#if index != 0}
                <div class="calendar-week-column-separator"></div>
            {/if}

            <div class="calendar-week-column">
                <div class="calendar-week-column-header" class:today={date.toLocaleDateString() === today.toLocaleDateString()}>
                    <h2 class="calendar-week-column-day">{date.toLocaleDateString("en-UK", { weekday: "short" }).toUpperCase()}</h2>
                    <p class="calendar-week-column-datenumber">{date.getDate()}</p>
                </div>
                <br>

                <TaskItems {tasks} {date} />
                
                <ContextMenu>
                    <ContextMenuOption onclick={() => {handleMenuAdd(date)}}>
                        <!-- TODO: Add Icons as well -->
                        <span>Add a task</span>
                    </ContextMenuOption>
                </ContextMenu>
            </div>
        {/each}
    </div>
    <div class="calendar-nav">
        <button onclick={handlePrevWeek}>&lt;&lt;</button>
        <button onclick={handlePrevDay}>&lt;</button>
        <button onclick={handleNextDay}>&gt;</button>
        <button onclick={handleNextWeek}>&gt;&gt;</button>
    </div>
</div>

{#if taskAddModal}
    <TaskAdd {toggleTaskAddModal} dateString={selectedDateString}/>
{/if}

<style>
    .content {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 100%;
        padding: 30px;
        box-sizing: border-box;
    }

    .calendar-week {
        flex-grow: 1;
        display: flex;
        flex-direction: row;
        column-gap: 10px;
    }

    .calendar-week-column {
        flex: 1 1 0;
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 0;
    }

    .calendar-week-column-separator {
        width: 0;
        height: 100%;
        border-left: 1px dashed var(--color-neutral-two);
    }

    .calendar-week-column-header {
        flex: 1 1 0;
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        padding: 5px;
        border: 1px dashed var(--color-neutral);
        border-radius: 20px;
        background-color: var(--color-neutral-light-background);
        box-sizing: border-box;
    }

    .calendar-week-column-header.today {
        background-color: var(--color-neutral-background);
    }

    .calendar-week-column-day {
        padding: 5px;
        margin: 0;
        color: var(--color-neutral-dark);
    }

    .calendar-week-column-datenumber {
        padding: 5px;
        margin: 0;
        color: var(--color-neutral-dark);
    }

    .calendar-nav {
        position: absolute;
        bottom: 60px;
        right: 60px;
    }
</style>