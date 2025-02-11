import { createSignal, createResource } from 'solid-js'
import { render } from 'solid-js/web'
import { Router, Route } from "@solidjs/router"
import { ParentProps } from 'solid-js' // Add this import
import './App.css'
import Login from "./pages/Login"
import Settings from "./pages/Settings"

interface Library {
  name: string
  filepath: string
  dirornot: boolean
}

// Added ParentProps type to App
function App(props: ParentProps) {
  return (
    <>
      {props.children}
    </>
  )
}

function ComicList() {
  const [count, setCount] = createSignal(0)
  const [library] = createResource<Library>(async () => {
    const response = await fetch('/api/library')
    if (!response.ok) {
      throw new Error('Failed to fetch library')
    }
    return response.json()
  })

  return (
    <>
      <h1>Comic list</h1>
      {library.loading && <div>Loading library...</div>}
      {library.error && <div>Error loading library: {library.error.message}</div>}
      <div class="series-list">
        
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