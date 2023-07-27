import type postgres from 'postgres';

const PostgresErrorCode = {
    UniqueViolation: '23505',
} as const;

type PostgresErrorResolutionResult = {
    message: string;
    status: number;
};

export function resolvePostgresError(error: postgres.PostgresError): PostgresErrorResolutionResult {
    switch (true) {
        case error.code === PostgresErrorCode.UniqueViolation: {
            return {
                message: error.detail + ' ' + error.message,
                status: 422,
            };
        }
        default: {
            return {
                message: error.message,
                status: 500,
            };
        }
    }
}
