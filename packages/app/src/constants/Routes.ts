import Route from '@/lib/Route';

const Routes = {
    Exercises: Route({name: Symbol('exercises'), pathName: 'exercises', parents: ['app']}),
    Landing: Route({name: Symbol('landing'), pathName: ''}),
    Login: Route({name: Symbol('login'), pathName: 'login'}),
    Register: Route({name: Symbol('register'), pathName: 'register'}),
    PageNotFound: Route({name: Symbol('page-not-found'), pathName: '*'}),
    UserDashboard: Route({name: Symbol('user-dashboard'), pathName: 'dashboard', parents: ['app']}),
    UserProgress: Route({name: Symbol('user-progress'), pathName: 'progress', parents: ['app']}),
    UserSettings: Route({name: Symbol('user-settings'), pathName: 'settings', parents: ['app']}),
    UserTracker: Route({
        name: Symbol('user-workout-tracker'),
        pathName: 'tracker',
        parents: ['app'],
    }),
} as const;

export const Exercises = Routes.Exercises;
export const Landing = Routes.Landing;
export const Login = Routes.Login;
export const Register = Routes.Register;
export const PageNotFound = Routes.PageNotFound;
export const UserDashboard = Routes.UserDashboard;
export const UserProgress = Routes.UserProgress;
export const UserSettings = Routes.UserSettings;
export const UserTracker = Routes.UserTracker;

export default Routes;
