<script lang="ts">
    import { v4 as uuidv4 } from 'uuid';
    
    import { clickOutside } from "$lib/lifecycle/clickOutside.svelte";

    import type { Task } from "$lib/stores/agendaStore.svelte";
    import { agendaStore } from "$lib/stores/agendaStore.svelte";
    const { getTasks, addTask } = agendaStore;

    const { toggleTaskAddModal, dateString } = $props();

    const tasks = $derived(getTasks());
    
    let expanded = $state(false);

    let title = $state("");
    let description = $state("");
    let completed = $state(false);

    let parent = $state(null);
    let subtasks = $state([]);

    const handleSubmit = () => {
        const task: Task = {
            id: uuidv4(),
            index: tasks.length,
            title: title,
            description: description,
            completed: completed,
            date_string: dateString,
            parent: parent,
            subtasks: subtasks,
        }
        addTask(task);
        toggleTaskAddModal();
    }

</script>

<div class="task-add-root">
    {#if !expanded}
        <div class="task-add-modal" use:clickOutside onclick_outside={toggleTaskAddModal}>
            <button class="modal-expand-toggle" onclick={() => {expanded = !expanded}}>
                <!-- TODO: Icons -->
                E
            </button>
            <input bind:value={title} placeholder="Add a task"/>
            <hr>
            <div class="task-add-modal-options">
                <button onclick={handleSubmit}>+</button>
            </div>
        </div>
    {:else}
        <div class="task-add-modal-expanded">
            <button class="modal-expand-toggle" onclick={() => {expanded = !expanded}}>
                <!-- TODO: Icons -->
                E
            </button>
            <input bind:value={title} placeholder="title" />
            <input bind:checked={completed} type="checkbox">
            <textarea bind:value={description} placeholder="description"></textarea>
            <!-- TODO: Date Selector -->
            <button onclick={handleSubmit}>+</button>
        </div>
    {/if}
</div>

<style>
    .task-add-root {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 99;
        background-color: rgba(252, 249, 247, 0.8);
    }

    .task-add-modal {
        display: flex;
        flex-direction: column;
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        z-index: 99;
        width: 40%;
        padding: 10px;
        border-radius: 10px;
        background-color: var(--color-neutral-light);
    }

    .task-add-modal input {
        width: 90%;
        border: none;
        background: none;
        color: var(--color-neutral-dark);
    }
    
    .task-add-modal input::placeholder {
        color: var(--color-neutral-dark-light);
    }

    .task-add-modal input:focus {
        outline: none;
    }

    .task-add-modal hr {
        width: 100%;
        height: 0.1px;
        border: none;
        background-color: var(--color-neutral-dark);
    }

    .task-add-modal-options {
        display: flex;
        flex-direction: row;
    }

    .task-add-modal-expanded {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        padding: 60px;
        background-color: var(--color-neutral-light-background);
        box-sizing: border-box;
    }

    .task-add-modal-expanded input {
        border: none;
        background: none;
        color: var(--color-neutral-dark);
        font-size: 40px;
        /* TODO: DEBUG THIS! */
        font-family: Gelasio;
    }
    
    .task-add-modal-expanded input::placeholder {
        color: var(--color-neutral-dark-light);
    }

    .task-add-modal-expanded input:focus {
        outline: none;
    }

    .modal-expand-toggle {
        position: absolute;
        top: 10px;
        right: 10px;
    }
</style>