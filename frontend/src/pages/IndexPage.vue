<template>
  <div class="app">
    <div class="header">
      <h1>Todo App</h1>
      <p>by Adam Rachid</p>
    </div>

    <div class="stats">
      <div class="stat">
        <div class="stat-val">{{ todos.length }}</div>
        <div class="stat-lbl">total</div>
      </div>
      <div class="stat">
        <div class="stat-val">{{ todos.filter((t) => t.completed).length }}</div>
        <div class="stat-lbl">done</div>
      </div>
      <div class="stat">
        <div class="stat-val">{{ todos.filter((t) => !t.completed).length }}</div>
        <div class="stat-lbl">remaining</div>
      </div>
    </div>

    <div class="compose">
      <input v-model="newTask" placeholder="Add a new task…" @keyup.enter="addTodo" />
      <button class="btn" :disabled="adding" @click="addTodo">+ Add</button>
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
      <input v-model="search" placeholder="Search tasks…" />
    </div>

    <div class="filters">
      <button
        v-for="f in ['all', 'active', 'done']"
        :key="f"
        class="filter-btn"
        :class="{ active: filter === f }"
        @click="filter = f"
      >
        {{ f }}
      </button>
    </div>

    <div class="list" v-if="visible.length">
      <div
        v-for="todo in visible"
        :key="todo.id"
        class="todo-item"
        :class="{ done: todo.completed }"
      >
        <div class="check" :class="{ checked: todo.completed }" @click="toggleDone(todo)"></div>

        <input
          v-if="editingId === todo.id"
          class="todo-edit"
          v-model="editText"
          @keyup.enter="saveEdit(todo)"
          @keyup.escape="editingId = null"
          @blur="saveEdit(todo)"
          ref="editInput"
        />
        <span v-else class="todo-text">{{ todo.task }}</span>

        <span class="todo-id">#{{ todo.id }}</span>

        <div class="todo-actions">
          <button v-if="editingId !== todo.id" class="act-btn" @click="startEdit(todo)">
            edit
          </button>
          <button class="act-btn danger" @click="deleteTodo(todo.id)">delete</button>
        </div>
      </div>
    </div>

    <div v-else class="empty">No tasks here.</div>
    <div class="toast" :class="{ show: toast }">{{ toast }}</div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, nextTick } from 'vue'

const todos = ref([])
const newTask = ref('')
const search = ref('')
const filter = ref('all')
const editingId = ref(null)
const editText = ref('')
const adding = ref(false)
const editInput = ref(null)


const visible = computed(() => {
  let list = todos.value
  if (search.value)
    list = list.filter((t) => t.task.toLowerCase().includes(search.value.toLowerCase()))
  if (filter.value === 'active') list = list.filter((t) => !t.completed)
  if (filter.value === 'done') list = list.filter((t) => t.completed)
  return list
})

async function api( method, path, body ) {
  const res = await fetch("http://192.168.0.14:3000" + path, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined,
  })
  if (!res.ok) throw new Error(res.status)
  if (res.status === 204) return null
  return res.json()
}

async function loadTodos() {
  try {
    todos.value = await api('GET', '/todos')
  } catch { showToast('Could not load todos') }
}

async function addTodo() {
  const task = newTask.value.trim()
  if (!task) return
  adding.value = true
  try {
    const t = await api('POST', '/todos', { task })
    todos.value.unshift(t)
    newTask.value = ''
  } catch { showToast('Failed to add task') }
  adding.value = false
  adding.value = false
}

async function toggleDone(todo) {
  try {
    const updated = await api('PATCH', `/todos/${todo.id}/completed`, {
      completed: !todo.completed,
    })
    Object.assign(todo, updated)
  } catch { showToast('Failed to delete') }
}

async function deleteTodo(id) {
  try {
    await api('DELETE', `/todos/${id}`)
    todos.value = todos.value.filter((t) => t.id !== id)
  } catch { showToast('Failed to delete') }
}

function startEdit(todo) {
  editingId.value = todo.id
  editText.value = todo.task
  nextTick(() => editInput.value?.focus())
}

async function saveEdit(todo) {
  if (!editText.value.trim()) {
    editingId.value = null
    return
  }
  try {
    const updated = await api('PATCH', `/todos/${todo.id}`, { task: editText.value.trim() })
    Object.assign(todo, updated)
  } catch { showToast('Failed to update') }
  editingId.value = null
}

const toast = ref('')
let toastTimer = null
function showToast(msg) {
  toast.value = msg
  clearTimeout(toastTimer)
  toastTimer = setTimeout(() => (toast.value = ''), 20000)
}

onMounted(loadTodos)
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
  --text2: #6b6a65;
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
  text-align: center;
}
.header h1 {
  font-size: 3rem;
  font-weight: 500;
  color: var(--text);
  text-align: center;
}
.header p {
  font-size: 13px;
  color: var(--text2);
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
.search-bar input::placeholder {
  color: var(--text3);
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

.todo-item {
  background: var(--surface);
  border: 0.5px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 12px 14px;
  display: flex;
  align-items: center;
  gap: 12px;
  transition: border-color 0.1s;
}
.todo-item:hover {
  border-color: var(--border2);
}
.todo-item.done .todo-text {
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

.todo-text {
  flex: 1;
  font-size: 14px;
  color: var(--text);
  line-height: 1.4;
}
.todo-edit {
  flex: 1;
  border: none;
  outline: none;
  font-size: 14px;
  background: transparent;
  color: var(--text);
  font-family: var(--font);
  border-bottom: 1px solid var(--border2);
}
.todo-id {
  font-size: 11px;
  color: var(--text3);
  font-family: var(--mono);
}

.todo-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.1s;
}
.todo-item:hover .todo-actions {
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
.toast {
  position: fixed;
  bottom: 1.5rem;
  left: 50%;
  transform: translateX(-50%);
  background: var(--text);
  color: var(--bg);
  padding: 8px 18px;
  border-radius: var(--radius-md);
  font-size: 13px;
  opacity: 0;
  transition: opacity 0.2s;
  pointer-events: none;
  white-space: nowrap;
  z-index: 100;
}
.toast.show {
  opacity: 1;
}
</style>
