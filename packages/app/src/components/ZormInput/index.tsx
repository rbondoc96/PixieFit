import {faEye, faEyeSlash} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {createMemo, createSignal, type JSX, mergeProps, Show} from 'solid-js';
import {type TypeOf, type ZodType} from 'zod';

import styles from './styles.module.scss';

type ZormInputType = 'date' | 'password' | 'text';

type ZormInputProps<TValidator extends ZodType> = {
    initialValue?: TypeOf<TValidator>;
    name: string;
    placeholder: string;
    type?: ZormInputType;
    validator: TValidator;
};

const ZormInput = <TValidator extends ZodType>(
    baseProps: ZormInputProps<TValidator>,
): JSX.Element => {
    const props = mergeProps(
        {
            type: 'text',
        },
        baseProps,
    );

    let inputRef: HTMLInputElement | undefined;
    const [error, setError] = createSignal<string | undefined>();
    const [touched, setTouched] = createSignal(false);
    const [value, setValue] = createSignal<string | undefined>(props.initialValue);

    const [showPassword, setShowPassword] = createSignal(false);
    const inputType = createMemo(() =>
        props.type === 'password' && showPassword() ? 'text' : props.type,
    );

    const validate = (inputValue: string | undefined) => {
        if (props.validator === undefined) {
            return;
        }

        const parserOutput = props.validator.safeParse(inputValue);
        if (parserOutput.success) {
            setError(undefined);
        } else {
            setError(parserOutput.error.errors.map(error => error.message).join('\n'));
        }
    };

    const handleSetTouched = (wasTouched: boolean, shouldValidate?: boolean) => {
        setTouched(wasTouched);

        if (!shouldValidate || shouldValidate === undefined) {
            return;
        }

        validate(value());
    };

    const handleSetValue = (targetValue: string | undefined, shouldValidate?: boolean) => {
        setValue(targetValue ?? '');
        setTouched(true);

        if (!shouldValidate || shouldValidate === undefined) {
            return;
        }

        validate(targetValue);
    };

    const onBlur = () => {
        handleSetTouched(true, props.validator !== undefined);
    };

    const onInput: JSX.IntrinsicElements['input']['onInput'] = event => {
        const {value} = event.currentTarget;
        handleSetValue(value === '' ? undefined : value, props.validator !== undefined);
    };

    return (
        <div class={styles.zormInput}>
            <label for={props.name}>
                <input
                    ref={inputRef}
                    type={inputType()}
                    id={props.name}
                    name={props.name}
                    placeholder={props.placeholder}
                    value={value() ?? ''}
                    onBlur={onBlur}
                    onInput={onInput}
                />
                <Show when={props.type === 'password'}>
                    <button
                        type="button"
                        tabIndex={-1}
                        onClick={() => setShowPassword(prevState => !prevState)}
                    >
                        <Show fallback={<FontAwesomeIcon icon={faEye} />} when={showPassword()}>
                            <FontAwesomeIcon icon={faEyeSlash} />
                        </Show>
                    </button>
                </Show>
            </label>
            <Show when={error() !== undefined && touched()}>
                <div class={styles.zormInputError}>{error()}</div>
            </Show>
        </div>
    );
};

export default ZormInput;
