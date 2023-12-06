import {type Accessor, createSignal} from 'solid-js';

/**
 * Uses TailwindCSS default breakpoints + ExtraSmall
 * @see {@link https://tailwindcss.com/docs/screens}
 */
const Screens = {
    ExtraSmall: 480,
    Small: 640,
    Medium: 768,
    Large: 1024,
    ExtraLarge: 1280,
    ExtraExtraLarge: 1536,
} as const;

type ScreenSize = keyof typeof Screens;

const [screenSize, setScreenSize] = createSignal<ScreenSize>(resolveScreenSize());

function resolveScreenSize(): ScreenSize {
    const size = Object.entries(Screens)
        .find(([_key, value]) => window.innerWidth <= value);

    return size !== undefined
        ? size[0] as ScreenSize
        : 'ExtraExtraLarge';
}

export const updateScreenSize = (): void => {
    setScreenSize(resolveScreenSize());
};

export const useScreenSize = (): Accessor<ScreenSize> => screenSize;
