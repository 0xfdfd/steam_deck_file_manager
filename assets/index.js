/**
 * The current directory. This should be an absolute path.
 */
let g_current_directory = "";

/**
 * Fetches data from the "/api/home" endpoint, then fetches data from the "/api/listdir/{data}" endpoint,
 * and fills a table with the retrieved data.
 *
 * @param {type} paramName - description of parameter
 * @return {type} description of return value
 */
function document_ready() {
  const uploadButton = document.getElementById("uploadButton");
  uploadButton.addEventListener("click", () => {
    upload_file();
  });

  const pathInput = document.getElementById("pathInput");
  pathInput.addEventListener("keyup", (event) => {
    if (event.key === "Enter") {
      const path = pathInput.value;
      file_explorer_reload_path(path);
    }
  });

  const goButton = document.getElementById("goButton");
  goButton.addEventListener("click", () => {
    const path = pathInput.value;
    file_explorer_reload_path(path);
  });

  fetch("/api/home")
    .then((response) => response.json())
    .then((data) => {
      file_explorer_reload_path(data);
    })
    .catch((error) => console.error("Error:", error));
}

/**
 * Reloads the file explorer with the contents of the specified path.
 *
 * @param {string} path - The path to the directory to be loaded.
 * @return {undefined} No return value.
 */
function file_explorer_reload_path(path) {
  fetch("/api/listdir?path=" + encodeURIComponent(path))
    .then((response) => response.json())
    .then((data) => fillTable(data))
    .catch((error) => console.error("Error:", error));
}

function fillTable(data) {
  const pathInput = document.getElementById("pathInput");
  g_current_directory = data.requested_path;

  const tableBody = document.getElementById("fileTable").querySelector("tbody");
  tableBody.innerHTML = "";

  data.entries.forEach((item) => {
    const row = document.createElement("tr");
    const size = formatSize(item.f_size).join(" ");
    const time = format_epoch_as_local(item.f_modified);
    row.innerHTML = `
        <td>${item.f_name}</td>
        <td>${item.f_type}</td>
        <td data-sort="${item.f_size}">${size}</td>
        <td data-sort="${item.f_modified}">${time}</td>
    `;
    tableBody.appendChild(row);
  });

  pathInput.value = data.requested_path;
}

/**
 * Formats the given epoch (in seconds) as a local date and time string.
 *
 * @param {number} epoch - The epoch time to format.
 * @return {string} The formatted local date and time string.
 */
function format_epoch_as_local(epoch) {
  // Convert the epoch to a Date object
  const date = new Date(epoch * 1000);

  // Format the date to local time
  let year = date.getFullYear();
  let month = (date.getMonth() + 1).toString().padStart(2, "0");
  let day = date.getDate().toString().padStart(2, "0");
  let hours = date.getHours().toString().padStart(2, "0");
  let minutes = date.getMinutes().toString().padStart(2, "0");
  let seconds = date.getSeconds().toString().padStart(2, "0");

  // Return the formatted date and time
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
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

function upload_file() {
  const fileInput = document.getElementById("fileInput");
  const uploadProgress = document.getElementById("uploadProgress");

  if (fileInput.files.length === 0) {
    alert("Please select a file to upload.");
    return;
  }

  const file = fileInput.files[0];
  const formData = new FormData();
  formData.append("file", file);

  const xhr = new XMLHttpRequest();
  xhr.open(
    "POST",
    "/api/upload?path=" + encodeURIComponent(g_current_directory),
    true
  ); // true for asynchronous

  xhr.upload.addEventListener("progress", (event) => {
    if (event.lengthComputable) {
      const percent = Math.round((event.loaded / event.total) * 100);
      uploadProgress.value = percent;
    }
  });

  xhr.onload = () => {
    if (xhr.status === 200) {
      alert("File uploaded successfully.");

      // Once successfully uploaded, reload the current directory.
      file_explorer_reload_path(g_current_directory);
    } else {
      alert("Error uploading file.");
    }
    uploadProgress.value = 0;
  };

  xhr.onerror = () => {
    alert("Error uploading file.");
    uploadProgress.value = 0;
  };

  xhr.send(formData);
}
