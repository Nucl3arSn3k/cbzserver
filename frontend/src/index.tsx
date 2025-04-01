import { render } from 'preact';
import { LocationProvider, Router, Route } from 'preact-iso';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { Header } from './components/Header.jsx';
import { Home } from './pages/Home/index.jsx';
import {ComicLibrary} from './pages/comiclibrary.jsx'
import { Login } from './pages/login.jsx';
import { NotFound } from './pages/_404.jsx';
import {ProtectedRoute} from './components/ProtectedRoutes.jsx'
import './style.css';

// Create a custom theme based on your existing CSS
const theme = createTheme({
  palette: {
    primary: {
      main: '#673ab8', // Purple from your header
      light: '#9059df',
      dark: '#4b2687',
    },
    secondary: {
      main: '#222222', // Dark color from your text
      light: '#484848',
      dark: '#000000',
    },
    background: {
      default: '#ffffff',
      paper: '#ffffff',
    },
    text: {
      primary: '#222222',
    },
    mode: 'light',
  },
  typography: {
    fontFamily: 'Inter, system-ui, Avenir, Helvetica, Arial, sans-serif',
    fontWeightRegular: 400,
    fontSize: 16,
  },
  shape: {
    borderRadius: 4,
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          textRendering: 'optimizeLegibility',
          WebkitFontSmoothing: 'antialiased',
          MozOsxFontSmoothing: 'grayscale',
        },
      },
    },
  },
});

// Create a dark theme variant
const darkTheme = createTheme({
  ...theme,
  palette: {
    ...theme.palette,
    mode: 'dark',
    background: {
      default: '#1a1a1a',
      paper: '#2a2a2a',
    },
    text: {
      primary: '#cccccc',
    },
  },
});

// Detect if user prefers dark color scheme
// You could also make this dynamic with a toggle
const prefersDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
const activeTheme = prefersDarkMode ? darkTheme : theme;

export function App() {
    return (
        <ThemeProvider theme={activeTheme}>
            <CssBaseline />
            <LocationProvider>
                <Header />
                <main>
                    <Router>
                        <ProtectedRoute path="/home" component={ComicLibrary} />
                        <Route path="/login" component={Login} />
                        <Route path="/" component={Home} />
                        <Route default component={NotFound} />
                    </Router>
                </main>
            </LocationProvider>
        </ThemeProvider>
    );
}

render(<App />, document.getElementById('app'));