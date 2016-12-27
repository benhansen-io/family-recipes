"use strict";

let recipes = [];

function compareRecipes(a, b) {
  if (a.name < b.name) {
    return -1;
  } else if (a.name == b.name) {
    return 0;
  } else {
    return 1;
  }
}

function onRecipeListDownloaded(data) {
  data.sort(compareRecipes);
  recipes = data;
  updateVisibleRecipeList();
}

const searchBar = document.getElementById("search-bar");

searchBar.addEventListener("input", updateVisibleRecipeList);

function matchesSearch(recipe, searchText) {
  const parts = searchText.toLowerCase().split(" ");
  for (const part of parts) {
    if (recipe.name.toLowerCase().indexOf(part) == -1) {
      return false;
    }
  }
  return true;
}


function updateVisibleRecipeList() {
  const domList = document.getElementById("recipe-list");
  domList.innerHTML = "";

  const searchText = searchBar.value;

  for (let recipe of recipes) {
    if (searchText == "" || matchesSearch(recipe, searchText)) {
      const item = document.createElement("a");
      item.classList.add("list-group-item");
      item.innerText = recipe.name;
      item.href = recipe.path;
      domList.appendChild(item);
    }
  }
}


$.ajax({
  url: "index.json",
  success: onRecipeListDownloaded,
  dataType: "json"
});
