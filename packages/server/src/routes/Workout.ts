import {Router} from 'express';
import type {Request, Response} from 'express';

import Exercise from '@/models/Exercise';
import Set, {type SetProperties} from '@/models/Set';
import User from '@/models/User';
import Workout from '@/models/Workout';

interface CreateWorkoutRequest extends Request {
    body: {
        user_id: string;
        title: string;
        notes?: string;
        start: string;
        end: number;
        exercises: [
            {
                name: string;
                notes: string;
                type: string;
                sets: Partial<SetProperties>[];
            },
        ];
    };
}

const path = '/workouts';
const router = Router();

router.post(path, async (req: CreateWorkoutRequest, res: Response) => {
    const {body} = req;

    const user = await User.findById(body.user_id);
    if (user === null) {
        res.status(400).json({
            error: `A user with ID ${body.user_id} does not exist`,
        });

        return;
    }

    const start = new Date(body.start);
    const end = new Date(body.end);
    const milliseconds = end.getTime() - start.getTime();

    if (milliseconds < 0) {
        res.status(400).json({
            error: 'The ending timestamp must be after the starting timestamp.',
        });
        return;
    } else if (milliseconds === 0) {
        res.status(200).json({});
        return;
    }

    const workout = new Workout({
        user_id: user._id,
        title: body.title,
        notes: body.notes,
        start,
        end,
        duration_ms: milliseconds,
    });

    for (const exercise of body.exercises) {
        let exerciseDocument = await Exercise.findOne({
            name: exercise.name,
            user_id: user._id,
        });

        if (exerciseDocument === null) {
            exerciseDocument = new Exercise({
                name: exercise.name,
                type: exercise.type,
                user_id: user._id,
            });
        }

        for (const set of exercise.sets) {
            const setDocument = await Set.create({
                ...set,
                exercise_id: exerciseDocument._id,
                user_id: user._id,
                workout_id: workout._id,
            });

            exerciseDocument.sets.push(setDocument._id);
        }

        exerciseDocument.notes = exercise.notes;
        await exerciseDocument.save();
        workout.exercises.push(exerciseDocument._id);
    }

    await workout.save();
    await workout.populate([
        {
            path: 'exercises',
            select: '-__v -user_id -created_at -updated_at',
            populate: {
                path: 'sets',
                model: 'Set',
                select: '-__v -user_id -exercise_id -workout_id -created_at -updated_at',
                match: {
                    workout: workout._id,
                },
            },
        },
    ]);

    res.status(201).json(
        workout.toJSON({
            versionKey: false,
        }),
    );
});

export = router;
