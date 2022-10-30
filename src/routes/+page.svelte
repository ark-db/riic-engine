<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    type FileData = {
        name: string,
        modified: number,
        created: number
    }

    let saves = fetchSaves();

    async function fetchSaves() {
        return await invoke("fetch_saves") as FileData[];
    }
</script>



<h1>RIIC Engine</h1>
<p>v0.1.0</p>

{#await saves}
    <p>Loading...</p>
{:then saveList}
    {#if saveList.length > 0}
        {#each saveList as save}
            <p>{save}</p>
        {/each}
    {:else}
        <p>No saves found!</p>
    {/if}
{:catch err}
    <p>{err.message}</p>
{/await}



<style>
    h1, p {
        text-align: center;
        color: var(--light);
    }
    h1 {
        font-size: 4em;
    }
</style>