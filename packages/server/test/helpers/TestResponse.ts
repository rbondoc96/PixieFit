/* eslint-disable @typescript-eslint/no-unused-expressions */
import {expect} from 'chai';
import {Response} from 'supertest';

export default class TestResponse {
    private readonly headers: Record<string, string>;
    private readonly response: Response;
    public readonly status: number;

    constructor(response: Response) {
        this.response = response;
        this.headers = response.headers;
        this.status = response.status;
    }

    public get baseResponse(): Response {
        return this.response;
    }

    assertOk(): void {
        expect(this.response.status).to.equal(200);
    }

    assertCreated(): void {
        expect(this.response.status).to.equal(201);
    }

    assertNoContent(): void {
        expect(this.response.status).to.equal(204);
    }

    assertRedirect(redirectTarget: string): void {
        expect(this.response.status).to.equal(302);
        expect(this.headers.location).to.equal(redirectTarget);
    }

    assertBadRequest(): void {
        expect(this.response.status).to.equal(400);
    }

    assertUnauthenticated(): void {
        expect(this.response.status).to.equal(401);
    }

    assertUnauthorized(): void {
        expect(this.response.status).to.equal(403);
    }

    assertNotFound(): void {
        expect(this.response.status).to.equal(404);
    }

    assertUnprocessable(): void {
        expect(this.response.status).to.equal(422);
    }

    assertServerError(): void {
        expect(this.response.status).to.equal(500);
    }
}
