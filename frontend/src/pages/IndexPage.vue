<template>
  <div class="app">
    <div class="header">
      <div class="header-main">
        <div class="header-title">
          <h1>Tasks Management App</h1>
        </div>
        <div class="header-actions">
          <button class="act-btn" @click="exportCSV">Export csv</button>
          <label class="act-btn">
            Import csv
            <input type="file" @change="importCSV" accept=".csv" hidden />
          </label>
        </div>
      </div>
    </div>

    <div class="stats">
      <div class="stat">
        <div class="stat-val">{{ tasks.length }}</div>
        <div class="stat-lbl">Total</div>
      </div>
      <div class="stat">
        <div class="stat-val">{{ tasks.filter((t) => t.completed).length }}</div>
        <div class="stat-lbl">Done</div>
      </div>
      <div class="stat">
        <div class="stat-val">{{ tasks.filter((t) => !t.completed).length }}</div>
        <div class="stat-lbl">Remaining</div>
      </div>
    </div>

    <div class="compose">
      <input v-model="newTaskInput" placeholder="Add a new task…" @keyup.enter="addTask" />
      <button class="btn" :disabled="adding" @click="addTask">+ Add</button>
    </div>

    <div class="search-bar">
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <circle cx="6.5" cy="6.5" r="5" stroke="currentColor" stroke-width="1.5" />
        <path
          d="M10.5 10.5L14 14"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
        />
      </svg>
      <input v-model="searchQuery" placeholder="Search tasks…" @input="handleSearch" />
    </div>

    <div class="filters">
      <button
        v-for="f in ['all', 'active', 'done']"
        :key="f"
        class="filter-btn"
        :class="{ active: currentFilter === f }"
        @click="currentFilter = f"
      >
        {{ f }}
      </button>
    </div>

    <div class="list" v-if="visibleTasks.length">
      <div
        v-for="task in visibleTasks"
        :key="task.id"
        class="task-item"
        :class="{ done: task.completed }"
      >
        <div class="check" :class="{ checked: task.completed }" @click="toggleTask(task)"></div>

        <div class="task-content">
          <input
            v-if="editingId === task.id"
            class="task-edit"
            v-model="editTaskText"
            @keyup.enter="saveEdit(task)"
            @keyup.escape="editingId = null"
            @blur="saveEdit(task)"
            ref="taskEditInput"
          />
          <span v-else class="task-text" @dblclick="startEdit(task)">{{ task.task }}</span>

          <div class="task-meta">
            Created: {{ formatDate(task.created_at) }}
            <span v-if="task.completed_at"> • Done: {{ formatDate(task.completed_at) }}</span>
          </div>
        </div>

        <span class="task-id">#{{ task.id }}</span>

        <div class="task-actions">
          <button v-if="editingId !== task.id" class="act-btn" @click="startEdit(task)">
            edit
          </button>
          <button class="act-btn danger" @click="deleteTask(task.id)">delete</button>
        </div>
      </div>
    </div>

    <div v-else class="empty">No tasks here.</div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, nextTick } from 'vue'

const API_BASE = `${import.meta.env.VITE_API_URL}/tasks`

const tasks = ref([])
const newTaskInput = ref('')
const searchQuery = ref('')
const currentFilter = ref('all')
const editingId = ref(null)
const editTaskText = ref('')
const adding = ref(false)
const taskEditInput = ref(null)

const formatDate = (dateStr) => {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const visibleTasks = computed(() => {
  let list = tasks.value
  if (currentFilter.value === 'active') list = list.filter((t) => !t.completed)
  if (currentFilter.value === 'done') list = list.filter((t) => t.completed)
  return list
})

async function loadTasks() {
  try {
    const res = await fetch(API_BASE)
    tasks.value = await res.json()
  } catch (e) {
    console.error(e)
  }
}

let searchTimeout
async function handleSearch() {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(async () => {
    if (!searchQuery.value.trim()) {
      loadTasks()
      return
    }
    try {
      const res = await fetch(`${API_BASE}/search/${encodeURIComponent(searchQuery.value)}`)
      const result = await res.json()
      tasks.value = result.data
    } catch (e) {
      console.error(e)
    }
  }, 300)
}

async function addTask() {
  const taskContent = newTaskInput.value.trim()
  if (!taskContent || adding.value) return
  adding.value = true
  try {
    const res = await fetch(API_BASE, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ task: taskContent }),
    })
    const newTask = await res.json()
    tasks.value.unshift(newTask)
    newTaskInput.value = ''
  } catch (e) {
    console.error(e)
  }
  adding.value = false
}

async function toggleTask(task) {
  try {
    const res = await fetch(`${API_BASE}/${task.id}/completed`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ completed: !task.completed }),
    })
    const updated = await res.json()
    Object.assign(task, updated)
  } catch (e) {
    console.error(e)
  }
}

async function deleteTask(id) {
  try {
    await fetch(`${API_BASE}/${id}`, { method: 'DELETE' })
    tasks.value = tasks.value.filter((t) => t.id !== id)
  } catch (e) {
    console.error(e)
  }
}

async function exportCSV() {
  window.location.href = `${API_BASE}/export`
}

async function importCSV(event) {
  const file = event.target.files[0]
  if (!file) return
  const formData = new FormData()
  formData.append('file', file)
  try {
    await fetch(`${API_BASE}/import-csv`, { method: 'POST', body: formData })
    loadTasks()
  } catch (e) {
    console.error(e)
  }
}

function startEdit(task) {
  editingId.value = task.id
  editTaskText.value = task.task
  nextTick(() => {
    if (Array.isArray(taskEditInput.value)) {
      taskEditInput.value[0]?.focus()
    } else {
      taskEditInput.value?.focus()
    }
  })
}

async function saveEdit(task) {
  const val = editTaskText.value.trim()
  if (!val || val === task.task) {
    editingId.value = null
    return
  }
  try {
    const res = await fetch(`${API_BASE}/${task.id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ task: val }),
    })
    const updated = await res.json()
    Object.assign(task, updated)
  } catch (e) {
    console.error(e)
  }
  editingId.value = null
}

onMounted(loadTasks)
</script>

<style>
*,
*::before,
*::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  --bg: #f5f4f0;
  --surface: #ffffff;
  --surface2: #f0efe9;
  --text: #1a1a18;
  --text2: #666665;
  --text3: #a8a7a2;
  --border: rgba(0, 0, 0, 0.1);
  --border2: rgba(0, 0, 0, 0.18);
  --radius-md: 8px;
  --radius-lg: 12px;
  --font: 'Segoe UI', system-ui, sans-serif;
  --mono: 'Fira Mono', 'Consolas', monospace;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #18181a;
    --surface: #222224;
    --surface2: #2a2a2c;
    --text: #f0efea;
    --text2: #9a9996;
    --text3: #5a5956;
    --border: rgba(255, 255, 255, 0.1);
    --border2: rgba(255, 255, 255, 0.18);
  }
}

body {
  font-family: var(--font);
  background: var(--bg) !important;
  min-height: 100vh;
  color: var(--text);
}

.app {
  max-width: 640px;
  margin: 0 auto;
  padding: 2.5rem 1.25rem;
}

.header {
  margin-bottom: 2rem;
}

.header-main {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}
.header h1 {
  font-size: 24px;
  font-weight: 500;
  color: var(--text);
}
.header p {
  font-size: 13px;
  color: var(--text2);
  margin-top: 4px;
}
.header-actions {
  display: flex;
  gap: 8px;
}

.stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
  margin-bottom: 1.5rem;
}
.stat {
  background: var(--surface2);
  border-radius: var(--radius-md);
  padding: 12px 14px;
}
.stat-val {
  font-size: 22px;
  font-weight: 500;
  color: var(--text);
}
.stat-lbl {
  font-size: 12px;
  color: var(--text2);
  margin-top: 2px;
}

.compose {
  background: var(--surface);
  border: 0.5px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 12px 16px;
  margin-bottom: 1.25rem;
  display: flex;
  gap: 10px;
}
.compose input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 14px;
  background: transparent;
  color: var(--text);
  font-family: var(--font);
}
.compose input::placeholder {
  color: var(--text3);
}

.btn {
  background: var(--text);
  color: var(--bg);
  border: none;
  border-radius: var(--radius-md);
  padding: 7px 16px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  font-family: var(--font);
  white-space: nowrap;
  transition: opacity 0.15s;
}
.btn:hover {
  opacity: 0.82;
}
.btn:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

.search-bar {
  background: var(--surface);
  border: 0.5px solid var(--border);
  border-radius: var(--radius-md);
  padding: 8px 12px;
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
  gap: 8px;
}
.search-bar svg {
  flex-shrink: 0;
  opacity: 0.4;
}
.search-bar input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 13px;
  background: transparent;
  color: var(--text);
  font-family: var(--font);
}

.filters {
  display: flex;
  gap: 6px;
  margin-bottom: 1rem;
}
.filter-btn {
  background: transparent;
  border: 0.5px solid var(--border);
  border-radius: 20px;
  padding: 5px 14px;
  font-size: 12px;
  cursor: pointer;
  color: var(--text2);
  font-family: var(--font);
  transition: all 0.1s;
  text-transform: capitalize;
}
.filter-btn.active {
  background: var(--text);
  color: var(--bg);
  border-color: var(--text);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.task-item {
  background: var(--surface);
  border: 0.5px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 12px 14px;
  display: flex;
  align-items: center;
  gap: 12px;
  transition: border-color 0.1s;
}
.task-item:hover {
  border-color: var(--border2);
}
.task-item.done .task-text {
  text-decoration: line-through;
  color: var(--text3);
}

.check {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 1.5px solid var(--border2);
  cursor: pointer;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}
.check.checked {
  background: var(--text);
  border-color: var(--text);
}
.check.checked::after {
  content: '';
  display: block;
  width: 5px;
  height: 9px;
  border: 2px solid var(--bg);
  border-top: none;
  border-left: none;
  transform: rotate(45deg) translateY(-1px);
}

.task-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}
.task-text {
  font-size: 14px;
  color: var(--text);
  line-height: 1.4;
}
.task-meta {
  font-size: 10px;
  color: var(--text3);
  margin-top: 2px;
}

.task-edit {
  border: none;
  outline: none;
  font-size: 14px;
  background: transparent;
  color: var(--text);
  font-family: var(--font);
  border-bottom: 1px solid var(--border2);
}
.task-id {
  font-size: 11px;
  color: var(--text3);
  font-family: var(--mono);
}

.task-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.1s;
}
.task-item:hover .task-actions {
  opacity: 1;
}

.act-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
  color: var(--text2);
  font-size: 12px;
  font-family: var(--font);
  transition:
    background 0.1s,
    color 0.1s;
}
.act-btn:hover {
  background: var(--surface2);
  color: var(--text);
}
.act-btn.danger:hover {
  background: #fcebeb;
  color: #a32d2d;
}
@media (prefers-color-scheme: dark) {
  .act-btn.danger:hover {
    background: #501313;
    color: #f7c1c1;
  }
}

.empty {
  text-align: center;
  padding: 3rem 1rem;
  color: var(--text3);
  font-size: 14px;
}
</style>
