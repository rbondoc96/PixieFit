import {faSpinner} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {type Accessor, type Component, type ComponentProps, createMemo, mergeProps} from 'solid-js';

import styles from './styles.module.scss';

type FontAwesomeIconProps = ComponentProps<typeof FontAwesomeIcon>;

type SpinnerColor = 'primary' | 'white';

type SpinnerSize = 'sm' | 'md' | 'lg' | 'xl';

type SpinnerProps = {
    color?: SpinnerColor;
    size?: SpinnerSize;
    speedMultiplier?: number;
};

const Spinner: Component<SpinnerProps> = baseProps => {
    const props = mergeProps({
        color: 'white',
        size: 'md',
    }, baseProps);

    const style = createMemo(
        () => {
            if (props.color === 'white')       {
                return styles.spinnerWhite;
            }

            if (props.color === 'primary') {
                return styles.spinnerPrimary;
            }
        },
    );

    const size: Accessor<Extract<
        FontAwesomeIconProps['size'],
        'sm' | 'lg' | '1x' | '2x'>
    > = createMemo(
        () => {
            if (props.size === 'sm') {
                return 'sm';
            }

            if (props.size === 'md') {
                return 'lg';
            }

            if (props.size === 'lg') {
                return '1x';
            }

            if (props.size === 'xl') {
                return '2x';
            }

            return 'lg';
        },
    );

    return (
        <span class={style()}>
            <FontAwesomeIcon class="animate-spin" icon={faSpinner} size={size()} />
        </span>
    );
};

export default Spinner;
