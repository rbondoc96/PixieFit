import type {JSX} from 'solid-js';

export type ClickEventHandler<TElement> = JSX.EventHandler<TElement, MouseEvent>;
export type ClickEvent<TElement> = Parameters<ClickEventHandler<TElement>>[0];
