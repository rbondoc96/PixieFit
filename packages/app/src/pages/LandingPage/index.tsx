import {A as Link} from '@solidjs/router';
import {type Component} from 'solid-js';

import Button from '@/components/Button';
import Logo from '@/components/Logo';
import RouterLink from '@/components/RouterLink';
import Routes from '@/constants/Routes';

import styles from './styles.module.scss';

const LandingPage: Component = () => {
    return (
        <>
            <header class={styles.header}>
                <div class={styles.headerContainer}>
                    <Logo />
                    <div class={styles.headerLinks}>
                        <nav>
                            <RouterLink label="Features" href="#features" />
                            <RouterLink label="About" href="#about" />
                            <RouterLink label="FAQ" href="#faq" />
                            <RouterLink label="Contact" href="#contact" />
                        </nav>
                        <Button as={Link} label="Sign In" href={Routes.Login.fullPath} />
                    </div>
                </div>
            </header>
            <main class={styles.main}>
                <div class={styles.hero}>
                    <div class={styles.heroContent}>
                        <div class={styles.heroContentTitle}>
                            <h3>An exercise tracker made just for you.</h3>
                            <h1>
                                Maximize your fitness <br />
                                journey.
                            </h1>
                        </div>
                        <Button as={Link} label="Get Started" href={Routes.Register.fullPath} />
                        <h2>Free. Forever.</h2>
                    </div>
                </div>
            </main>
        </>
    );
};

export default LandingPage;
