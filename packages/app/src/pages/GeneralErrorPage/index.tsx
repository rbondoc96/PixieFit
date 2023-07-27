import {type Component} from 'solid-js';

type GeneralErrorPageProps = {
    error: unknown;
    reset: () => void;
};

const GeneralErrorPage: Component<GeneralErrorPageProps> = props => {
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
