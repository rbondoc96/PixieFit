mod exercise;
mod exercise_equipment;
mod exercise_instruction;
mod link;
mod measurement;
mod muscle;
mod muscle_group;
mod name;
mod profile;
mod user;

pub use exercise::ExerciseResource;
pub use exercise_equipment::ExerciseEquipmentResource;
pub use exercise_instruction::ExerciseInstructionResource;
pub use link::LinkResource;
pub use measurement::MeasurementResource;
pub use muscle::MuscleResource;
pub use muscle_group::MuscleGroupResource;
pub use name::NameResource;
pub use profile::ProfileResource;
pub use user::UserResource;

use async_trait::async_trait;
use database::DatabaseManager;
use serde::Serialize;

pub(self) type ResourceResult<T> = Result<T, crate::models::Error>;

pub enum ModelResourceFormat {
    Default,
    Simple,
}

/// Trait for converting a model into an HTTP resource
/// that is sent as part of an HTTP response.
///
/// # Implementing `ModelResource`
///
/// `ModelResource` provides default implementations for creating
///  a list of resources in the following formats:
///
///     - `ModelResourceFormat::Default`
///     - `ModelResourceFormat::Simple`
///
/// When this trait is implemented for a model, it must specify
/// the `Model` type and how a single instance is converted
/// into an HTTP resource in the above formats, by implementing
/// the `async default()` and `async simple()` methods.
///
/// ## Tip
///
/// When creating the structure of the resource, list out all
/// fields that will be included in the `Default` format. Then, wrap any
/// fields that should be excluded from the `Simple` format in an `Option` and
/// apply the `#[serde(skip_serializing_if = "Option::is_none")]` attribute to
/// the field.
///
/// Then, when implementing `async simple()`, place a `None` value for fields
/// that should be excluded. This will omit the field from the serialized JSON,
/// rather than putting a `null` value in its place.
///
/// ## Example
///
/// ```rust
/// use super::{LinkResource, ModelResource};
/// use crate:: {
///     enums::MuscleGroup,
///     models::{Link, Muscle},
/// };
/// use async_trait::async_trait;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// pub struct MuscleResource {
///     id: i64,
///     muscle_group: MuscleGroup;
///     name: String,
///     simple_name: Option<String>,
///
///     #[serde(skip_serializing_if = "Option::is_none")]
///     links: Option<Vec<LinkResource>>,
/// }
///
/// #[async_trait]
/// impl ModelResource for MuscleResource {
///    type Model = Muscle;
///
///     async fn default(muscle: Muscle) -> Self {
///         let links = match muscle.links().await {
///             Ok(links) => links,
///             Err(_) => vec![],
///         };
///
///         Self {
///             id: muscle.id(),
///             muscle_group: muscle.muscle_group(),
///             name: muscle.name(),
///             simple_name: muscle.simple_name(),
///             links: Some(LinkResource::list(links)),
///         }
///     }
///
///     async fn simple(muscle: Muscle) -> Self {
///         Self {
///             id: muscle.id(),
///             muscle_group: muscle.muscle_group(),
///             name: muscle.name(),
///             simple_name: muscle.simple_name(),
///             links: None,
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait ModelResource
where
    Self: Send + Serialize + Sized + Sync,
{
    type Model: database::Model + Unpin + Send;

    async fn default(model: Self::Model, database: &DatabaseManager) -> ResourceResult<Self>;
    async fn simple(model: Self::Model, database: &DatabaseManager) -> ResourceResult<Self>;

    async fn list(models: Vec<Self::Model>, database: &DatabaseManager) -> ResourceResult<Vec<Self>> {
        let mut results = Vec::with_capacity(models.len());

        for model in models {
            results.push(Self::simple(model, database).await?);
        }
        
        Ok(results)
    }
}
