import {type ComponentProps, type ParentComponent, Show} from 'solid-js';
import {Transition as SolidTransition} from 'solid-transition-group';

type SolidTransitionProps = ComponentProps<typeof SolidTransition>;

/**
 * A wrapper component around `<Transition />` from the
 * `solid-transition-group` package and `<Show />` from the core
 * SolidJS package.
 *
 * This wrapper provides an interface similar to the one provided
 * by HeadlessUI's `<Transition />` component.
 *
 * Note: This only supports transitioning a single element at a time.
 */
const Transition: ParentComponent<{
    enter?: SolidTransitionProps['enterActiveClass'];
    enterFrom?: SolidTransitionProps['enterClass'];
    enterTo?: SolidTransitionProps['enterToClass'];
    leave?: SolidTransitionProps['exitActiveClass'];
    leaveFrom?: SolidTransitionProps['exitClass'];
    leaveTo?: SolidTransitionProps['exitToClass'];
    show: boolean | null | undefined;
}> = props => (
    <SolidTransition
        enterActiveClass={props.enter}
        enterClass={props.enterFrom}
        enterToClass={props.enterTo}
        exitActiveClass={props.leave}
        exitClass={props.leaveFrom}
        exitToClass={props.leaveTo}
    >
        <Show when={props.show}>
            {props.children}
        </Show>
    </SolidTransition>
);

export default Transition;
