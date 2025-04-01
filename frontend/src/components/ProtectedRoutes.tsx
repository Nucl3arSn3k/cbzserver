import { LocationProvider, Router, Route } from 'preact-iso';
import { useEffect } from 'preact/hooks';
import { useLocation } from 'preact-iso';
import { useAuthContext } from '../context/AuthContext';
export function ProtectedRoute(props) {
    const { route } = useLocation();
    const { isAuthenticated } = useAuthContext();

    useEffect(() => {
        if (!isAuthenticated) route('/login', true);
    }, [isAuthenticated]);

    return <Route {...props} />;
}