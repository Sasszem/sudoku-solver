<script>
    export let index;

    import {globalState as store} from "./stores.js";

    let value;

    export function refresh() {
        value = $store.squares[index] || "";
    }

    function commit_value() {
        const setVal = parseInt(value) || (value=="" ? 0 : $store.squares[index]);
        store.setSquare(index, setVal);
        refresh();
    }

    $: error = $store.problems.includes(index);

</script>

<style>
    input {
        font-size: 2rem;
        text-align: center;
        width: 3rem;
        height: 3rem;
        display: inline-block;
        margin: 0;
        padding: 0;
    }
    .error {
        background-color: red;
    }
</style>

<input maxlength="1" bind:value on:change="{commit_value}" class:error>