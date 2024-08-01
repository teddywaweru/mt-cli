<script>
	import { invoke } from "@tauri-apps/api/tauri";
	let order_types;
	let symbols;

	async function load_order_types() {
		order_types = await invoke("get_order_types", {});
	}
	async function load_symbols() {
	symbols = await invoke("load_symbols")
	}
</script>

<div>
	<h2>Orders</h2>
	Execute a new Order here
	<div id="calculate">
		<div class="execution">
			<div>
				<label for="order_types">Order Types</label>
				<select
					id="order-types"
					bind:value={order_types}
					on:click|preventDefault={load_order_types}
				>
					{#if order_types}
						{#each order_types as order_type}
							<option value={order_type}>{order_type}</option>
						{/each}
					{/if}
				</select>
			</div>
			<input id="risk" placeholder="Risk(Default:0.02)" />
			<div class="symbols">
				<label for="symbols">Symbols</label>
				<select bind:value={symbols} id="symbols">
				{#if symbols}
					{#each symbols as symbol }
						<option value={symbol}>{symbol}</option>
					{/each}
					
				{/if}
				</select>
			</div>
			<div>
				<button id="advanced-options">Show Advanced Options</button>
			</div>
			<div class="actions">
				<button id="calculate-order">Calculate Order</button>
				<button id="generate-order">Execute Instant Order</button>
			</div>
		</div>
		<button id="generate-order">Execute Calculated Order</button>
		<div id="calculated-order">
			<p id="symbol"></p>
			<p id="volume"></p>
			<p id="price"></p>
			<p id="sl"></p>
			<p id="tp"></p>
			<p id="order-type"></p>
			<p id="order-type-filling"></p>
			<p id="order-type-time"></p>
			<p id="comment"></p>
		</div>
	</div>
</div>

<style>
	div {
		padding: 10px;
	}
</style>
