import {writable} from "svelte/store";

function createStore() {
    let initial = {
        squares: new Array(81).fill(0),
        problems: [],
        wasm: null,
    }

    const {subscribe, update} = writable(initial);
    return {
        subscribe,
        setSquare: (index, value) => update(
            old => {
                const squares = old.squares.map(
                    (oldVal, idx) => (idx==index? parseInt(value) || 0 : oldVal)
                )
                const problems = old.wasm.Board.new(squares).validate_all();
                return {...old, squares, problems};
            }),
        setWasm: (wasm) => update(
            old => {
                return {...old, wasm};
            }
        )
    };
}




export const globalState = createStore();