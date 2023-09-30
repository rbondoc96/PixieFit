import {type Accessor, createSignal} from 'solid-js';

import {AuthAPI} from '@/api';
import {type User} from '@/parsers/authParsers';

interface AuthStore {
    fetchUser: () => Promise<void>;
    login: (payload: AuthAPI.LoginUserPayload) => Promise<void>;
    logout: () => Promise<void>;
    register: (payload: AuthAPI.RegisterUserPayload) => Promise<void>;
    user: Accessor<User | null>;
}

const [user, setUser] = createSignal<User | null>(null);

export async function fetchUser(): Promise<void> {
    try {
        const user = await AuthAPI.fetchUser();
        setUser(user);
    } catch (error) {
        setUser(null);
        throw error;
    }
}

export async function login(payload: AuthAPI.LoginUserPayload): Promise<void> {
    const user = await AuthAPI.login(payload);
    setUser(user);
}

export async function logout(): Promise<void> {
    await AuthAPI.logout();
    setUser(null);
}

export async function register(payload: AuthAPI.RegisterUserPayload): Promise<void> {
    const user = await AuthAPI.register(payload);
    setUser(user);
}

export function useAuthenticatedUser(): User {
    const authenticatedUser = user();

    if (authenticatedUser === null) {
        throw new Error('User is not authenticated');
    }

    return authenticatedUser;
}

export default {
    fetchUser,
    login,
    logout,
    register,
    user,
} satisfies AuthStore;
