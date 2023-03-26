document
  .getElementById("csv-file")
  .addEventListener("change", handleFileSelect);
document.getElementById("add-task-form").addEventListener("submit", addTask);
document
  .getElementById("filter-tasks-form")
  .addEventListener("submit", filterTasks);

let tasks = [];

function handleFileSelect(event) {
  const file = event.target.files[0];
  const reader = new FileReader();
  reader.onload = function (e) {
    const csvData = e.target.result;
    parseCSV(csvData);
  };
  reader.readAsText(file);
}

function parseCSV(csvData) {
  const rows = csvData.split("\n");
  tasks = rows.map((row) => {
    const [name, tag, status] = row.split(",");
    return { name, tag, status };
  });
  renderTasks();
}

function renderTasks() {
  const tbody = document.getElementById("task-table").querySelector("tbody");
  tbody.innerHTML = "";
  tasks.forEach((task, index) => {
    const row = tbody.insertRow();
    row.insertCell().textContent = task.name;
    row.insertCell().textContent = task.tag;
    row.insertCell().textContent = task.status;
    const actionsCell = row.insertCell();
    const removeBtn = document.createElement("button");
    removeBtn.textContent = "Remove";
    removeBtn.addEventListener("click", () => removeTask(index));
    actionsCell.appendChild(removeBtn);
  });
}

function addTask(event) {
  event.preventDefault();
  const name = document.getElementById("task-name").value;
  const tag = document.getElementById("task-tag").value;
  const status = document.getElementById("task-status").value;
  tasks.push({ name, tag, status });
  renderTasks();
}

function removeTask(index) {
  tasks.splice(index, 1);
  renderTasks();
}

function filterTasks(event) {
  event.preventDefault();
  const filterTag = document.getElementById("filter-tag").value;
  const filterStatus = document.getElementById("filter-status").value;
  const filteredTasks = tasks.filter((task) => {
    return (
      (!filterTag || task.tag === filterTag) &&
      (!filterStatus || task.status === filterStatus)
    );
  });
  tasks = filteredTasks;
  renderTasks();
}
