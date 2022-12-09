import supertest, {SuperTest, Test} from 'supertest';

import Server from '@/core/Server';
import TestResponse from './TestResponse';

class TestRequest {
    private readonly request: SuperTest<Test>;

    constructor() {
        this.request = supertest(new Server().start());
    }

    async post(url: string, body?: string | object) {
        const response = await this.request.post(url).send(body);

        return new TestResponse(response);
    }

    async postForm(url: string, body: string | Record<string, string>) {
        const response = await this.request
            .post(url)
            .set('Content-Type', 'application/x-www-form-urlencoded')
            .send(new URLSearchParams(body).toString());

        return new TestResponse(response);
    }

    async postJSON(url: string, body?: object) {
        const response = await this.request.post(url).send(body);

        return new TestResponse(response);
    }
}

export default new TestRequest();
