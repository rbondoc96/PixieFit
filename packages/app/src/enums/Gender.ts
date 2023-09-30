const Gender = {
    Female: 'female',
    Male: 'male',
    NonBinary: 'non_binary',
    Other: 'other',
} as const;

type Gender = typeof Gender[keyof typeof Gender];

export default Gender;
