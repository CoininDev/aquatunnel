use legion::Resources;
use rapier2d::prelude::*;

pub struct PhysicsContext {
    pub gravity: Vector<f32>,
    pub pipeline: PhysicsPipeline,
    pub integration_parameters: IntegrationParameters,
    pub islands: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
}

impl PhysicsContext {
    fn new(gravity: Vector<f32>) -> Self {
        PhysicsContext {
            gravity,
            pipeline: PhysicsPipeline::default(),
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::default(),
            broad_phase: DefaultBroadPhase::default(),
            narrow_phase: NarrowPhase::default(),
            bodies: RigidBodySet::default(),
            colliders: ColliderSet::default(),
            impulse_joints: ImpulseJointSet::default(),
            multibody_joints: MultibodyJointSet::default(),
            ccd_solver: CCDSolver::default(),
            query_pipeline: QueryPipeline::default(),
        }
    }
}

pub fn init_physics(resources: &mut Resources) {
    resources.insert(PhysicsContext::new(vector![0.0, 9.8]));
}
