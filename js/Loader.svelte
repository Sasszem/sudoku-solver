<script>
    import Field from "./Field.svelte";
    import {globalState as store} from "./stores.js";
</script>

<style>
    .text {
        font-size: 2rem;
        text-transform: uppercase;
    }
    .spinner {
        display: inline-block;
        width: 5rem;
        height: 5rem;
        border-radius: 5rem;
        border: 3px double;
        border-color: black transparent black transparent;
        animation: rotate 1s linear infinite;
    }

    @keyframes rotate {
        0% {
            transform: rotate(0dex);
        }
        100% {
            transform: rotate(180deg);
        }
    }
</style>

{#await import("../pkg/index.js").then(store.setWasm)}
    {$store.wasm}
    <div class="loader-container">
        <p class="text">Loading, please wait...</p>
        <div class="spinner"></div>
    </div>
{:then val}
    <Field />
{/await}
