const Role = {
    Admin: 'admin',
    User: 'user',
} as const;

type Role = typeof Role[keyof typeof Role];

export default Role;
