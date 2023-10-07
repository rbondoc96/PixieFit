export default async function delay(durationMs: number): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, durationMs));
}
