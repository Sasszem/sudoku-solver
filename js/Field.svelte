<script>
    import BigSquare from "./BigSquare.svelte";
</script>

<style>
    .field-container {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        width: fit-content;
        margin: auto;
        border: 2px solid black;
    }
</style>

<div class="field-container">
    {#each [0,1,2] as rowBig}
        {#each [0,1,2] as columnBig}
            <BigSquare {rowBig} {columnBig} />
        {/each}
    {/each}
</div>