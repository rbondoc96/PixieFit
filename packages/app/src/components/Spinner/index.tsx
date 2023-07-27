import {faSpinner} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {type Component, type ComponentProps} from 'solid-js';

import styles from './styles.module.scss';

type FontAwesomeIconProps = ComponentProps<typeof FontAwesomeIcon>;

type SpinnerSize = 'sm' | 'md' | 'lg' | 'xl';

type SpinnerProps = {
    size?: SpinnerSize;
    speedMultiplier?: number;
};

const Spinner: Component<SpinnerProps> = ({size = 'md'}) => {
    let spinnerSize: Extract<FontAwesomeIconProps['size'], 'sm' | 'lg' | '1x' | '2x'>;

    switch (size) {
        case 'sm':
            spinnerSize = 'sm';
            break;
        case 'md':
            spinnerSize = 'lg';
            break;
        case 'lg':
            spinnerSize = '1x';
            break;
        case 'xl':
            spinnerSize = '2x';
            break;
    }

    return (
        <span class={styles.spinner}>
            <FontAwesomeIcon class="animate-spin" icon={faSpinner} size={spinnerSize} />
        </span>
    );
};

export default Spinner;
