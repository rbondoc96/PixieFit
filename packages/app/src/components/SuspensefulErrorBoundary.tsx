import {type ComponentProps, ErrorBoundary, type ParentComponent, Suspense} from 'solid-js';

const SuspensefulErrorBoundary: ParentComponent<{
    error: ComponentProps<typeof ErrorBoundary>['fallback'];
    loading: ComponentProps<typeof Suspense>['fallback'];
}> = props => (
    <Suspense fallback={props.loading}>
        <ErrorBoundary fallback={props.error}>
            {props.children}
        </ErrorBoundary>
    </Suspense>
);

export default SuspensefulErrorBoundary;
