const ExerciseForce = {
    Hold: 'hold',
    Pull: 'pull',
    Push: 'push',
} as const;

type ExerciseForce = typeof ExerciseForce[keyof typeof ExerciseForce];

export default ExerciseForce;
