import {Router} from 'express';
import type {Request, Response} from 'express';

import Exercise from '@/models/Exercise';
import Workout from '@/models/Workout';

const path = '/users';
const router = Router();

interface GetUserWorkoutRequest extends Request {
    params: {
        id: string;
    };
}

interface GetUserSetsByExerciseRequest extends Request {
    params: {
        id: string;
    };
    query: {
        exercise_id: string;
    };
}

router.get(
    path + '/:id/workouts',
    async (req: GetUserWorkoutRequest, res: Response) => {
        const {id} = req.params;

        try {
            const workouts = await Workout.find({
                user: id,
            }).select('-__v -created_at -updated_at');

            const populatedWorkouts = await Promise.all(
                workouts.map(
                    async (workout) =>
                        await workout.populate([
                            {
                                path: 'exercises',
                                select: '-__v -user_id -created_at -updated_at',
                                populate: {
                                    path: 'sets',
                                    model: 'Set',
                                    select: '-__v -user_id -workout_id -exercise_id -updated_at',
                                    match: {
                                        workout_id: workout._id,
                                    },
                                },
                            },
                        ]),
                ),
            );

            console.log(populatedWorkouts);

            res.status(200).json(populatedWorkouts);
        } catch (error: unknown) {
            res.status(400).json({});
        }
    },
);

router.get(
    path + '/:id/sets',
    async (req: GetUserSetsByExerciseRequest, res: Response) => {
        const {params, query} = req;

        const exercise = await Exercise.findOne({
            _id: query.exercise_id,
            user_id: params.id,
        })
            .populate([
                {
                    path: 'sets',
                    select: '-__v -exercise_id -user_id -updated_at',
                },
            ])
            .select('-__v -created_at -updated_at');

        res.status(200).json(exercise ?? {});
    },
);

export = router;
