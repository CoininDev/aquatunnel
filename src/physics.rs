use rapier2d::prelude::*;

pub struct PhysicsContext {
    pub pipeline: PhysicsPipeline,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub islands: IslandManager,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub gravity: Vector<Real>,
    pub integration_params: IntegrationParameters,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub ccd_solver: CCDSolver,
    pub hooks: (),
    pub events: (),
}

impl Default for PhysicsContext {
    fn default() -> Self {
        PhysicsContext {
            pipeline: PhysicsPipeline::new(),
            bodies: RigidBodySet::default(),
            colliders: ColliderSet::default(),
            gravity: vector![0.0, 9.81],
            islands: IslandManager::default(),
            impulse_joints: ImpulseJointSet::default(),
            multibody_joints: MultibodyJointSet::default(),
            integration_params: IntegrationParameters::default(),
            broad_phase: DefaultBroadPhase::default(),
            narrow_phase: NarrowPhase::default(),
            ccd_solver: CCDSolver::default(),
            hooks: (),
            events: (),
        }
    }
}
