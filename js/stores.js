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
                old.wasm.set(index, parseInt(value)||0);
                old.wasm.validate();
                const problems = Array.from({length: 81}, (_, id) => old.wasm.get_error(id));
                return {...old, problems, squares};
            }),
        setWasm: (wasm) => update(
            old => {
                wasm.clear();
                return {...old, wasm};
            }
        ),
        solveSteps: () => update(
            old => {
                old.wasm.solve_step();
                const problems = Array.from({length: 81}, (_, id) => old.wasm.get_error(id));
                const squares = Array.from({length: 81}, (_, id) => old.wasm.get(id));
                return {...old, squares, problems};
            }
        ),
    };
}




export const globalState = createStore();