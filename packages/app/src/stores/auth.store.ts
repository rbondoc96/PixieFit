import {createResource, type Resource} from 'solid-js';

import {AuthAPI} from '@/api';
import {type User} from '@/parsers/authParsers';

const [userResource, {mutate: setUser}] = createResource(fetchUser, {
    initialValue: null,
});

export const useUser = (): Resource<User | null> => userResource;

export async function fetchUser(): Promise<User|null> {
    try {
        await new Promise(resolve => setTimeout(resolve, 2000));
        return await AuthAPI.fetchUser();
    } catch (_error) {
        console.log('returning null user');
        return null;
    }
}

export async function login(payload: AuthAPI.LoginUserPayload): Promise<void> {
    const user = await AuthAPI.login(payload);
    setUser(user);
}

export async function logout(): Promise<void> {
    await AuthAPI.logout();
    console.log('logged out');
    setUser(null);
}

export async function register(payload: AuthAPI.RegisterUserPayload): Promise<void> {
    const user = await AuthAPI.register(payload);
    setUser(user);
}
