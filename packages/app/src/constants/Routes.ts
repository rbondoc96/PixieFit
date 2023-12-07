import Route from '@/lib/Route';

const Routes = {
    ExerciseDetailsPage: Route({
        name: Symbol('exercise-details'),
        pathName: 'exercises/:id',
        parents: ['app'],
    }),
    ExerciseHomePage: Route({name: Symbol('exercises'), pathName: 'exercises', parents: ['app']}),
    ExercisesByMuscleGroupPage: Route({
        name: Symbol('exercises-by-muscle-group'),
        pathName: 'exercises/muscle-groups/:id',
        parents: ['app', 'exercises'],
    }),
    Landing: Route({name: Symbol('landing'), pathName: ''}),
    Login: Route({name: Symbol('login'), pathName: 'login'}),
    Logout: Route({name: Symbol('logout'), pathName: 'logout'}),
    Register: Route({name: Symbol('register'), pathName: 'register'}),
    UserDashboard: Route({name: Symbol('user-dashboard'), pathName: 'dashboard', parents: ['app']}),
    UserProgress: Route({name: Symbol('user-progress'), pathName: 'progress', parents: ['app']}),
    UserSettings: Route({name: Symbol('user-settings'), pathName: 'settings', parents: ['app']}),
    UserTracker: Route({
        name: Symbol('user-workout-tracker'),
        pathName: 'tracker',
        parents: ['app'],
    }),
} as const;

export const ExerciseDetailsPage = Routes.ExerciseDetailsPage;
export const ExerciseHomePage = Routes.ExerciseHomePage;
export const ExercisesByMuscleGroupPage = Routes.ExercisesByMuscleGroupPage;
export const Landing = Routes.Landing;
export const Login = Routes.Login;
export const Logout = Routes.Logout;
export const Register = Routes.Register;
export const UserDashboard = Routes.UserDashboard;
export const UserProgress = Routes.UserProgress;
export const UserSettings = Routes.UserSettings;
export const UserTracker = Routes.UserTracker;
