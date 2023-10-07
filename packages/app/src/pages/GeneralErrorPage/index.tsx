import {type Component} from 'solid-js';

const GeneralErrorPage: Component<{
    error: unknown;
    reset: () => void;
}> = props => {
    return (
        <div>
            <h1>General Error Page</h1>
            <span>Please contact support or try again later.</span>
            <div>
                <p>Description of Error:</p>
                <pre>{JSON.stringify(props.error, null, 2)}</pre>
            </div>
        </div>
    );
};

export default GeneralErrorPage;
