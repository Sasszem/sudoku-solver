<script>
    export let rowBig;
    export let columnBig;

    import Square from "./Square.svelte";
    import {generateIndex} from "./utils.js";
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
            <Square index="{generateIndex(rowBig, columnBig, rowSub, columnSub)}" />
        {/each}
    {/each}
</div>
