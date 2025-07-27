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

    const weekdays: string[] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    const months: string[] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    const today = new Date();

    let startDate = $state(today);
    let thisYear = $derived(startDate.getFullYear());
    let thisMonth = $derived(startDate.getMonth());
    const monthDates = $derived((() => {
        const lastDay = new Date(thisYear, thisMonth + 1, 0); // gets last day of previous month; in this case this month's last day
        const numOfDays = lastDay.getDate();

        const dates: Date[] = [];
        
        const preFiller = new Date(thisYear, thisMonth, 0).getDay() + 1;
        for (let i = 0; i < preFiller; i++) { // add filler before first day
            const date = new Date(thisYear, thisMonth, 1);
            date.setDate(date.getDate() - preFiller + i);
            dates.push(date);
        }

        for (let i = 0; i < numOfDays; i++) { // add actual month days
            const date = new Date(thisYear, thisMonth, 1);
            date.setDate(date.getDate() + i);
            dates.push(date);
        }

        const postFiller = 7 - (((dates.length - 1) % 7) + 1);
        for (let i = 0; i < postFiller; i++) { // add filler after last day
            const date = new Date(thisYear, thisMonth, 1);
            date.setDate(date.getDate() + numOfDays + i);
            dates.push(date);
        }

        return dates;
    })());

    let selectedDateString = $state((new Date()).toDateString());

    const toggleTaskAddModal = () => {
        taskAddModal = !taskAddModal;
    }

    const handleMenuAdd = (date: Date) => {
        selectedDateString = date.toDateString();
        toggleTaskAddModal();
    }

    const handlePrevMonth = () => {
        const newDate = new Date(startDate);
        newDate.setMonth(newDate.getMonth() - 1);
        startDate = newDate;
    }

    const handleNextMonth = () => {
        const newDate = new Date(startDate);
        newDate.setMonth(newDate.getMonth() + 1);
        startDate = newDate;
    }

    $effect(() => {
        ctrlSaveTasks();
    })
</script>

<div class="content">
    <div class="calendar-month-title">
        <h1>{months[thisMonth]} {thisYear}</h1>
    </div>
    <div class="calendar-month-header">
        {#each weekdays as day}
            <div class="calendar-month-header-day">
                {day}
            </div>
        {/each}
    </div>
    <div class="calendar-month">
        {#each monthDates as date}
            {#if date.getMonth() !== startDate.getMonth()}
                <div class="calendar-month-cell-empty">
                    <p class="calendar-month-cell-datenumber">{date.getDate()}</p>
                    <div class="calendar-month-items-wrapper"> 
                        <TaskItems {tasks} {date}/>
                    </div>
                </div>
            {:else}
                <div class="calendar-month-cell" class:today={date.toLocaleDateString() === today.toLocaleDateString()}>
                    <p class="calendar-month-cell-datenumber">{date.getDate()}</p>
                    <div class="calendar-month-items-wrapper"> 
                        <TaskItems {tasks} {date}/>
                    </div>

                    <ContextMenu>
                        <ContextMenuOption onclick={() => {handleMenuAdd(date)}}>
                            <!-- TODO: Add Icons as well -->
                            <span>Add a task</span>
                        </ContextMenuOption>
                    </ContextMenu>
                </div>
            {/if}
        {/each}
    </div>
    <div class="calendar-nav">
        <button onclick={handlePrevMonth}>&lt;</button>
        <button onclick={handleNextMonth}>&gt;</button>
    </div>
</div>

{#if taskAddModal}
    <TaskAdd {toggleTaskAddModal} dateString={selectedDateString}/>
{/if}

<style>
    .content {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        width: 100%;
        height: 100%;
        padding: 30px 60px 60px 60px;
        box-sizing: border-box;
    }

    .calendar-month-title h1 {
        padding: 10px;
        border-radius: 10px;
        margin: 10px 0 10px 0;
        color: var(--color-neutral-dark);
        background-color: var(--color-neutral-background);
        box-sizing: border-box;
    }

    .calendar-month-header {
        display: flex;
        flex-direction: row;
        padding: 10px 0 10px 0;
        box-sizing: border-box;
    }

    .calendar-month-header-day {
        flex-grow: 1;
        color: var(--color-neutral-dark);
        text-align: center;
    }

    .calendar-month {
        flex-grow: 1;
        display: grid;
        grid-template-columns: repeat(7, 1fr);
        grid-auto-rows: 1fr;
        min-width: 0;
        min-height: 0;
    }

    .calendar-month-cell {
        display: flex;
        flex-direction: column;
        min-width: 0;
        min-height: 0;
        border: 1px solid var(--color-neutral-background);
    }

    .calendar-month-cell.today {
        border: 2px solid var(--color-neutral-light);
    }

    .calendar-month-items-wrapper {
        flex-grow: 1;
        overflow-x: hidden;
        overflow-y: auto;
    }

    .calendar-month-cell-empty {
        display: flex;
        flex-direction: column;
        min-width: 0;
        min-height: 0;
        border: 1px solid var(--color-neutral-background);
        background-color: var(--color-neutral-light-background);
    }

    .calendar-month-cell-datenumber {
        align-self: flex-end;
        padding: 5px;
        margin: 0;
        color: var(--color-neutral-dark);
    }

    .calendar-nav {
        position: absolute;
        bottom: 30px;
        right: 60px;
    }
</style>