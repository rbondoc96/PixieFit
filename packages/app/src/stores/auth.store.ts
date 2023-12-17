import {createResource, type InitializedResource} from 'solid-js';

import {AuthAPI} from '@/api';
import {type User} from '@/parsers/authParsers';

const [userResource, {mutate: setUser}] = createResource(fetchUser, {
    initialValue: null,
});

export const useUser = (): InitializedResource<User | null> => userResource;

export async function fetchUser(): Promise<User|null> {
    try {
        return await AuthAPI.fetchUser();
    } catch (_error) {
        return null;
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
