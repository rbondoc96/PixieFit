export type UserResource = {
    email: string;
    name: {
        first: string;
        last: string;
        full: string;
    };
    created_at: Date;
    updated_at: Date;
};
