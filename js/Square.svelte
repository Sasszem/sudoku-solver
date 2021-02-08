<script>
    export let index;

    import {globalState as store} from "./stores.js";

    let value;

    function commit_value() {
        let newVal = parseInt(value) || 0;
        if (newVal > 9 || newVal <=0) {
            value = $store.squares[index] || "";
        }
        store.setSquare(index, value);
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