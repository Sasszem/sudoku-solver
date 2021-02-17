<script>
	import Loader from "./Loader.svelte";
</script>

<main>
	<h1>Sudoku solver</h1>
	<p>Enter the puzzle down below. When finished, click the <span>solve some squares</span> button!</p>
	<Loader />
	<p>
		Made by <a href="https://sasszem.github.io">László Baráth (2021)</a>
	</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
		margin-top: 0;
		margin-bottom: 0;
	}

	p {
		margin-top: 0;
		margin-bottom: 0;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

	span {
		text-transform: uppercase;
	}
</style>
