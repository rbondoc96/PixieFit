import type {JSX} from 'solid-js';

type LinkIcon = {
    rel: 'icon';
    type: 'image/ico';
    href: `${string}.ico`;
};

type LinkPreload = {
    rel: 'preload';
    as: JSX.IntrinsicElements['link']['as'];
    type: `text/${'css' | 'html' | 'js'}`;
};

type LinkStyleSheet = {
    rel: 'stylesheet';
    href: string;
    title?: string;
};

export type LinkAttribute =
    | LinkIcon
    | LinkPreload
    | LinkStyleSheet
    | {
          rel: string;
      };

type MetaAuthor = {
    name: 'author';
    content: string;
};

type MetaDescription = {
    name: 'description';
    content: string;
};

type MetaImage = {
    name: 'image';
    content: string;
};

type MetaOpenGraph = {
    property: `og:${'description' | 'image' | 'title' | 'type' | 'url'}`;
    content: string;
};

type MetaTwitter = {
    name: `twitter:${'card' | 'creator' | 'description' | 'image' | 'title'}`;
    content: string;
};

export type MetaAttribute =
    | MetaAuthor
    | MetaDescription
    | MetaImage
    | MetaOpenGraph
    | MetaTwitter
    | {
          name?: string;
          property?: string;
          content: string;
      };
