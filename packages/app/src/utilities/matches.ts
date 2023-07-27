export default function matches<TValueType extends TValue, TValue = unknown>(
    value: TValue,
    predicate: (value: TValue) => value is TValueType,
): TValueType | false {
    return predicate(value) ? value : false;
}
