// pages/Login.tsx
import { createSignal } from 'solid-js'

export default function Login() {
  const [username, setUsername] = createSignal('')
  const [password, setPassword] = createSignal('')

  const handleSubmit = async (e: Event) => {
    e.preventDefault()
    const response = await fetch('/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username: username(),
        password: password()
      })
    })
    if (response.ok) {
      // Redirect or handle successful login
    }
  }

  return (
    <>
      <h1>Login</h1>
      <form onSubmit={handleSubmit} style={{
        "max-width": "400px",
        "margin": "2rem auto"
      }}>
        <div style={{ "margin-bottom": "1rem" }}>
          <label 
            for="username" 
            style={{ 
              "display": "block",
              "margin-bottom": "0.5rem"
            }}
          >
            Username:
          </label>
          <input
            type="text"
            id="username"
            value={username()}
            onInput={(e) => setUsername(e.currentTarget.value)}
            required
            style={{ "width": "100%" }}
          />
        </div>
        <div style={{ "margin-bottom": "1rem" }}>
          <label 
            for="password"
            style={{ 
              "display": "block",
              "margin-bottom": "0.5rem"
            }}
          >
            Password:
          </label>
          <input
            type="password"
            id="password"
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
            required
            style={{ "width": "100%" }}
          />
        </div>
        <button 
          type="submit"
          style={{
            "width": "100%",
            "margin-top": "1rem"
          }}
        >
          Login
        </button>
      </form>
    </>
  )
}