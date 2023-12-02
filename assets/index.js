/**
 * Fetches data from the "/api/home" endpoint, then fetches data from the "/api/listdir/{data}" endpoint,
 * and fills a table with the retrieved data.
 *
 * @param {type} paramName - description of parameter
 * @return {type} description of return value
 */
function document_ready() {
  const pathInput = document.getElementById("pathInput");
  const goButton = document.getElementById("goButton");

  pathInput.addEventListener("keyup", (event) => {
    if (event.key === "Enter") {
      const path = pathInput.value;
      load_path(path);
    }
  });
  goButton.addEventListener("click", () => {
    const path = pathInput.value;
    load_path(path);
  });

  fetch("/api/home")
    .then((response) => response.json())
    .then((data) => {
      load_path(data);
    })
    .catch((error) => console.error("Error:", error));
}

function load_path(path) {
  fetch("/api/listdir?path=" + encodeURIComponent(path))
    .then((response) => response.json())
    .then((data) => fillTable(data))
    .catch((error) => console.error("Error:", error));
}

function fillTable(data) {
  const pathInput = document.getElementById("pathInput");

  const tableBody = document.getElementById("fileTable").querySelector("tbody");
  tableBody.innerHTML = "";

  data.entries.forEach((item) => {
    const row = document.createElement("tr");
    const size = formatSize(item.f_size).join(" ");
    row.innerHTML = `
        <td>${item.f_name}</td>
        <td>${item.f_type}</td>
        <td data-sort="${item.f_size}">${size}</td>
    `;
    tableBody.appendChild(row);
  });

  pathInput.value = data.requested_path;
}

/**
 * Formats the given size value into a human-readable format.
 *
 * @param {number} size - The size value to be formatted.
 * @return {Array} An array containing the formatted size value and its unit.
 */
function formatSize(size) {
  if (size == null) return [];
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  if (size == 0) return [0, "B"];
  const i = parseInt(Math.floor(Math.log(size) / Math.log(1024)));
  ratio = 1;
  if (i >= 3) {
    ratio = 100;
  }
  return [Math.round((size * ratio) / Math.pow(1024, i), 2) / ratio, sizes[i]];
}
