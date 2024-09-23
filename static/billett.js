const wrapperName = ".lufthavn-search";

const searchWrappers = document.querySelectorAll(wrapperName);

const limit = 5;

function getSearchItems(wrapper) {
	return wrapper.querySelectorAll(`div>div:has(input:not(:checked))`)
}

function createAirportElement(data, name) {
	const item = document.createElement("div");

	const label = document.createElement("label");
	label.setAttribute("for", data.iata_code);
	console.log(data.iata_code);
	label.textContent = data.name;

	const input = document.createElement("input");
	input.type = "radio"
	input.name = name;
	input.value = data.iata_code;
	input.id = data.iata_code;
	item.append(label, input);
	return item;
}

searchWrappers.forEach(searchWrapper => searchWrapper.addEventListener("keyup", async e => {
	e.preventDefault();
	const searchElement = searchWrapper.querySelector("input[type=\"text\"]");
	console.log(searchElement);
	const value = searchElement.value;

	const optionsWrapper = searchWrapper.querySelector(".options");
	const name = optionsWrapper.dataset.name;
	const newList = (await search(value)).map(data => createAirportElement(data, name));
	console.log(newList);
	const list = getSearchItems(optionsWrapper);
	console.log(list);
	const findDuplicate = value => optionsWrapper.querySelector(`div>div>input[value="${value}"]`)
	for (let i = 0; i < limit; i++) {
		if (list[i]) {
			optionsWrapper.removeChild(list[i]);
		}
		if (newList[i] && !findDuplicate(newList[i].querySelector("input").value)) {
			optionsWrapper.append(newList[i]);
		}
	}
})
);

const searchURL = "/api/v1/lufthavn/search";
async function search(byNavn) {
	const response = await fetch(searchURL + "?limit=" + limit + "&municipality=" + byNavn);
	const data = await response.json();
	return data;
}
