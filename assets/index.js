/**
 * The current directory. This should be an absolute path.
 */
let g_current_directory = "";

const ICONS = {
  dir: `<svg height="16" viewBox="0 0 14 16" width="14"><path fill-rule="evenodd" d="M13 4H7V3c0-.66-.31-1-1-1H1c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zM6 4H1V3h5v1z"></path></svg>`,
  symlinkFile: `<svg height="16" viewBox="0 0 12 16" width="12"><path fill-rule="evenodd" d="M8.5 1H1c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h10c.55 0 1-.45 1-1V4.5L8.5 1zM11 14H1V2h7l3 3v9zM6 4.5l4 3-4 3v-2c-.98-.02-1.84.22-2.55.7-.71.48-1.19 1.25-1.45 2.3.02-1.64.39-2.88 1.13-3.73.73-.84 1.69-1.27 2.88-1.27v-2H6z"></path></svg>`,
  symlinkDir: `<svg height="16" viewBox="0 0 14 16" width="14"><path fill-rule="evenodd" d="M13 4H7V3c0-.66-.31-1-1-1H1c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zM1 3h5v1H1V3zm6 9v-2c-.98-.02-1.84.22-2.55.7-.71.48-1.19 1.25-1.45 2.3.02-1.64.39-2.88 1.13-3.73C4.86 8.43 5.82 8 7.01 8V6l4 3-4 3H7z"></path></svg>`,
  file: `<svg height="16" viewBox="0 0 12 16" width="12"><path fill-rule="evenodd" d="M6 5H2V4h4v1zM2 8h7V7H2v1zm0 2h7V9H2v1zm0 2h7v-1H2v1zm10-7.5V14c0 .55-.45 1-1 1H1c-.55 0-1-.45-1-1V2c0-.55.45-1 1-1h7.5L12 4.5zM11 5L8 2H1v12h10V5z"></path></svg>`,
  download: `<svg width="16" height="16" viewBox="0 0 16 16"><path d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5z"/><path d="M7.646 11.854a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 10.293V1.5a.5.5 0 0 0-1 0v8.793L5.354 8.146a.5.5 0 1 0-.708.708l3 3z"/></svg>`,
  move: `<svg width="16" height="16" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M1.5 1.5A.5.5 0 0 0 1 2v4.8a2.5 2.5 0 0 0 2.5 2.5h9.793l-3.347 3.346a.5.5 0 0 0 .708.708l4.2-4.2a.5.5 0 0 0 0-.708l-4-4a.5.5 0 0 0-.708.708L13.293 8.3H3.5A1.5 1.5 0 0 1 2 6.8V2a.5.5 0 0 0-.5-.5z"/></svg>`,
  edit: `<svg width="16" height="16" viewBox="0 0 16 16"><path d="M12.146.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-10 10a.5.5 0 0 1-.168.11l-5 2a.5.5 0 0 1-.65-.65l2-5a.5.5 0 0 1 .11-.168l10-10zM11.207 2.5 13.5 4.793 14.793 3.5 12.5 1.207 11.207 2.5zm1.586 3L10.5 3.207 4 9.707V10h.5a.5.5 0 0 1 .5.5v.5h.5a.5.5 0 0 1 .5.5v.5h.293l6.5-6.5zm-9.761 5.175-.106.106-1.528 3.821 3.821-1.528.106-.106A.5.5 0 0 1 5 12.5V12h-.5a.5.5 0 0 1-.5-.5V11h-.5a.5.5 0 0 1-.468-.325z"/></svg>`,
  delete: `<svg width="16" height="16" viewBox="0 0 16 16"><path d="M6.854 7.146a.5.5 0 1 0-.708.708L7.293 9l-1.147 1.146a.5.5 0 0 0 .708.708L8 9.707l1.146 1.147a.5.5 0 0 0 .708-.708L8.707 9l1.147-1.146a.5.5 0 0 0-.708-.708L8 8.293 6.854 7.146z"/><path d="M14 14V4.5L9.5 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2zM9.5 3A1.5 1.5 0 0 0 11 4.5h2V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5.5v2z"/></svg>`,
  view: `<svg width="16" height="16" viewBox="0 0 16 16"><path d="M4 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2zm0 1h8a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1"/></svg>`,
};

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

  /**
   * Get json data from fs_data
   */
  const fs_data = JSON.parse(document.getElementById("fs_data").textContent);

  /* Set the current directory and update title */
  g_current_directory = fs_data.requested_path;
  document.title = `Index of ${g_current_directory}`;

  /* Fill the table */
  fillTable(fs_data.requested_path, fs_data.entries);
}

function fillTable(path, data) {
  const pathInput = document.getElementById("pathInput");

  const tableBody = document.getElementById("fileTable").querySelector("tbody");
  tableBody.innerHTML = "";

  data.forEach((item) => {
    /* Create a table row */
    const row = document.createElement("tr");
    /* Fill the table row */
    const path = "/fs/" + encodeURIComponent(item.f_path);
    const size = formatSize(item.f_size).join(" ");
    const time = format_epoch_as_local(item.f_modified);
    row.innerHTML = `
        <td><a href="${path}">${item.f_name}</a></td>
        <td>${item.f_type}</td>
        <td data-sort="${item.f_size}">${size}</td>
        <td data-sort="${item.f_modified}">${time}</td>
        <td><div>${ICONS.download}</div></td>
    `;
    tableBody.appendChild(row);
  });

  pathInput.value = path;
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
    "/upload?path=" + encodeURIComponent(g_current_directory),
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
      window.location.reload(true);
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
