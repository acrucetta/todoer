let tasks = [];

const folderPath = '/Users/andrescrucettanieto/Downloads/andres-vault/Areas/journal/thoughts/';
const fileName = 'tasks.csv';
const url = folderPath + fileName;

function fetchCSV() {
  fetch(url)
    .then(response => response.text())
    .then(csvData => parseCSV(csvData))
    .catch(error => console.error('Error fetching CSV:', error));
}

function parseCSV(csvData) {
  const rows = csvData.split('\n');
  tasks = rows.slice(1).map(row => {
    const [id, description, tags, due, timestamp, priority, status] = row.split(',');

    return {
      id: parseInt(id),
      description,
      tags: tags.split(';'),
      due,
      timestamp: new Date(parseInt(timestamp) * 1000),
      priority,
      status
    };
  });
  renderTasks();
}

function renderTasks() {
  const tbody = document.getElementById('task-table').querySelector('tbody');
  tbody.innerHTML = '';
  tasks.forEach((task, index) => {
    const row = tbody.insertRow();
    row.insertCell().textContent = task.id;
    row.insertCell().textContent = task.description;
    row.insertCell().textContent = task.tags.join(', ');
    row.insertCell().textContent = task.due;
    row.insertCell().textContent = task.timestamp.toLocaleString();
    row.insertCell().textContent = task.priority;
    row.insertCell().textContent = task.status;
    const actionsCell = row.insertCell();
    const removeBtn = document.createElement('button');
    removeBtn.textContent = 'Remove';
    removeBtn.addEventListener('click', () => removeTask(index));
    actionsCell.appendChild(removeBtn);
  });
}

function addTask(e) {
  e.preventDefault();
  const id = tasks.length > 0 ? Math.max(...tasks.map(task => task.id)) + 1 : 1;
  const description = document.getElementById('task-description').value;
  const tags = document.getElementById('task-tags').value.split(';');
  const due = document.getElementById('task-due').value;
  const timestamp = Date.now();
  const priority = document.getElementById('task-priority').value;
  const status = document.getElementById('task-status').value;

  tasks.push({ id, description, tags, due, timestamp, priority, status });
  renderTasks();
  updateCSV();
  document.getElementById('task-form').reset();
}

function removeTask(index) {
  tasks.splice(index, 1);
  renderTasks();
  updateCSV();
}

function updateCSV() {
  const header = 'id,description,tags,due,timestamp,priority,status';
  const csvRows = tasks.map(task => `${task.id},${task.description},${task.tags.join(';')},${task.due},${Math.floor(task.timestamp.getTime() / 1000)},${task.priority},${task.status}`);
  const csvData = [header, ...csvRows].join('\n');

  // Update the CSV file here
}

document.getElementById('task-form').addEventListener('submit', addTask);
fetchCSV();
