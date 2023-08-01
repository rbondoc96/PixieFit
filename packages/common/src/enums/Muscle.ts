export const Muscle = {
    Abs: 'abs',
    Biceps: 'biceps',
    Brachialis: 'brachialis',
    Calves: 'calves',
    DeltoidAnterior: 'deltoid_anterior',
    Forearms: 'forearms',
    Glutes: 'glutes',
    Hamstrings: 'hamstrings',
    Lats: 'lats',
    Quadriceps: 'quadriceps',
    Triceps: 'triceps',
} as const;

type Muscle = (typeof Muscle)[keyof typeof Muscle];

export function displayMuscle(muscle: Muscle): string {
    switch (muscle) {
        case Muscle.Abs:
            return 'Abs';
        case Muscle.Biceps:
            return 'Biceps';
        case Muscle.Brachialis:
            return 'Brachialis';
        case Muscle.Calves:
            return 'Calves';
        case Muscle.DeltoidAnterior:
            return 'Deltoid (Anterior)';
        case Muscle.Forearms:
            return 'Forearms';
        case Muscle.Glutes:
            return 'Glutes';
        case Muscle.Hamstrings:
            return 'Hamstrings';
        case Muscle.Lats:
            return 'Lats';
        case Muscle.Quadriceps:
            return 'Quadriceps';
        case Muscle.Triceps:
            return 'Triceps';
    }
}
