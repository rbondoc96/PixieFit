/**
 * A utility function that asserts if a value's type matches the given predicate.
 *
 * - If it matches, it returns the value with its narrowed, correct type.
 * - Otherwise, it returns `false`.
 */
export default function matches<TValueType extends TValue, TValue = unknown>(
    value: TValue,
    predicate: (value: TValue) => value is TValueType,
): TValueType | false {
    return predicate(value) ? value : false;
}
