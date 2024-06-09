const { invoke } = window.__TAURI__.tauri;

async function message() {
	let messageContent = document.querySelector("#return-message");
	messageContent.textContent = "Suck a nipple";
	messageContent.textContent = await invoke("message_sender", { name: "Reuben" });
}
async function struct() {
	let structContent = document.querySelector("#return-struct");
	structContent.textContent = await invoke("struct_sender", { name: "Steven" });
}


async function greet() {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

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
	// let symbols_list = document.getElementById("symbols");
	// let opt = document.createElement('option');
	// opt.text = "whoosh";
	// opt.value = "stumped";
	// symbols_list.add(opt, null);
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
		// var opt = document.createElement("option");
		// opt.value = "this";
		// opt.text = "this";
		// order_types.add(opt, null);
		// var opt = document.createElement("option");
		// opt.value =response;
		// opt.text =response;
		// order_types.add(opt, null);
	// let order_type_list = document.getElementById("order-types");

	order_types.innerHTML = "";
	response.forEach((order_type) => {
		var opt = document.createElement("option");
		opt.value =order_type;
		opt.text =order_type;

		order_types.add(opt, null);
	});
}
