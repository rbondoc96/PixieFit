import {useNavigate} from '@solidjs/router';
import {type Component} from 'solid-js';

import {type LoginUserPayload, loginUserPayloadSchema} from '@/api/auth';
import Helmet from '@/components/Helmet';
import Logo from '@/components/Logo';
import RouterLink from '@/components/RouterLink';
import createZorm from '@/components/Zorm';
import useUnauthGuard from '@/hooks/useUnauthGuard';
import {login} from '@/stores/auth.store';

import styles from './styles.module.scss';

const Zorm = createZorm(loginUserPayloadSchema);

const LoginPage: Component = () => {
    useUnauthGuard();

    const navigate = useNavigate();

    const onSubmit = async (values: LoginUserPayload) => {
        await login(values);
        navigate('/app/dashboard');
    };

    return (
        <>
            <Helmet title="Login - PixieFit" />
            <main class={styles.main}>
                <div class={styles.formContainer}>
                    <Logo theme="dark" />
                    <div class={styles.formHeading}>
                        <h1>Welcome back!</h1>
                        <h2>Log in to your account</h2>
                    </div>
                    <Zorm.Provider>
                        <Zorm.Alert />
                        <Zorm.Form class={styles.form} onSubmit={onSubmit}>
                            <Zorm.Input
                                label="Email Address"
                                name="email"
                                placeholder="Email address"
                            />
                            <Zorm.Input
                                label="Password"
                                type="password"
                                name="password"
                                placeholder="Password"
                            />
                            <Zorm.Submit label="Sign In" />
                        </Zorm.Form>
                    </Zorm.Provider>
                    <div class={styles.formFooter}>
                        <span>Don&apos;t have an account?&nbsp;</span>
                        <RouterLink href="/register" label="Sign up here." />
                    </div>
                </div>
            </main>
        </>
    );
};

export default LoginPage;
