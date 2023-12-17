use crate::enums::*;
use crate::http::Error;
use crate::models::*;
use chrono::{Datelike, DateTime, NaiveDate, Utc};
use database::DatabaseManager;
use fake::Fake;
use fake::faker::{
    chrono::en::DateTime,
    company::en::BsNoun,
    filesystem::en::FileName,
    internet::en::{FreeEmail, Password as FakerPassword},
    lorem::en::{Paragraph, Words},
    name::en::{FirstName, LastName},
};

type Result<T> = core::result::Result<T, Error>;

// region User

use crate::models::user::{
    Email,
    Password,
    Name as UserFullName,
    NoEmail,
    NoName as NoUserFullName,
    NoPassword,
    NoUserRole,
    UserBuilder,
    UserRole
};

impl User {
    pub fn fake() -> UserBuilder<Password, UserRole, Email, UserFullName> {
        UserBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<User> {
        let user = UserBuilder::fake()
            .create(database)
            .await?;

        Ok(user)
    }
}

impl UserBuilder<NoPassword, NoUserRole, NoEmail, NoUserFullName> {
    pub fn fake() -> UserBuilder<Password, UserRole, Email, UserFullName> {
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();
        let email: String = FreeEmail().fake();
        let password: String = FakerPassword(8..32).fake::<String>();

        User::new()
            .name(first_name, last_name)
            .email(email)
            .role(Role::User)
            .password(password)
    }
}

// endregion

// region Profile

use crate::models::profile::{
    NoBirthday,
    NoUserGender,
    NoUserId,
    Birthday,
    UserGender,
    UserId,
    ProfileBuilder,
};

impl Profile {
    pub fn fake() -> ProfileBuilder<NoUserId, Birthday, UserGender> {
        ProfileBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<Profile> {
        let profile = ProfileBuilder::fake()
            .create(database)
            .await?;

        Ok(profile)
    }
}

impl ProfileBuilder<NoUserId, NoBirthday, NoUserGender> {
    pub fn fake() -> ProfileBuilder<NoUserId, Birthday, UserGender> {
        let birthday = DateTime().fake::<DateTime<Utc>>();

        ProfileBuilder::new()
            .birthday(NaiveDate::from_ymd_opt(
                birthday.year(),
                birthday.month(),
                birthday.day()
            ).unwrap())
            .gender(Gender::Other)
    }
}

impl ProfileBuilder<NoUserId, Birthday, UserGender> {
    pub async fn create(self, database: &DatabaseManager) -> Result<Profile> {
        let user = User::mocked(database).await?;

        let profile = self.user_id(user.id)
            .create(database)
            .await?;

        Ok(profile)
    }
}

// endregion

// region MuscleGroup

use crate::models::muscle_group::{
    MuscleGroupBuilder,
    NoName as NoMuscleGroupName,
    Name as MuscleGroupName,
};

impl MuscleGroup {
    pub fn fake() -> MuscleGroupBuilder<MuscleGroupName> {
        MuscleGroupBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<MuscleGroup> {
        let group = MuscleGroupBuilder::fake()
            .create(database)
            .await?;

        Ok(group)
    }
}

impl MuscleGroupBuilder<NoMuscleGroupName> {
    pub fn fake() -> MuscleGroupBuilder<MuscleGroupName> {
        let tokens: Vec<String> = Words(1..5).fake();
        let name = tokens.join(" ");

        MuscleGroup::new().name(name)
    }
}

// endregion

// region Muscle

use crate::models::muscle::{
    MuscleBuilder,
    GroupId as MuscleGroupId,
    Name as MuscleName,
    NoName as NoMuscleName,
    NoGroupId as NoMuscleGroupId,
};

impl Muscle {
    pub fn fake() -> MuscleBuilder<NoMuscleGroupId, MuscleName> {
        MuscleBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<Muscle> {
        let muscle = MuscleBuilder::fake()
            .create(database)
            .await?;

        Ok(muscle)
    }
}

impl MuscleBuilder<NoMuscleGroupId, NoMuscleName> {
    pub fn fake() -> MuscleBuilder<NoMuscleGroupId, MuscleName> {
        let tokens: Vec<String> = Words(1..5).fake();
        let name = tokens.join(" ");

        Muscle::new().name(name)
    }
}

impl MuscleBuilder<NoMuscleGroupId, MuscleName> {
    pub async fn create(self, database: &DatabaseManager) -> Result<Muscle> {
        let group = MuscleGroup::mocked(database).await?;

        let muscle = self.group(&group)
            .create(database)
            .await?;

        Ok(muscle)
    }
}

// endregion

// region ExerciseEquipment

use crate::models::exercise_equipment::{
    ExerciseEquipmentBuilder,
    NoName as NoExerciseEquipmentName,
    Name as ExerciseEquipmentName,
};

impl ExerciseEquipment {
    pub fn fake() -> ExerciseEquipmentBuilder<ExerciseEquipmentName> {
        ExerciseEquipmentBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<ExerciseEquipment> {
        let group = ExerciseEquipmentBuilder::fake()
            .create(database)
            .await?;

        Ok(group)
    }
}

impl ExerciseEquipmentBuilder<NoExerciseEquipmentName> {
    pub fn fake() -> ExerciseEquipmentBuilder<ExerciseEquipmentName> {
        let tokens: Vec<String> = Words(1..5).fake();
        let name = tokens.join(" ");

        ExerciseEquipment::new().name(name)
    }
}

// endregion

// region Exercise

use crate::models::exercise::{
    ExerciseBuilder,
    NoName as NoExerciseName,
    NoType as NoExerciseType,
    Name as ExerciseName,
    Type as WrappedExerciseType,
};

impl Exercise {
    pub fn fake() -> ExerciseBuilder<WrappedExerciseType, ExerciseName> {
        ExerciseBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<Exercise> {
        let exercise = Exercise::fake()
            .create(database)
            .await?;

        Ok(exercise)
    }
}

impl ExerciseBuilder<NoExerciseType, NoExerciseName> {
    pub fn fake() -> ExerciseBuilder<WrappedExerciseType, ExerciseName> {
        let tokens: Vec<String> = Words(1..5).fake();
        let name = tokens.join(" ");

        let tokens: Vec<String> = Words(1..5).fake();
        let simple_name = tokens.join(" ");

        let tokens: Vec<String> = Words(1..5).fake();
        let description = tokens.join(" ");

        Exercise::new()
            .exercise_type(ExerciseType::Strength)
            .name(name)
            .name_alternative(Some(simple_name))
            .description(Some(description))
            .force(Some(ExerciseForce::Hold))
            .mechanic(Some(ExerciseMechanic::Compound))
            .measurement(Some(Measurement::WeightedRepetitions))
    }
}

// endregion

// region ExerciseInstruction

use crate::models::exercise_instruction::{
    ExerciseInstructionBuilder,
    Content,
    ExerciseId,
    SequenceNumber,
    NoContent,
    NoExerciseId,
    NoSequenceNumber,
};

impl ExerciseInstruction {
    pub fn fake() -> ExerciseInstructionBuilder<NoExerciseId, SequenceNumber, Content> {
        ExerciseInstructionBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<ExerciseInstruction> {
        let instruction = ExerciseInstructionBuilder::fake()
            .create(database)
            .await?;

        Ok(instruction)
    }
}

impl ExerciseInstructionBuilder<NoExerciseId, NoSequenceNumber, NoContent> {
    pub fn fake() -> ExerciseInstructionBuilder<NoExerciseId, SequenceNumber, Content> {
        let paragraph: String = Paragraph(1..3).fake();

        ExerciseInstructionBuilder::new()
            .sequence_number(1)
            .content(paragraph)
    }
}

impl ExerciseInstructionBuilder<NoExerciseId, SequenceNumber, Content> {
    pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseInstruction> {
        let exercise = Exercise::mocked(database).await?;

        let instruction = self.exercise(&exercise)
            .create(database)
            .await?;

        Ok(instruction)
    }
}

// endregion

// region ExerciseMuscleMap

use crate::models::exercise_muscle_map::{
    ExerciseMuscleMapBuilder,
    ExerciseId as ExerciseIdMap,
    MuscleId as MuscleIdMap,
    Target as ExerciseMuscleMapTarget,
    NoExerciseId as NoExerciseIdMap,
    NoMuscleId as NoMuscleIdMap,
    NoTarget as NoExerciseMuscleMapTarget,
};

impl ExerciseMuscleMap {
    pub fn fake() -> ExerciseMuscleMapBuilder<NoExerciseIdMap, NoMuscleIdMap, ExerciseMuscleMapTarget> {
        ExerciseMuscleMapBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<ExerciseMuscleMap> {
        let map = ExerciseMuscleMapBuilder::fake()
            .create(database)
            .await?;

        Ok(map)
    }
}

impl ExerciseMuscleMapBuilder<NoExerciseIdMap, NoMuscleIdMap, NoExerciseMuscleMapTarget> {
    pub fn fake() -> ExerciseMuscleMapBuilder<NoExerciseIdMap, NoMuscleIdMap, ExerciseMuscleMapTarget> {
        ExerciseMuscleMapBuilder::new()
            .target(ExerciseMuscleTarget::Primary)
    }
}

impl ExerciseMuscleMapBuilder<NoExerciseIdMap, NoMuscleIdMap, ExerciseMuscleMapTarget> {
    pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseMuscleMap> {
        let muscle = Muscle::mocked(database).await?;
        let exercise = Exercise::mocked(database).await?;

        let map = self.exercise(&exercise)
            .muscle(&muscle)
            .create(database)
            .await?;

        Ok(map)
    }
}

impl ExerciseMuscleMapBuilder<ExerciseIdMap, NoMuscleIdMap, ExerciseMuscleMapTarget> {
    pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseMuscleMap> {
        let muscle = Muscle::mocked(database).await?;

        let map = self.muscle(&muscle)
            .create(database)
            .await?;

        Ok(map)
    }
}

impl ExerciseMuscleMapBuilder<NoExerciseIdMap, MuscleIdMap, ExerciseMuscleMapTarget> {
    pub async fn create(self, database: &DatabaseManager) -> Result<ExerciseMuscleMap> {
        let exercise = Exercise::mocked(database).await?;

        let map = self.exercise(&exercise)
            .create(database)
            .await?;

        Ok(map)
    }
}

// endregion

// region Link

use crate::models::link::{
    LinkBuilder,
    ModelData as LinkModelData,
    Type as LinkTypeEnum,
    Format as LinkFormatEnum,
    Label as LinkLabel,
    Src as LinkSrc,
    NoModelData as NoLinkModelData,
    NoType as NoLinkTypeEnum,
    NoFormat as NoLinkFormatEnum,
    NoLabel as NoLinkLabel,
    NoSrc as NoLinkSrc,
};

impl Link {
    pub fn fake() -> LinkBuilder<NoLinkModelData, LinkTypeEnum, LinkFormatEnum, LinkLabel, LinkSrc> {
        LinkBuilder::fake()
    }

    pub async fn mocked(database: &DatabaseManager) -> Result<Link> {
        let link = LinkBuilder::fake()
            .create(database)
            .await?;

        Ok(link)
    }
}

impl LinkBuilder<NoLinkModelData, NoLinkTypeEnum, NoLinkFormatEnum, NoLinkLabel, NoLinkSrc> {
    pub fn fake() -> LinkBuilder<NoLinkModelData, LinkTypeEnum, LinkFormatEnum, LinkLabel, LinkSrc> {
        let src: String = FileName().fake();
        let label_tokens: Vec<String> = Words(3..8).fake();

        LinkBuilder::new()
            .link_type(LinkType::Image)
            .format(LinkFormat::Jpeg)
            .label(label_tokens.join(" "))
            .src(src)
    }
}

impl LinkBuilder<NoLinkModelData, LinkTypeEnum, LinkFormatEnum, LinkLabel, LinkSrc> {
    pub async fn create(self, database: &DatabaseManager) -> Result<Link> {
        let model = Muscle::mocked(database).await?;
        let link = self.model(Table::Muscles, model.id)
            .create(database)
            .await?;

        Ok(link)
    }
}

// endregion
