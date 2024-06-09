window.addEventListener("DOMContentLoaded", () => {

	addPseudoActions();
	addAccountEventListeners();

	// setInterval( loadSymbols, 5000);
});

function addPseudoActions() {
	document.querySelector("#greet-form").addEventListener("submit", (e) => {
		e.preventDefault();
		greet();
	});

	document.querySelector("#message").addEventListener("click", (e) => {
		e.preventDefault();
		message();
	})

	document.querySelector("#struct").addEventListener("click", (e) => {
		e.preventDefault();
		struct();
	})
	document.querySelector("#struct").addEventListener("click", (e) => {
		e.preventDefault();
		struct();
	})
	document.querySelector("#order-types").addEventListener("click", (e) => {
		e.preventDefault()
		loadOrderTypes();
	})
	document.querySelector("#refresh-symbols").addEventListener("click", (e) => {
		e.preventDefault();
		loadSymbols();
	})
	document.querySelector("#calculate-order").addEventListener("click", (e) => {
		e.preventDefault();
		calculateOrder();
	})
}

function addAccountEventListeners() {
	document.querySelector("#update-account-info").addEventListener("click", (e) => {
		e.preventDefault();
		updateAccountInfo();
	})
}
