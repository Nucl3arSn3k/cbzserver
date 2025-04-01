import { createContext } from "preact";
import { useContext, useState } from "preact/hooks";

type AuthContextType = {
    isAuthenticated: boolean;
    login: (token: string) => void;
    logout: () => void;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: preact.ComponentChildren }) {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(() => {
        // Check if token exists in localStorage
        return !!localStorage.getItem('auth_token');
    });

    const login = (token: string) => {
        localStorage.setItem('auth_token', token);
        setIsAuthenticated(true);
    };

    const logout = () => {
        localStorage.removeItem('auth_token');
        setIsAuthenticated(false);
    };

    return (
        <AuthContext.Provider value={{ isAuthenticated, login, logout }}>
            {children}
        </AuthContext.Provider>
    );
}


export function useAuthContext() {
    const context = useContext(AuthContext)
    if (context == undefined) {

        throw new Error('use auth context must be used within an AuthProvider')
    }
    return context;

}