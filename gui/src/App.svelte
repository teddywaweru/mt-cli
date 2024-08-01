<script>
	// import Greet from "./lib/Greet.svelte";
	import Account from "./lib/Account.svelte";
	import Order from "./lib/Order.svelte";
	import Trade from "./lib/Trade.svelte";


</script>

<main class="container">
	<!-- <h1>Welcome to Tauri!</h1> -->
	<!---->
	<!-- <div class="row"> -->
	<!-- 	<a href="https://vitejs.dev" target="_blank"> -->
	<!-- 		<img src="/vite.svg" class="logo vite" alt="Vite Logo" /> -->
	<!-- 	</a> -->
	<!-- 	<a href="https://tauri.app" target="_blank"> -->
	<!-- 		<img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" /> -->
	<!-- 	</a> -->
	<!-- 	<a href="https://svelte.dev" target="_blank"> -->
	<!-- 		<img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" /> -->
	<!-- 	</a> -->
	<!-- </div> -->
	<!---->
	<!-- <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p> -->

	<div class="row">
		<!-- <Greet /> -->
	</div>
	<div class="row">
		<div class="trader-activities">
			<Account />
			<Trade />
			<Order />
		</div>
	</div>
</main>

<style>
	.trader-activities {
		display: flex;
		flex-direction: row;
	}
	/* .trader-activities > :global(*) { */
	/* 	padding: 0 8%; */
	/* } */
	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}
</style>
