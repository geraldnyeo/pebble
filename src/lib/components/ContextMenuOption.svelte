<script lang="ts">
    const { children, onclick } = $props();

    const handleClick = (e: any) => {
        onclick();
        const menuRoot = e.currentTarget.parentNode;
        menuRoot.dispatchEvent(
            new CustomEvent("close_context_menu")
        );
    }
</script>

<button class="menu-option" onclick={e => handleClick(e)}>
    {@render children()}
</button>

<style>
    .menu-option {
        padding: 5px;
    }
</style>