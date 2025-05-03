use std::{cell::RefCell, sync::Arc};

use legion::Resources;
use rapier2d::prelude::*;

pub struct PhysicsContext {
    pub gravity: Arc<RefCell<Vector<f32>>>,
    pub pipeline: Arc<RefCell<PhysicsPipeline>>,
    pub integration_parameters: Arc<RefCell<IntegrationParameters>>,
    pub islands: Arc<RefCell<IslandManager>>,
    pub broad_phase: Arc<RefCell<DefaultBroadPhase>>,
    pub narrow_phase: Arc<RefCell<NarrowPhase>>,
    pub bodies: Arc<RefCell<RigidBodySet>>,
    pub colliders: Arc<RefCell<ColliderSet>>,
    pub impulse_joints: Arc<RefCell<ImpulseJointSet>>,
    pub multibody_joints: Arc<RefCell<MultibodyJointSet>>,
    pub ccd_solver: Arc<RefCell<CCDSolver>>,
    pub query_pipeline: Arc<RefCell<QueryPipeline>>,
}

impl PhysicsContext {
    fn new() -> Self {
        PhysicsContext {
            gravity: Arc::new(RefCell::new(vector![0.0, 1.0])),
            pipeline: Arc::new(RefCell::new(PhysicsPipeline::default())),
            integration_parameters: Arc::new(RefCell::new(IntegrationParameters::default())),
            islands: Arc::new(RefCell::new(IslandManager::default())),
            broad_phase: Arc::new(RefCell::new(DefaultBroadPhase::default())),
            narrow_phase: Arc::new(RefCell::new(NarrowPhase::default())),
            bodies: Arc::new(RefCell::new(RigidBodySet::default())),
            colliders: Arc::new(RefCell::new(ColliderSet::default())),
            impulse_joints: Arc::new(RefCell::new(ImpulseJointSet::default())),
            multibody_joints: Arc::new(RefCell::new(MultibodyJointSet::default())),
            ccd_solver: Arc::new(RefCell::new(CCDSolver::default())),
            query_pipeline: Arc::new(RefCell::new(QueryPipeline::default())),
        }
    }
}

pub fn init_physics(resources: &mut Resources) {
    resources.insert(PhysicsContext::new());
}
