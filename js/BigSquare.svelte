<script>
    export let rowBig;
    export let columnBig;

    import Square from "./Square.svelte";
    import {generateIndex} from "./utils.js";

    let squares = [];

    export function refresh() {
        for (const sq of squares) {
            sq.refresh();
        }
    }
</script>

<style>
    .container {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        border: 2px solid black;
    }
</style>

<div class="container">
    {#each [0,1,2] as rowSub}
        {#each [0,1,2] as columnSub}
            <Square index="{generateIndex(rowBig, columnBig, rowSub, columnSub)}" bind:this="{squares[3*rowSub+columnSub]}"/>
        {/each}
    {/each}
</div>
