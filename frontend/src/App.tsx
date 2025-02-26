import { createSignal, createResource, Show, For } from 'solid-js'
import { render } from 'solid-js/web'
import { Router, Route } from "@solidjs/router"
import { ParentProps } from 'solid-js'
import { ListGroup } from 'solid-bootstrap'
import './App.css'
import Login from "./pages/Login"
import Settings from "./pages/Settings"

interface Item {
  name: string
  filepath: string
  cover_path: string
  dirornot: boolean
}

interface Library {
  items: Item[]
}

function App(props: ParentProps) {
  return (
    <>
      {props.children}
    </>
  )
}

function ComicList() {
  const [selectedItem, setSelectedItem] = createSignal<string | null>(null)
  
  // Update the type definition to match your API response
  const [library] = createResource(async () => {
    const response = await fetch('/api/library')
    if (!response.ok) {
      throw new Error('Failed to fetch library')
    }
    return response.json()
  })
  
  // Function to get top-level folders only
  const topLevelFolders = () => {
    if (!library() || !library().items) {
      return [];
    }
    
    // Filter items that are directories and in the top level
    return library().items.filter((item: { dirornot: any; filepath: string }) => {
      if (!item.dirornot) return false; // Only directories
      
      // Only include top-level folders
      const pathParts = item.filepath.split('/');
      return pathParts.length <= 2;
    });
  }
  
  return (
    <>
      <h1>Comic list</h1>
      <Show when={library.loading}>
        <div class="loading">Loading library...</div>
      </Show>
      <Show when={library.error}>
        <div class="error">Error loading library: {library.error.message}</div>
      </Show>
      <div class="series-list">
        <Show when={library()} fallback={<p>No folders found</p>}>
          <ListGroup>
            <For each={topLevelFolders()}>
              {(folder) => (
                <ListGroup.Item 
                  action 
                  active={selectedItem() === folder.filepath}
                  onClick={() => setSelectedItem(folder.filepath)}
                >
                  {folder.name}
                </ListGroup.Item>
              )}
            </For>
          </ListGroup>
        </Show>
      </div>
    </>
  )
}

const root = document.getElementById('root')
if (root) {
  render(() => (
    <Router root={App}>
      <Route path="/" component={Login} />
      <Route path="/login" component={ComicList} />
      <Route path="/settings" component={Settings} />
    </Router>
  ), root)
}

export default App