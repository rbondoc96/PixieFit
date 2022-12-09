/* eslint-disable @typescript-eslint/no-unused-expressions */
import {expect} from 'chai';
import {Response} from 'supertest';

import Http from '@/core/enums/Http';

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
        expect(this.response.status).to.equal(Http.OK);
    }

    assertCreated(): void {
        expect(this.response.status).to.equal(Http.CREATED);
    }

    assertNoContent(): void {
        expect(this.response.status).to.equal(Http.NO_CONTENT);
    }

    assertRedirect(redirectTarget: string): void {
        expect(this.response.status).to.equal(Http.MOVED_TEMPORARILY);
        expect(this.headers.location).to.equal(redirectTarget);
    }

    assertBadRequest(): void {
        expect(this.response.status).to.equal(Http.BAD_REQUEST);
    }

    assertUnauthenticated(): void {
        expect(this.response.status).to.equal(Http.UNAUTHORIZED);
    }

    assertUnauthorized(): void {
        expect(this.response.status).to.equal(Http.FORBIDDEN);
    }

    assertNotFound(): void {
        expect(this.response.status).to.equal(Http.NOT_FOUND);
    }

    assertUnprocessable(): void {
        expect(this.response.status).to.equal(Http.UNPROCESSABLE_ENTITY);
    }

    assertServerError(): void {
        expect(this.response.status).to.equal(Http.INTERNAL_SERVER_ERROR);
    }
}
