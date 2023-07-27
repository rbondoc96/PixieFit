export abstract class RequestException extends Error {
    public abstract override readonly name: string;
    public readonly status: number;

    constructor(message: string, status: number) {
        super(message);
        this.status = status;
    }

    public jsonResponse(): JSONResponse {
        return {
            success: false,
            message: this.message,
        };
    }
}

export default RequestException;
