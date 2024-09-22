const form = document.querySelector(".search-form");

const wrapperName = ".v";

function getListItems() {
	return document.querySelectorAll(`${wrapperName}>div:has(input:not(:checked))`)
}

let id = "abc";

function createAirportElement(data) {
	const item = document.createElement("div");
	const label = document.createElement("label");
	label.for = data.iata_code;
	label.textContent = data.name;
	const input = document.createElement("input");
	input.type = "radio"
	input.name = "fra";
	input.id = data.iata_code;
	item.append(label, input);
	return item;
}

form.addEventListener("submit", async e => {
	e.preventDefault();
	const value = e.target.search.value;
	const newList = (await search(value)).map(createAirportElement);
	console.log(newList);
	const list = getListItems();
	const wrapper = document.querySelector(wrapperName);
	for (const item of list) {
		console.log(item)
		wrapper.removeChild(item);
		const newItem = newList.pop()
		if (newItem) {

			wrapper.append(newItem);
		} else {
		}
	}
})

const searchURL = "/api/v1/lufthavn/search";
async function search(byNavn) {
	const response = await fetch(searchURL + "?limit=5&municipality=" + byNavn);
	const data = await response.json();
	console.log(data);
	return data;
}
console.log(await search("bergen"));
