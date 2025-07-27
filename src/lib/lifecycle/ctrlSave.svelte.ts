import { agendaStore } from "$lib/stores/agendaStore.svelte";
const { syncTasks } = agendaStore;

export const ctrlSaveTasks = async () => {

    const handleSave = async () => {
        await syncTasks();
    }

    const handleKeyDown = async (event: any) => {
        if (event.ctrlKey && event.which === 83) {
            event.preventDefault();
            await handleSave();
        }
    }

    document.addEventListener('keydown', handleKeyDown, true);

    return {
        async destroy() {
            await handleSave();
            document.removeEventListener('keydown', handleKeyDown, true);
        }
    }
}