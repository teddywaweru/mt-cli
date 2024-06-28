const { invoke } = window.__TAURI__.tauri;

async function updateAccountInfo() {
	// Get response
	let response = await invoke("update_account_info", {});
	// Get fields to update

	let account_number = document.getElementById("account_number");
	let name = document.getElementById("name");
	let current_time = document.getElementById("current_time");
	let current_balance = document.getElementById("current_balance");
	let current_equity = document.getElementById("current_equity");
	let leverage = document.getElementById("leverage");
	let currency = document.getElementById("currency");
	let free_margin = document.getElementById("free_margin");
	let current_profit = document.getElementById("current_profit");

	account_number.textContent = response['account_number'];
	name.textContent = response['name'];
	currency.textContent = response['account_number'];
	current_time.textContent = response['current_time'];
	current_balance.textContent = response['current_balance'];
	current_equity.textContent = response['current_equity'];
	leverage.textContent = response['leverage'];
	currency.textContent = response['currency'];
	free_margin.textContent = response['free_margin'];
	current_profit.textContent = response['current_profit'];
}

async function loadSymbols() {
	let response = await invoke("get_symbols");

	let symbols_list = document.getElementById("symbols");

	symbols_list.innerHTML = "";
	response.symbols.forEach((symbol) => {
		var opt = document.createElement("option");
		opt.value = symbol.name;
		opt.text = symbol.name;

		symbols_list.add(opt, null);
	})
}
async function loadOrderTypes() {

	let response = await invoke("get_order_types");

	let order_types = document.getElementById("order-types");

	order_types.innerHTML = "";
	response.forEach((order_type) => {
		var opt = document.createElement("option");
		opt.value = order_type;
		opt.text = order_type;

		order_types.add(opt, null);
	});
}

async function calculateOrder() {
	let account = document.getElementById("account_number").textContent;
	let order_type = document.getElementById("order-types").value;
	let symbol = document.getElementById("symbols").value;
	let risk = document.getElementById("risk").value;
	let volume = document.getElementById("volume").textContent;
	let price = document.getElementById("price").textContent;
	let sl = document.getElementById("sl").textContent;
	let tp = document.getElementById("tp").textContent;
	let account_number = document.getElementById("account_number");



	// let order_request = {
	// 	account: account,
	// 	order_type: order_type,
	// 	symbol: symbol,
	// 	risk: risk,
	// 	volume: volume,
	// 	price: price,
	// 	sl: sl,
	// 	tp: tp
	// };
	// let response = await invoke("calculate_order", { order_request: order_request });
	order_type = "buy";
	let response = await invoke("calculate_order", {symbol: symbol, order_type: order_type, risk: 0.02});
	let calculatedOrderSection = document.getElementById("calculated-order");
	let tester = calculatedOrderSection.querySelector("#symbol");
	tester.textContent = response;
	// response = "oops";



}
