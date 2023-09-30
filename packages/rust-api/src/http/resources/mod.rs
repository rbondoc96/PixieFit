mod link;
mod muscle;
mod name;
mod profile;
mod user;

pub use link::LinkResource;
pub use muscle::MuscleResource;
pub use name::NameResource;
pub use profile::ProfileResource;
pub use user::UserResource;

use async_trait::async_trait;
use futures::future::join_all;
use serde::Serialize;

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
    type Model: crate::models::Model + Unpin + Send;

    /// Create a new resource from a model
    async fn default(model: Self::Model) -> Self;
    async fn simple(model: Self::Model) -> Self;

    async fn new(model: Self::Model, format: ModelResourceFormat) -> Self {
        match format {
            ModelResourceFormat::Default => Self::default(model).await,
            ModelResourceFormat::Simple => Self::simple(model).await,
        }
    }

    async fn list(models: Vec<Self::Model>) -> Vec<Self> {
        Self::list_simple(models).await
    }

    async fn list_default(models: Vec<Self::Model>) -> Vec<Self> {
        join_all(models.into_iter().map(Self::default)).await
    }

    async fn list_simple(models: Vec<Self::Model>) -> Vec<Self> {
        join_all(models.into_iter().map(Self::simple)).await
    }
}
