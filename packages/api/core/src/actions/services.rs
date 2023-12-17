use crate::enums::{ExerciseForce, ExerciseMechanic, ExerciseMuscleTarget, ExerciseType, Measurement};
use crate::models::{Exercise, ExerciseInstruction, ExerciseMuscleMap};
use database::DatabaseManager;
use futures::future::join_all;
use serde::Deserialize;
use sqlx::postgres::PgRow;
use sqlx::{Decode, FromRow, PgPool, Postgres, Type};
use std::collections::HashMap;
use std::convert::Infallible;

pub struct IdMap<K>
where
    K: Type<Postgres> + for<'q> Decode<'q, Postgres> + Send + std::fmt::Display + Clone + Unpin,
{
    map: HashMap<i64, K>,
    database: DatabaseManager,
    table: &'static str,
}

impl<K> IdMap<K>
where
    K: Type<Postgres> + for<'q> Decode<'q, Postgres> + Send + std::fmt::Display + Clone + Unpin,
{
    pub fn new(table: &'static str, database: DatabaseManager) -> Self {
        Self {
            map: HashMap::new(),
            database,
            table,
        }
    }

    pub fn get(&self, identifier: impl Into<i64>) -> Option<&K> {
        self.map.get(&identifier.into())
    }

    pub async fn insert(mut self, identifier: impl Into<i64>, name: impl Into<String> + Clone) -> Self {
        let id = sqlx::query_as::<_, (K,)>(format!(
            "SELECT id FROM {} WHERE name = $1", self.table)
            .as_str())
            .bind(name.clone().into())
            .fetch_one(self.database.connection())
            .await
            .expect(format!(
                "Failed to find an id in {} with name \"{}\".",
                self.table, name.into()
            ).as_str())
            .0;

        self.map.insert(identifier.into(), id);
        self
    }
}

pub async fn init(database: &DatabaseManager) -> Result<(), Infallible> {
    let muscle_map = create_muscle_map(database).await;
    let muscle_group_map = create_muscle_group_map(database).await;
    let equipment_map = create_equipment_map(database).await;

    let mut limit = 50;
    let mut offset = 0;

    loop {
        let mw_exercises = fetch_mw_exercise_list(limit, offset).await;

        if mw_exercises.len() == 0 {
            break;
        }

        join_all(mw_exercises.iter()
            .map(|exercise| {
                convert_mw_exercise_to_model(
                    exercise.clone(),
                    &muscle_map,
                    &muscle_group_map,
                    &equipment_map,
                    database
                )
            })
        ).await;

        offset += limit;
    }

    Ok(())
}

async fn create_equipment_map(database: &DatabaseManager) -> IdMap<i16> {
    let map = IdMap::new("exercise_equipment", database.clone());

    map.insert(10, "resistance_band").await
        .insert(1, "barbell").await
        .insert(3, "bodyweight").await
        .insert(24, "bosu_ball").await
        .insert(9, "cable_machine").await
        .insert(27, "cardio_machine").await
        .insert(2, "dumbbell").await
        .insert(7, "kettlebell").await
        .insert(4, "weight_machine").await
        .insert(6, "medicine_ball").await
        .insert(11, "plate").await
        .insert(85, "smith_machine").await
        .insert(13, "yoga").await
        .insert(12, "trx").await
        .insert(26, "other").await
}

async fn create_muscle_group_map(database: &DatabaseManager) -> IdMap<i16> {
    // key is tree_id, value is id in the DB
    let map = IdMap::new("muscle_groups", database.clone());

    map.insert(1, "Arms").await
        .insert(2, "Upper Back").await
        .insert(3, "Lower Back").await
        .insert(4, "Core").await
        .insert(5, "Calves").await
        .insert(6, "Forearms").await
        .insert(7, "Glutes").await
        .insert(8, "Hamstrings").await
        .insert(9, "Lats").await
        .insert(10, "Shoulders").await
        .insert(11, "Arms").await
        .insert(12, "Upper Back").await
        .insert(13, "Quadriceps").await
        .insert(14, "Chest").await
        .insert(15, "Core").await
        .insert(16, "Hands").await
}

async fn create_muscle_map(database: &DatabaseManager) -> IdMap<i16> {
    // key is muscle_id, value is id in the DB
    let map = IdMap::new("muscles", database.clone());

    map.insert(1,"Biceps Brachii").await
        .insert(16, "Biceps Brachii, Long Head").await
        .insert(17, "Biceps Brachii, Short Head").await
        .insert(14, "Trapezius").await
        .insert(13, "Lower Back").await
        .insert(12, "Abdominals").await
        .insert(21, "Transversus Abdominis").await
        .insert(22, "Rectus Abdominis").await
        .insert(11, "Calves").await
        .insert(31, "Tibialis").await
        .insert(32, "Soleus").await
        .insert(33, "Gastrocnemius").await
        .insert(10, "Forearms").await
        .insert(25, "Wrist Extensors").await
        .insert(26, "Wrist Flexors").await
        .insert(9, "Glutes").await
        .insert(37, "Gluteus Medius").await
        .insert(38, "Gluteus Maximus").await
        .insert(8, "Hamstrings").await
        .insert(39, "Medial Hamstrings").await
        .insert(40, "Lateral Hamstrings").await
        .insert(7, "Latissimus Dorsi").await
        .insert(6, "Shoulders").await
        .insert(18, "Lateral Deltoid").await
        .insert(19, "Anterior Deltoid").await
        .insert(20, "Posterior Deltoid").await
        .insert(5, "Triceps Brachii").await
        .insert(34, "Triceps Brachii, Long Head").await
        .insert(35, "Triceps Brachii, Lateral Head").await
        .insert(36, "Triceps Brachii, Medial Head").await
        .insert(4, "Trapezius").await
        .insert(41, "Trapezius, Superior").await
        .insert(42, "Trapezius, Inferior").await
        .insert(3, "Quadriceps").await
        .insert(27, "Hip Adductors").await
        .insert(28, "Vastus Medialis").await
        .insert(29, "Vastus Lateralis").await
        .insert(30, "Rectus Femoris").await
        .insert(2, "Pectoralis").await
        .insert(23, "Pectoralis Major, Clavicular Head").await
        .insert(24, "Pectoralis Major, Sternocostal Head").await
        .insert(15, "Obliques").await
        .insert(43, "Palmar Fascia").await
}

async fn fetch_mw_exercise_list(limit: u16, offset: u16) -> Vec<MWExercise> {
    let endpoint = format!(
        "https://musclewiki.com/newapi/exercise/exercises/?limit={}&offset={}",
        limit, offset
    );

    println!("Request made to: {}", endpoint);
    let response = reqwest::get(&endpoint)
        .await
        .expect("Failed to fetch musclewiki exercise list")
        .json::<MWExerciseListResponse>()
        .await
        .expect("Failed to parse musclewiki exercise list response");

    response.results
}

async fn convert_mw_exercise_to_model(
    mw_exercise: MWExercise,
    muscle_map: &IdMap<i16>,
    muscle_group_map: &IdMap<i16>,
    equipment_map: &IdMap<i16>,
    database: &DatabaseManager,
) -> Result<(), Infallible> {
    let name = mw_exercise.name.clone();
    let group_id = mw_exercise.muscles_primary.get(0)
        .map(|muscle| muscle.tree_id)
        .map(|tree_id| {
            muscle_group_map.get(tree_id).expect(format!(
                "Expected to find id for identifier {} in muscle_group_map", tree_id,
            ).as_str())
        })
        .cloned();

    let equipment_id = equipment_map.get(mw_exercise.category.id)
        .cloned();

    let exercise_type = match &equipment_id {
        Some(_) => ExerciseType::Strength,
        None => ExerciseType::Stretch,
    };

    let exercise = Exercise::new()
        .external_id(Some(mw_exercise.id))
        .exercise_type(exercise_type)
        .target_muscle_group_id(group_id.clone())
        .name(name.clone())
        .name_alternative(mw_exercise.name_alternative)
        .description(None)
        .equipment_id(equipment_id)
        .mechanic(mw_mechanic_to_exercise_mechanic(mw_exercise.mechanic))
        .force(mw_force_to_exercise_force(mw_exercise.force))
        .measurement(mw_full_measure_to_exercise_measurement(mw_exercise.full_measure))
        .create(&database)
        .await;

    if let Err(error) = exercise {
        println!("Failed to create exercise: {:?}", name);
        println!("Error: {:?}", error);
        return Ok(());
    }

    let exercise = exercise.unwrap();

    // Create muscle relationships
    join_all(mw_exercise.muscles_primary.iter()
        .map(|muscle| {
            let id = muscle_map.get(muscle.id).expect(format!(
                "Expected to find muscle with identifier {} in muscle_map", muscle.id,
            ).as_str());

            ExerciseMuscleMap::new()
                .exercise_id(exercise.id)
                .muscle_id(id.clone())
                .target(ExerciseMuscleTarget::Primary)
                .create(&database)
        })
    ).await
    .iter()
    .for_each(|relation| match relation {
        Err(error) => println!("Failed to create primary muscle: {:?}", error),
        _ => {},
    });

    join_all(mw_exercise.muscles_secondary.iter()
        .map(|muscle| {
            let id = muscle_map.get(muscle.id).expect(format!(
                "Expected to find muscle with identifier {} in muscle_map", muscle.id,
            ).as_str());

            ExerciseMuscleMap::new()
                .exercise_id(exercise.id)
                .muscle_id(id.clone())
                .target(ExerciseMuscleTarget::Secondary)
                .create(&database)
        })
    ).await
    .iter()
    .for_each(|relation| match relation {
        Err(error) => println!("Failed to create primary muscle: {:?}", error),
        _ => {},
    });


    join_all(mw_exercise.muscles_tertiary.iter()
        .map(|muscle| {
            let id = muscle_map.get(muscle.id).expect(format!(
                "Expected to find muscle with identifier {} in muscle_map", muscle.id,
            ).as_str());

            ExerciseMuscleMap::new()
                .exercise_id(exercise.id)
                .muscle_id(id.clone())
                .target(ExerciseMuscleTarget::Tertiary)
                .create(&database)
        })
    ).await
    .iter()
    .for_each(|relation| match relation {
        Err(error) => println!("Failed to create tertiary muscle: {:?}", error),
        _ => {},
    });

    // Create instruction relationships
    join_all(mw_exercise.correct_steps.iter()
        .map(|step| {
            ExerciseInstruction::new()
                .exercise_id(exercise.id)
                .sequence_number(step.order)
                .content(step.text.clone())
                .create(&database)
        })
    ).await
    .iter()
    .for_each(|instruction| match instruction {
        Err(error) => println!("Failed to create instruction: {:?}", error),
        _ => {},
    });

    Ok(())
}

fn mw_force_to_exercise_force(force: Option<MWForce>) -> Option<ExerciseForce> {
    if force.is_none() {
        return None;
    }

    match force.unwrap().name.to_lowercase().as_str() {
        "hold" => Some(ExerciseForce::Hold),
        "pull" => Some(ExerciseForce::Pull),
        "push" => Some(ExerciseForce::Push),
        _ => None
    }
}

fn mw_mechanic_to_exercise_mechanic(mechanic: Option<MWMechanic>) -> Option<ExerciseMechanic> {
    if mechanic.is_none() {
        return None;
    }

    match mechanic.unwrap().name.to_lowercase().as_str() {
        "compound" => Some(ExerciseMechanic::Compound),
        "isolation" => Some(ExerciseMechanic::Isolation),
        _ => None
    }
}

fn mw_full_measure_to_exercise_measurement(measurement: Option<MWFullMeasure>) -> Option<Measurement> {
    if measurement.is_none() {
        return None;
    }

    let measurement = measurement.unwrap();
    let id = measurement.id;

    match id {
        4 => Some(Measurement::WeightedRepetitions),
        11 => Some(Measurement::WeightedDuration),
        18 => Some(Measurement::Duration),
        19 => Some(Measurement::Repetitions),
        _ => None
    }
}

#[derive(Debug, Deserialize)]
pub struct MWExerciseListResponse {
    pub count: i16,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<MWExercise>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWExercise {
    pub id: i16,
    pub name: String,
    pub name_alternative: Option<String>,
    // pub description: Option<String>,
    pub muscles: Vec<MWMuscle>,
    pub muscles_primary: Vec<MWMuscle>,
    pub muscles_secondary: Vec<MWMuscle>,
    pub muscles_tertiary: Vec<MWMuscle>,
    pub category: MWCategory,
    pub force: Option<MWForce>,
    pub mechanic: Option<MWMechanic>,
    pub correct_steps: Vec<MWStep>,
    pub full_measure: Option<MWFullMeasure>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWMuscle {
    pub id: i16,
    pub name: String,
    pub tree_id: i16,
    // pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWCategory {
    pub id: i16,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWForce {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWMechanic {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWStep {
    pub order: i16,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWFullMeasure {
    pub id: i16,
    pub measure: MWMeasure,
    pub denominator: Option<MWDenominator>,
    pub calculation_mode: MWCalculationMode,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWMeasure {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWDenominator {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MWCalculationMode {
    pub name: String,
}
