<script lang="ts">
    import { rightClick } from "$lib/lifecycle/rightClick.svelte";
    import { clickOutside } from "$lib/lifecycle/clickOutside.svelte";

    const { children } = $props();

    let show = $state(false);

    const openContextMenu = (e: any) => {
        const { clientX, clientY } = e.detail;
        const menuElement = e.target;

        // TODO: Checking for whether menu is at the edges
        menuElement.style.left = `${clientX}px`;
        menuElement.style.top = `${clientY}px`;

        show = true;
    }
</script>

<div 
    class="menu"
    class:show={show}
    use:rightClick
    onright_click={openContextMenu}
    onfalse_right_click={() => {if (show) {show = false}}}
    use:clickOutside
    onclick_outside={() => {if (show) {show = false}}}
    onclose_context_menu={() => {if (show) {show = false}}}
>
    {@render children()}
</div>

<style>
    .menu {
        display: none;
        flex-direction: column;
        position: fixed;
        box-shadow: 2px 2px 5px 0px rgb(220, 220, 220);
    }

    .menu.show {
        display: flex;
    }
</style>