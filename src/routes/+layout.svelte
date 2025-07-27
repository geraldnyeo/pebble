<script lang="ts">
    import '$lib/styles/fonts.css';

    import { onMount } from 'svelte';

    import { agendaStore } from "$lib/stores/agendaStore.svelte";
    const { readTasks } = agendaStore;

    let { children } = $props();

    onMount(async () => {
        await readTasks();
    })
</script>

<nav id="app-nav">
    <a class="app-nav-item" href="/calendar/day">C</a>
    <a class="app-nav-item" href="/calendar/day">C</a>
    <a class="app-nav-item" href="/calendar/day">C</a>
</nav>

{@render children()}

<style>
    :global(:root) {
        --color-primary: rgb(232, 182, 162);
        --color-primary-dark: rgb(207, 147, 124);
        --color-secondary: rgb(232, 119, 74);
        --color-neutral: rgb(197, 181, 165);
        --color-neutral-background: rgb(247, 240, 233);
        --color-neutral-light: rgb(230, 218, 207);
        --color-neutral-light-background: rgb(252, 249, 247);
        --color-neutral-dark: rgb(69, 63, 58);
        --color-neutral-dark-light: rgb(94, 86, 79);
        --color-neutral-two: rgb(168, 160, 143);
        --color-supporting-green: rgb(179, 200, 177);
        --color-supporting-blue: rgb(126, 169, 180);
    }

    :global(html), :global(body) {
        margin: 0;
        padding: 0;
        width: 100vw;
        height: 100vh;
    }

    #app-nav {
        display: flex;
        flex-direction: row;
        column-gap: 10px;
        position: fixed;
        bottom: 5%;
        left: 50%;
        transform: translate(-50%, -50%);
        z-index: 1;
        padding: 5px 15px;
        border-radius: 25px;
        background-color: rgba(220, 220, 220, 0.8);
    }
</style>