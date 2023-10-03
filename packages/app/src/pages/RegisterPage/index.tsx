import {useNavigate} from '@solidjs/router';
import {type Component} from 'solid-js';

import {type RegisterUserPayload, registerUserPayloadSchema} from '@/api/auth';
import Helmet from '@/components/Helmet';
import Logo from '@/components/Logo';
import RouterLink from '@/components/RouterLink';
import createZorm from '@/components/Zorm';
import useUnauthGuard from '@/hooks/useUnauthGuard';
import {register} from '@/stores/auth.store';

import styles from './styles.module.scss';

const Zorm = createZorm(registerUserPayloadSchema);

const RegisterPage: Component = () => {
    useUnauthGuard();

    const navigate = useNavigate();

    const onSubmit = async (values: RegisterUserPayload) => {
        await register(values);
        navigate('/app/dashboard');
    };

    return (
        <>
            <Helmet title="Sign up - PixieFit" />
            <main class={styles.main}>
                <div class={styles.formContainer}>
                    <Logo theme="dark" />
                    <div class={styles.formHeading}>
                        <h1>Create an Account</h1>
                    </div>
                    <Zorm.Provider>
                        <Zorm.Alert />
                        <Zorm.Form class={styles.form} onSubmit={onSubmit}>
                            <div class={styles.formInputRow}>
                                <Zorm.Input
                                    label="First Name"
                                    name="first_name"
                                    placeholder="First name"
                                />
                                <Zorm.Input
                                    label="Last Name"
                                    name="last_name"
                                    placeholder="Last name"
                                />
                            </div>

                            <div class={styles.formInputRow}>
                                <Zorm.Input
                                    label="Email Address"
                                    name="email"
                                    placeholder="Email address"
                                />
                            </div>

                            <div class={styles.formInputRow}>
                                <Zorm.Input
                                    label="Birthday"
                                    name="birthday"
                                    placeholder="Birthday"
                                    type="date"
                                />
                                <Zorm.Select
                                    label="Gender"
                                    name="gender"
                                    initialValue="male"
                                    options={[
                                        {label: 'Male', value: 'male'},
                                        {label: 'Female', value: 'female'},
                                        {label: 'Non-Binary', value: 'non_binary'},
                                        {label: 'Other', value: 'other'},
                                    ]}
                                />
                            </div>

                            <div class={styles.formInputRow}>
                                <Zorm.Input
                                    label="Password"
                                    name="password"
                                    placeholder="Password"
                                    type="password"
                                />
                                <Zorm.Input
                                    label="Confirm Password"
                                    name="password_confirm"
                                    placeholder="Confirm password"
                                    type="password"
                                />
                            </div>

                            <Zorm.Submit label="Register" />
                        </Zorm.Form>
                    </Zorm.Provider>

                    <div class={styles.formFooter}>
                        <span>Already have an account?&nbsp;</span>
                        <RouterLink href="/login" label="Log in." />
                    </div>
                </div>
            </main>
        </>
    );
};

export default RegisterPage;
