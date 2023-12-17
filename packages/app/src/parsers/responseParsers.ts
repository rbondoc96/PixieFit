import {
    array,
    boolean,
    literal,
    object,
    optional,
    type output,
    record,
    string,
    type ZodType,
} from 'zod';

export const errorResponseParser = object({
    success: literal(false),
    message: string().optional(),
    errors: record(string().array()).optional(),
    _error: string().optional(),
    _stack: string().optional(),
});

export type ErrorResponse = output<typeof errorResponseParser>;

export function createGetResponseParser<TParser extends ZodType>(parser: TParser): ZodType {
    return object({
        success: boolean(),
        data: parser,
        message: optional(string()),
    });
}

export function createListResponseParser<TParser extends ZodType>(parser: TParser): ZodType {
    return object({
        success: boolean(),
        data: array(parser),
        message: optional(string()),
    });
}
