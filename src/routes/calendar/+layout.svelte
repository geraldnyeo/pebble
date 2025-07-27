<script>
    let { children } = $props();
</script>

<nav id="calendar-nav">
    <a class="calendar-nav-item" href="/calendar/day">D</a>
    <a class="calendar-nav-item" href="/calendar/week">W</a>
    <a class="calendar-nav-item" href="/calendar/month">M</a>
</nav>

{@render children()}

<style>
    #calendar-nav {
        display: flex;
        flex-direction: column;
        row-gap: 10px;
        position: fixed;
        top: 50%;
        left: 5%;
        transform: translate(-50%, -50%);
        background-color: rgba(220, 220, 220, 0.8);
        padding: 15px 5px;
        border-radius: 25px;
    }
</style>