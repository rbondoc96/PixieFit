import {boolean, number, object, type output, string, type ZodType} from 'zod';

function createListResponseParser<TParser extends ZodType>(parser: TParser): ZodType {
    return object({
        count: number(),
        next: string().nullable(),
        previous: string().nullable(),
        results: parser.array(),
    });
}

export const muscleParser = object({
    id: number(),
    name: string(),
    name_en: string().nullable(),
    is_front: boolean(),
    image_url_main: string().nullable(),
    image_url_secondary: string().nullable(),
}).transform(data => ({
    id: data.id,
    name: data.name,
    simple_name: data.name_en,
    is_front: data.is_front,
    image_url_main: data.image_url_main,
    image_url_secondary: data.image_url_secondary,
}));

export type Muscle = output<typeof muscleParser>;
export const muscleListParser = createListResponseParser(muscleParser);
