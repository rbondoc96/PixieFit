export default abstract class Factory<Properties, Document> {
    public abstract create(overrides: Partial<Properties>): Promise<Document>;
    public abstract createMany(
        count: number,
        overrides: Partial<Properties>,
    ): Promise<Document[]>;
}
