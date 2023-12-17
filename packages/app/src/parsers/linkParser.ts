import {nativeEnum, nullable, object, type output, string} from 'zod';

import LinkFormat from '@/enums/LinkFormat';
import LinkType from '@/enums/LinkType';

const simpleLinkSchema = object({
    id: string(),
    type: nativeEnum(LinkType),
    format: nativeEnum(LinkFormat),
    label: string(),
    description: nullable(string()),
    src: string(),
});

export type SimpleLink = output<typeof simpleLinkSchema>;
