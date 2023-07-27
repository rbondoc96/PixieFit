export default abstract class Throwable extends Error {
    public abstract override readonly name: string;

    constructor(public readonly displayName: string, public readonly message: string) {
        super(message);
    }
}
