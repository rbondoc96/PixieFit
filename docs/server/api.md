# API

## 📜 Table of Contents
- [API Routes](#routes)
    - [User Routes](#user-routes)
- [API Response Structure](#response-structure)


## 🌐 API Routes
<a id="routes"></a>

### 🙋🏽 User Authentication Routes
<a id="user-routes"></a>

<table>
    <tr>
        <th>Endpoint</th>
        <th>HTTP Method</th>
        <th>Description</th>
    </tr>
    <tr>
        <td><code>/auth/register</code></td>
        <td>POST</td>
        <td>Create a new user in DB</td>
    </tr>
    <tr>
        <td><code>/auth/login</code></td>
        <td>POST</td>
        <td>Authenticate user</td>
    </tr>
    <tr>
        <td><code>/auth/logout</code></td>
        <td>GET</td>
        <td>Log a user out</td>
    </tr>
</table>


## API Response Structure
<a id="response-structure"></a>

```ts
interface ApiResponse {
    /**
     *  `true` for ApiDataResponse type
     *  `false` for ApiErrorResponse type
     */
    success: boolean;
    /**
     *  `httpStatus` is only present in ApiErrorResponse type
     */
    httpStatus?: number;
    /**
     *  `data` is only present in ApiDataResponse type
     */
    data?: Record<string, unknown | unknown[]> | null;
    /**
     *  `error` is only present in ApiErrorResponse type
     */
    error?: {
        name: string;
        message: string;
        data?: Record<string, unknown> | string;  
    };
}
```

### Example Responses
**GET `/posts`**
```json
{
    "success": true,
    "data": {
        "posts": [
            {"id": 1, "title": "Title1", "body": "Body1"},
            {"id": 2, "title": "Title2", "body": "Body2"},
        ]
    }
}
```

**GET `/posts/2`**
```json
{
    "success": true,
    "data": {
        "post": {"id": 2, "title": "Title2", "body": "Body2"}
    }
}
```

**DELETE `/posts/2`**
```json
{
    "success": true,
    "data": null
}
```

**POST `/posts` - Failed to create a Post**
```json
{
    "success": false,
    "httpStatus": 422,
    "error": {
        "name": "ValidationError",
        "data": {
            "title": "This field is required.",
            "author": "This field is required.",
        }
    },
}
```

**GET `/posts` - Server error**
```json5
{
    "success": false,
    "httpStatus": 500,
    "error": {
        "name": "ConnectionException",
        "message": "Unable to communicate with database.",
        "data": {
            "errorName": "",
            "errorMessage": "",
            "errorStack": "",
        },
    }
}
```
