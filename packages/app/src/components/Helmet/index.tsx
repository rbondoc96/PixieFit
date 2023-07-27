import {Link, Meta, Title} from '@solidjs/meta';
import {type Component, Index, mergeProps, Show} from 'solid-js';

import type {LinkAttribute, MetaAttribute} from '@/components/Helmet/types';

type HelmetProps = {
    title?: string;
    links?: LinkAttribute[];
    meta?: MetaAttribute[];
};

const Helmet: Component<HelmetProps> = baseProps => {
    const props = mergeProps(
        {
            links: [],
            meta: [],
        },
        baseProps,
    );

    return (
        <>
            <Show when={props.title} keyed>
                {title => <Title>{title}</Title>}
            </Show>
            <Index each={props.links}>{link => <Link {...link()} />}</Index>
            <Index each={props.meta}>{meta => <Meta {...meta()} />}</Index>
        </>
    );
};

export default Helmet;
