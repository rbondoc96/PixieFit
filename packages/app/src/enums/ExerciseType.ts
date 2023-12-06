const ExerciseType = {
    Activation: 'activation',
    Class: 'class',
    Endurance: 'endurance',
    Strength: 'strength',
    Stretch: 'stretch',
} as const;

type ExerciseType = typeof ExerciseType[keyof typeof ExerciseType];

export default ExerciseType;
