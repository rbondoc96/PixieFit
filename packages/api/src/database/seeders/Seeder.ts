export default abstract class Seeder {
    public abstract run(): Promise<void>;
}
