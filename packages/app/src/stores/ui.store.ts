import {type Accessor, createSignal} from 'solid-js';

/**
 * Uses TailwindCSS default breakpoints + ExtraSmall
 * @see {@link https://tailwindcss.com/docs/screens}
 */
const Breakpoint = {
    ExtraSmall: 480,
    Small: 640,
    Medium: 768,
    Large: 1024,
    ExtraLarge: 1280,
    ExtraExtraLarge: 1536,
} as const;

type ScreenSize = keyof typeof Breakpoint;

export const ViewportState = {
    Mobile: 'mobile',
    MobileToDesktop: 'mobile_to_desktop',
    DesktopToMobile: 'desktop_to_mobile',
    Desktop: 'desktop',
} as const;

export type ViewportState = typeof ViewportState[keyof typeof ViewportState];

const initialScreenSize = resolveWindowWidthToScreenSize();
const [screenSize, setScreenSize] = createSignal<ScreenSize>(initialScreenSize);
const [viewportState, setViewportState] = createSignal<ViewportState>(
    resolveViewportState(initialScreenSize, initialScreenSize),
);
const isMobileView: Accessor<boolean> = () => [
    'ExtraSmall',
    'Small',
].includes(screenSize());

function resolveViewportState(previousSize: ScreenSize, currentSize: ScreenSize): ViewportState {
    if (previousSize === 'Medium' && currentSize === 'Large') {
        return ViewportState.MobileToDesktop;
    }

    if (previousSize === 'Large' && currentSize === 'Medium') {
        return ViewportState.DesktopToMobile;
    }

    if (['Large', 'ExtraLarge', 'ExtraExtraLarge'].includes(currentSize)) {
        return ViewportState.Desktop;
    }

    return ViewportState.Mobile;
}

function resolveWindowWidthToScreenSize(): ScreenSize {
    const size = Object.entries(Breakpoint)
        .find(([_key, value]) => window.innerWidth <= value);

    return size !== undefined
        ? size[0] as ScreenSize
        : 'ExtraExtraLarge';
}

function onWindowResize(): void {
    const previousScreenSize = screenSize();
    const currentScreenSize = resolveWindowWidthToScreenSize();

    setScreenSize(currentScreenSize);
    setViewportState(resolveViewportState(previousScreenSize, currentScreenSize));
}

export const useIsMobileView = (): Accessor<boolean> => isMobileView;
export const useScreenSize = (): Accessor<ScreenSize> => screenSize;
export const useViewportState = (): Accessor<ViewportState> => viewportState;
export const useOnWindowResize = (): () => void => onWindowResize;
