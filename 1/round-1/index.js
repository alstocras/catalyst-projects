let inputWordContent;
let language;
let typeOfLearning;
let languageJsonString;

async function loadJsonString() {
	// async bcs we want this to run in the background and not freeze the site
	const file = await fetch("./languages/" + language + ".json"); // await it so we wait for it
	if (!file.ok) {
		// this checks if the file actually was fetched
		console.log("unknown language/unable to find file");
	}
	const fileString = await file.text();
	languageJsonString = fileString;
}

function lang2eng() {
	typeOfLearning = "lang2eng";
}

function
