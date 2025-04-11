
import '../style.css'
import { useLocation } from 'preact-iso';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
export function Login() {
    const { route } = useLocation();
    const handleSubmit = (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);

        fetch('/api/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                username: formData.get('username'),
                password: formData.get('password')
            })
        })
            .then(response => {
                if (response.ok) {
                    route('/');
                }
            });
    }

    return (
        <div>
            <h1>Login</h1>
            <form onSubmit={handleSubmit} style={{ maxWidth: "400px", margin: "2rem auto" }}>
                <div style={{ marginBottom: "1rem" }}>
                    <label for="username" style={{ display: "block", marginBottom: "0.5rem" }}>
                        Username:
                    </label>
                    <input type="text" id="username" name="username" required style={{ width: "100%" }} />
                </div>
                <div style={{ marginBottom: "1rem" }}>
                    <label for="password" style={{ display: "block", marginBottom: "0.5rem" }}>
                        Password:
                    </label>
                    <input type="password" id="password" name="password" required style={{ width: "100%" }} />
                </div>
                <button type="submit" style={{ width: "100%", marginTop: "1rem" }}>
                    Login
                </button>
            </form>
        </div>
    );
}