<script>
    import BigSquare from "./BigSquare.svelte";

    let bigSquares = [];

    function refresh() {
        for (const sq of bigSquares) {
            sq.refresh();
        }
    }
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
            <BigSquare {rowBig} {columnBig} bind:this="{bigSquares[3*rowBig+columnBig]}"/>
        {/each}
    {/each}
</div>