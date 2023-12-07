const ExerciseForce = {
    Hold: 'hold',
    Pull: 'pull',
    Push: 'push',
} as const;

type ExerciseForce = typeof ExerciseForce[keyof typeof ExerciseForce];

export function displayExerciseForce(force: ExerciseForce): string {
    switch (force) {
        case ExerciseForce.Hold:
            return 'Hold';
        case ExerciseForce.Pull:
            return 'Pull';
        case ExerciseForce.Push:
            return 'Push';
    }
}

export default ExerciseForce;
