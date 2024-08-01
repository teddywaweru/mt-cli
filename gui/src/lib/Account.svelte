<script>
	import { invoke } from "@tauri-apps/api/tauri";

	// Trader account Struct is defined at ../../../mt5/src/account.rs
	let account;

	async function update_account_info() {
		account = await invoke("update_account_info", {});
	}
</script>

<div class={$$props.class}>
	<div class="account-info">
		<h2>Account Info</h2>
		<button
			id="update-account-info"
			on:click|preventDefault={update_account_info}
		>
			Update Account Info.
		</button>
		{#if account}
			<table>
				<tr>
					<td class="title"> Account Number </td>
					<td id="account_number">{account["account_number"]}</td>
				</tr>
				<tr>
					<td class="title">Name</td>
					<td id="name">{account["name"]}</td>
				</tr>
				<tr>
					<td class="title">current_time</td>
					<td id="current_time">{account["current_time"]}</td>
				</tr>
				<tr>
					<td class="title">Current Balance</td>
					<td id="current_balance">{account["current_balance"]}</td>
				</tr>
				<tr>
					<td class="title">Current Equity</td>
					<td id="current_equity">{account["current_equity"]}</td>
				</tr>
				<tr>
					<td class="title">Leverage</td>
					<td id="leverage">{account["leverage"]}</td>
				</tr>
				<tr>
					<td class="title">Currency</td>
					<td id="currency">{account["currency"]}</td>
				</tr>
				<tr>
					<td class="title">Free Margin</td>
					<td id="free_margin">{account["free_margin"]}</td>
				</tr>
				<tr>
					<td class="title">Current Profit</td>
					<td id="current_profit">{account["current_profit"]}</td>
				</tr>
			</table>
		{/if}
	</div>
</div>
