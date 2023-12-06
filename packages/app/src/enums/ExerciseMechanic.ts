const ExerciseMechanic = {
    Compound: 'compound',
    Isolation: 'isolation',
} as const;

type ExerciseMechanic = typeof ExerciseMechanic[keyof typeof ExerciseMechanic];

export default ExerciseMechanic;
