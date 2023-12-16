import {
    faExclamationCircle,
    faXmarkCircle,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import clsx from 'clsx';
import FontAwesomeIcon from 'solid-fa';
import {type Component, createSignal, For, Show} from 'solid-js';

import RequestException from '@/exceptions/RequestException';
import Throwable from '@/exceptions/Throwable';
import UnexpectedError from '@/exceptions/UnexpectedError';

import styles from './styles.module.scss';

type ZormAlertType = 'error' | 'info' | 'success' | 'warning';

type ZormAlertProps = {
    message: string;
    messages?: string[];
    show: boolean;
    title: string;
    type: ZormAlertType;
};

export type ZormAlertContent = Pick<ZormAlertProps, 'message' | 'messages' | 'title'>;

const AlertStyles: Record<
    ZormAlertType,
    {
        icon: IconDefinition;
        componentStyle: string;
    }
> = {
    error: {
        icon: faExclamationCircle,
        componentStyle: clsx(styles.zormAlert, styles.zormAlertError),
    },
    info: {
        icon: faExclamationCircle,
        componentStyle: clsx(styles.zormAlert, styles.zormAlertInfo),
    },
    success: {
        icon: faExclamationCircle,
        componentStyle: clsx(styles.zormAlert, styles.zormAlertSuccess),
    },
    warning: {
        icon: faExclamationCircle,
        componentStyle: clsx(styles.zormAlert, styles.zormAlertWarning),
    },
} as const;

const ZormAlert: Component<ZormAlertProps> = props => {
    const [showAlert, setShowAlert] = createSignal(false);

    const alertStyles = () => AlertStyles[props.type];

    const formattedMessages = () => props.messages?.map(message => `${'\u2022'} ${message}`);

    return (
        <Show when={showAlert() || props.show}>
            <div class={alertStyles().componentStyle}>
                <div class={styles.zormAlertIcon}>
                    <FontAwesomeIcon icon={alertStyles().icon} />
                </div>
                <div class={styles.zormAlertContent}>
                    <span class={styles.zormAlertContentTitle}>{props.title}</span>
                    <span class={styles.zormAlertContentMessage}>{props.message}</span>
                    <Show when={formattedMessages() !== undefined}>
                        <div class={styles.zormAlertContentDetails}>
                            <For each={formattedMessages()}>
                                {message => (
                                    <span class={styles.zormAlertContentDetailsMessage}>
                                        {message}
                                    </span>
                                )}
                            </For>
                        </div>
                    </Show>
                </div>
                <button class={styles.zormAlertClearIcon} onClick={() => setShowAlert(false)}>
                    <FontAwesomeIcon icon={faXmarkCircle} />
                </button>
            </div>
        </Show>
    );
};

export function errorToZormAlertContent(error: unknown): ZormAlertContent {
    if (error instanceof RequestException) {
        return {
            message: error.message,
            messages: error.messages,
            title: error.displayName,
        };
    }

    if (error instanceof Throwable) {
        return {
            message: error.message,
            title: error.displayName,
        };
    }

    // prettier-ignore
    const unexpectedError = error instanceof Error
        ? new UnexpectedError(error)
        : new UnexpectedError(new Error(
            typeof error === 'object' ? JSON.stringify(error) : String(error),
        ));

    return {
        message: unexpectedError.message,
        title: unexpectedError.displayName,
    };
}

export default ZormAlert;
