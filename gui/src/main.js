window.addEventListener("DOMContentLoaded", () => {

	accountEventListeners();
	orderEventListeners();

	// setInterval( loadSymbols, 5000);
});

function orderEventListeners() {
	document.querySelector("#symbols").addEventListener("click", (e) => {
		e.preventDefault();
		loadSymbols();
	})
	document.querySelector("#order-types").addEventListener("click", (e) => {
		e.preventDefault()
		loadOrderTypes();
	})
	document.querySelector("#calculate-order").addEventListener("click", (e) => {
		e.preventDefault();
		calculateOrder();
	})
}
function accountEventListeners() {
	document.querySelector("#update-account-info").addEventListener("click", (e) => {
		e.preventDefault();
		updateAccountInfo();
	})
}

