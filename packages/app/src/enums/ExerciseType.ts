const ExerciseType = {
    Activation: 'activation',
    Class: 'class',
    Endurance: 'endurance',
    Strength: 'strength',
    Stretch: 'stretch',
} as const;

type ExerciseType = typeof ExerciseType[keyof typeof ExerciseType];

export function displayExerciseType(type: ExerciseType): string {
    switch (type) {
        case ExerciseType.Activation:
            return 'Activation';
        case ExerciseType.Class:
            return 'Class';
        case ExerciseType.Endurance:
            return 'Endurance';
        case ExerciseType.Strength:
            return 'Strength';
        case ExerciseType.Stretch:
            return 'Stretch';
    }
}

export default ExerciseType;
