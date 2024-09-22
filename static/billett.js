const mybutton = document.querySelector("#my-button");

const wrapper = ".v";
function getListItems() {
	return document.querySelectorAll(`${wrapper}>div:has(input:not(:checked))`)
}
mybutton.addEventListener("click", () => {
	const list = getListItems();
	const w = document.querySelector(wrapper);
	for (const item of list) {
		console.log(item)
		w.removeChild(item);
	}
})
