import {faEye, faEyeSlash} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {
    type Accessor,
    type Component,
    createContext,
    createMemo,
    createSignal,
    type JSX,
    mergeProps,
    type ParentComponent,
    type Setter,
    Show,
    splitProps,
    useContext,
} from 'solid-js';
import {createStore, type SetStoreFunction} from 'solid-js/store';
import {type TypeOf, type ZodObject, type ZodRecord, type ZodString} from 'zod';

import Button from '@/components/Button';
import ZormAlert, {
    errorToZormAlertContent,
    type ZormAlertContent,
} from '@/components/Zorm/ZormAlert';

import styles from './styles.module.scss';

type ZormSchema = ZodObject<TypeOf<ZodRecord<ZodString>>>;
type ZormValues<TSchema extends ZormSchema> = TypeOf<TSchema>;
type ZormErrors<TSchema extends ZormSchema> = Record<
    keyof ZormValues<TSchema>,
    string | string[] | undefined
>;

type ZormFormProps<TSchema extends ZormSchema> = Omit<
    JSX.IntrinsicElements['form'],
    'novalidate' | 'onSubmit'
> & {
    initialValues?: ZormValues<TSchema>;
    onSubmit: (values: ZormValues<TSchema>) => void | Promise<void>;
};

type ZormInputProps<TSchema extends ZormSchema> = Omit<
    JSX.IntrinsicElements['input'],
    'id' | 'name' | 'placeholder' | 'type' | 'value' | 'onBlur' | 'onChange' | 'onInput'
> & {
    label?: string;
    name: keyof ZormValues<TSchema>;
    placeholder?: string;
    type?: 'date' | 'password' | 'text';
};

type ZormSubmitProps = {
    class?: string;
    label: string;
};

type ZormContext<TSchema extends ZormSchema> = {
    errors: ZormErrors<TSchema>;
    formError: Accessor<ZormAlertContent | undefined>;
    isFormValid: Accessor<boolean>;
    isSubmitting: Accessor<boolean>;
    isValidating: Accessor<boolean>;
    onInputBlur: JSX.IntrinsicElements['input']['onBlur'];
    onInputChange: JSX.IntrinsicElements['input']['onInput'];
    setErrors: SetStoreFunction<ZormErrors<TSchema>>;
    setFormError: Setter<ZormAlertContent | undefined>;
    setIsSubmitting: Setter<boolean>;
    touched: Accessor<boolean>;
    validateForm: () => void;
    values: ZormValues<TSchema>;
};

type Zorm<TSchema extends ZormSchema> = {
    Alert: Component;
    Form: ParentComponent<ZormFormProps<TSchema>>;
    Input: Component<ZormInputProps<TSchema>>;
    Provider: ParentComponent;
    Submit: Component<ZormSubmitProps>;
};

export default function createZorm<TSchema extends ZormSchema>(schema: TSchema): Zorm<TSchema> {
    const Context = createContext<ZormContext<TSchema>>({} as ZormContext<TSchema>);

    const Provider: ParentComponent = props => {
        const [errors, setErrors] = createStore<ZormErrors<TSchema>>({} as ZormErrors<TSchema>);
        const [formError, setFormError] = createSignal<ZormAlertContent>();
        const [isFormValid, setIsFormValid] = createSignal(false);
        const [isSubmitting, setIsSubmitting] = createSignal(false);
        const [isValidating, setIsValidating] = createSignal(false);
        const [touched, setTouched] = createSignal(false);
        const [values, setValues] = createStore<ZormValues<TSchema>>({} as ZormValues<TSchema>);

        const validateForm = () => {
            setErrors({} as ZormErrors<TSchema>);
            setFormError(undefined);
            setIsValidating(true);

            const parserOutput = schema.safeParse(values);
            if (!parserOutput.success) {
                const errors = parserOutput.error.errors;
                errors.forEach(error => {
                    const name = error.path[0] as keyof ZormErrors<TSchema>;
                    setFieldError(name, error.message);
                });
            }

            setIsValidating(false);
            setIsFormValid(parserOutput.success);
        };

        const validateField = <TKey extends keyof ZormErrors<TSchema>>(
            name: TKey,
            value?: ZormValues<TSchema>[TKey],
        ) => {
            // We know that `name` must be a field in the Zod schema
            // So we can safely grab the field parser
            const fieldParser = schema.shape[name as string];

            const fieldValue = value ?? values[name];
            const parserOutput = fieldParser.safeParse(fieldValue);

            if (parserOutput.success) {
                setFieldError(name, undefined);
            } else {
                setFieldError(name, parserOutput.error.errors[0].message);
            }
        };

        const setFieldError = <TKey extends keyof ZormErrors<TSchema>>(
            name: TKey,
            error: ZormErrors<TSchema>[TKey],
        ) => {
            setErrors({[name]: error} as ZormErrors<TSchema>);
        };

        const setFieldValue = <TKey extends keyof ZormValues<TSchema>>(
            name: TKey,
            value: ZormValues<TSchema>[TKey],
        ) => {
            setValues({[name]: value} as ZormValues<TSchema>);
            validateField(name, value);
        };

        const onInputBlur: JSX.IntrinsicElements['input']['onBlur'] = event => {
            const {name} = event.currentTarget;
            setTouched(true);
            validateField(name as keyof ZormValues<TSchema>);
        };

        const onInputChange: JSX.IntrinsicElements['input']['onInput'] = event => {
            const {name, value} = event.currentTarget;
            setFieldValue(
                name as keyof ZormValues<TSchema>,
                (value === ''
                    ? undefined
                    : value) as ZormValues<TSchema>[keyof ZormValues<TSchema>],
            );
        };

        return (
            <Context.Provider
                value={{
                    errors,
                    formError,
                    isFormValid,
                    isSubmitting,
                    isValidating,
                    onInputBlur,
                    onInputChange,
                    setErrors,
                    setFormError,
                    setIsSubmitting,
                    touched,
                    validateForm,
                    values,
                }}
            >
                {props.children}
            </Context.Provider>
        );
    };

    const Form: ParentComponent<ZormFormProps<TSchema>> = props => {
        const [split, rest] = splitProps(props, ['onSubmit']);

        const zormContext = useContext(Context);

        if (zormContext === undefined) {
            throw new Error('Zorm.Form must be used within a Zorm.Provider');
        }

        const handleSubmit = async (event: SubmitEvent) => {
            event.preventDefault();

            zormContext.validateForm();

            if (!zormContext.isFormValid()) {
                return;
            }

            zormContext.setIsSubmitting(true);

            try {
                await split.onSubmit(zormContext.values);
            } catch (error) {
                zormContext.setFormError(errorToZormAlertContent(error));
            } finally {
                zormContext.setIsSubmitting(false);
            }
        };

        return (
            <form novalidate onSubmit={handleSubmit} {...rest}>
                {props.children}
            </form>
        );
    };

    const Input: Component<ZormInputProps<TSchema>> = baseProps => {
        const props = mergeProps(
            {
                type: 'text',
            },
            baseProps,
        );
        const [split, rest] = splitProps(props, ['label', 'name', 'placeholder', 'type']);

        const zormContext = useContext(Context);

        if (zormContext === undefined) {
            throw new Error('Zorm.Input must be used within a Zorm.Provider');
        }

        const error = createMemo(() => zormContext.errors[split.name as keyof ZormErrors<TSchema>]);
        const value = createMemo(() => zormContext.values[split.name as keyof ZormValues<TSchema>]);

        const [showPassword, setShowPassword] = createSignal(false);
        const inputType = createMemo(() =>
            split.type === 'password' && showPassword() ? 'text' : split.type,
        );

        return (
            <div class={styles.zormInput}>
                <label for={split.name as string}>
                    <Show when={split.label} keyed>
                        {label => <span>{label}</span>}
                    </Show>
                    <div class={styles.zormInputInputContainer}>
                        <input
                            disabled={zormContext.isSubmitting()}
                            type={inputType()}
                            // We can assert this is a string since ZormSchemas
                            // only allow string keys
                            id={split.name as string}
                            name={split.name as string}
                            placeholder={split.placeholder}
                            // Prevents "undefined" from being displayed in the input box
                            // if value() is `undefined`
                            value={value() ?? ''}
                            onBlur={zormContext.onInputBlur}
                            onInput={zormContext.onInputChange}
                            {...rest}
                        />
                        <Show when={split.type === 'password'}>
                            <button
                                type="button"
                                tabIndex={-1}
                                onClick={event => {
                                    // Prevents the input from being focused
                                    // when toggling password visibility
                                    event.preventDefault();
                                    setShowPassword(prevState => !prevState);
                                }}
                            >
                                <Show
                                    when={showPassword()}
                                    fallback={<FontAwesomeIcon icon={faEye} />}
                                >
                                    <FontAwesomeIcon icon={faEyeSlash} />
                                </Show>
                            </button>
                        </Show>
                    </div>
                </label>
                <Show when={error() !== undefined}>
                    <div class={styles.zormInputError}>{error()}</div>
                </Show>
            </div>
        );
    };

    const Submit: Component<ZormSubmitProps> = props => {
        const zormContext = useContext(Context);

        if (zormContext === undefined) {
            throw new Error('Zorm.Submit must be used within a Zorm.Provider');
        }

        const isButtonDisabled = createMemo(
            () => zormContext.isSubmitting() || zormContext.isValidating(),
        );

        return (
            <Button
                type="submit"
                disabled={isButtonDisabled()}
                isLoading={zormContext.isSubmitting()}
                label={props.label}
            />
        );
    };

    const Alert: Component = () => {
        const zormContext = useContext(Context);

        if (zormContext === undefined) {
            throw new Error('Zorm.Alert must be used within a Zorm.Provider');
        }

        return (
            <Show when={zormContext.formError()} keyed>
                {formError => <ZormAlert show type="error" {...formError} />}
            </Show>
        );
    };

    return {
        Alert,
        Form,
        Input,
        Provider,
        Submit,
    };
}
