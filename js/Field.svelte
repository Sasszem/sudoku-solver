<script>
    import BigSquare from "./BigSquare.svelte";
    import {globalState as store} from "./stores.js";

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

    button {
        text-transform: capitalize;
        font-size: 20px;
        font-family:Impact, Haettenschweiler, 'Arial Narrow Bold', sans-serif;
        margin: 10px;
        color: darkorchid;
    }
</style>

<div class="field-container">
    {#each [0,1,2] as rowBig}
        {#each [0,1,2] as columnBig}
            <BigSquare {rowBig} {columnBig} bind:this="{bigSquares[3*rowBig+columnBig]}"/>
        {/each}
    {/each}
</div>
<button on:click="{() => {
    store.solveSteps();
    refresh();
}}">solve some squres</button>