import {nullable, object, type output, string} from 'zod';

export const measurementSchema = object({
    unit: string(),
    denominator: nullable(string()),
    operation: string(),
});

export type Measurement = output<typeof measurementSchema>;
