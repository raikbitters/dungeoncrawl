use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(WantsToAttack)]
#[read_component(Player)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
            
        {
            health.current -= 1;
            
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}