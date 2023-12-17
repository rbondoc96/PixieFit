import {
    type Component,
    type ComponentProps,
    ErrorBoundary,
    type ParentComponent,
    Suspense,
} from 'solid-js';

const SuspensefulErrorBoundary: ParentComponent<{
    error: ComponentProps<typeof ErrorBoundary>['fallback'];
    loading: Component;
}> = props => (
    <Suspense fallback={props.loading({})}>
        <ErrorBoundary fallback={props.error}>
            {props.children}
        </ErrorBoundary>
    </Suspense>
);

export default SuspensefulErrorBoundary;
