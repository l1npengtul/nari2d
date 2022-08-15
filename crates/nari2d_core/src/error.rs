use crate::geometry::point2d::Point2d;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Nari2DCoreError {
    // Mesh
    #[error("Too few points on mesh: {0}")]
    TooFewPoints(u8),
    #[error("This operation creates/encountered a non manifold structure")]
    NonManifoldStructure,
    #[error("Point ({0}) already exists.")]
    AlreadyExists(Point2d),
    #[error("Mesh Element {0} does not exist.")]
    DoesNotExist(Cow<'static, str>),
    #[error("Failed to calculate hull: {0}")]
    HullCalculation(Cow<'static, str>),
    #[error("Failed to triangulate points: {0}")]
    Triangulation(Cow<'static, str>),

    // General
    #[error("General Operation Error: {0}")]
    General(Cow<'static, str>),
    #[error("This error \"{0}\" should not have happened! Please report it here: https://github.com/l1npengtul/nari2d")]
    ThisIsABug(Cow<'static, str>),
}

pub type NCResult<T> = Result<T, Nari2DCoreError>; // new california result <T>
