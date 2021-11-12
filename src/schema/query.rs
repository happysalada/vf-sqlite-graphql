use super::{Action, Agent, Commitment, Label, Plan, Process, ResourceSpecification, Unit};
use crate::Context;
use juniper::{graphql_object, FieldResult};
use sqlx::Row;
use std::collections::HashMap;

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Individuals")]
    async fn individuals(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents WHERE agents.agent_type == 'Individual' ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents.to_vec())
    }

    #[graphql(description = "Get all Organizations")]
    async fn organizations(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents WHERE agents.agent_type == 'Organization' ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents.to_vec())
    }

    #[graphql(description = "Get all Plans for an agent")]
    async fn plans(context: &Context, agent_unique_name: String) -> FieldResult<Vec<Plan>> {
        let plans =
            sqlx::query("SELECT plans.id, title, description, plans.inserted_at FROM plans JOIN plan_agents ON plan_agents.plan_id = plans.id WHERE plan_agents.agent_unique_name = ? ORDER BY plans.inserted_at DESC")
                .bind(agent_unique_name)
                .map(Plan::from_row)
                .fetch_all(&context.pool)
                .await?;
        Ok(plans.to_vec())
    }

    #[graphql(description = "Get a Plan")]
    async fn plan(context: &Context, plan_id: String) -> FieldResult<Plan> {
        let mut plan = sqlx::query("SELECT * FROM plans WHERE plans.id = ?")
            .bind(plan_id)
            .map(Plan::from_row)
            .fetch_one(&context.pool)
            .await?;
        let mut processes = sqlx::query("SELECT * FROM processes WHERE processes.plan_id = ? ")
            .bind(&plan.id)
            .map(Process::from_row)
            .fetch_all(&context.pool)
            .await?;
        let process_id_labels_tuples = sqlx::query(
            "
            SELECT labels.id, name, color, process_id
            FROM labels
            INNER JOIN process_labels
            ON process_labels.label_id = labels.id
            WHERE process_labels.process_id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("process_id"), Label::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let process_id_labels_hashmap: HashMap<String, Vec<Label>> =
            process_id_labels_tuples.iter().fold(
                HashMap::<String, Vec<Label>>::new(),
                |mut acc: HashMap<String, Vec<Label>>, (process_id, label): &(String, Label)| {
                    let labels = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                    labels.push(label.clone());
                    acc
                },
            );

        let process_id_agents_tuples = sqlx::query(
            "
            SELECT agents.id, name, unique_name, process_id
            FROM agents 
            INNER JOIN process_agents
            ON process_agents.agent_id = agents.id
            WHERE process_agents.process_id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("process_id"), Agent::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let process_id_agents_hashmap: HashMap<String, Vec<Agent>> =
            process_id_agents_tuples.iter().fold(
                HashMap::<String, Vec<Agent>>::new(),
                |mut acc: HashMap<String, Vec<Agent>>, (process_id, agent): &(String, Agent)| {
                    let agents = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                    agents.push(agent.clone());
                    acc
                },
            );
        let process_id_commitments_tuples = sqlx::query(
            "
            SELECT id, description, inserted_at, process_id, action_id, assigned_agent_id, quantity, unit_id, resource_specification_id
            FROM commitments 
            WHERE commitments.process_id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("process_id"), Commitment::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let process_id_commitments_hashmap: HashMap<String, Vec<Commitment>> =
            process_id_commitments_tuples.iter().fold(
                HashMap::<String, Vec<Commitment>>::new(),
                |mut acc: HashMap<String, Vec<Commitment>>,
                 (process_id, commitment): &(String, Commitment)| {
                    let commitments = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                    commitments.push(commitment.clone());
                    acc
                },
            );

        let commitment_id_action_tuples = sqlx::query(
            "
            SELECT actions.id, name, input_output, actions.inserted_at, commitments.id AS commitment_id
            FROM actions 
            JOIN commitments
            ON actions.id = commitments.action_id
            JOIN processes
            ON processes.id = commitments.process_id
            WHERE processes.id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("commitment_id"), Action::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let commitment_id_action_hashmap: HashMap<String, Action> =
            commitment_id_action_tuples.iter().fold(
                HashMap::<String, Action>::new(),
                |mut acc: HashMap<String, Action>, (commitment_id, action): &(String, Action)| {
                    let _ = acc
                        .entry(commitment_id.to_owned())
                        .or_insert_with(|| action.clone());
                    acc
                },
            );

        let commitment_id_resource_specification_tuples = sqlx::query(
            "
            SELECT resource_specifications.id, name, resource_specifications.inserted_at, commitments.id AS commitment_id
            FROM resource_specifications 
            JOIN commitments
            ON resource_specifications.id = commitments.resource_specification_id
            JOIN processes
            ON processes.id = commitments.process_id
            WHERE processes.id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| {
            (
                row.get("commitment_id"),
                ResourceSpecification::from_row(row),
            )
        })
        .fetch_all(&context.pool)
        .await?;
        let commitment_id_resource_specification_hashmap: HashMap<String, ResourceSpecification> =
            commitment_id_resource_specification_tuples.iter().fold(
                HashMap::<String, ResourceSpecification>::new(),
                |mut acc: HashMap<String, ResourceSpecification>, (commitment_id, resource_specification): &(String, ResourceSpecification)| {
                    let _ = acc.entry(commitment_id.to_owned()).or_insert_with(|| resource_specification.clone());
                    acc
                },
            );

        let commitment_id_unit_tuples = sqlx::query(
            "
            SELECT units.id, label, units.inserted_at, commitments.id AS commitment_id
            FROM units 
            JOIN commitments
            ON units.id = commitments.unit_id
            JOIN processes
            ON processes.id = commitments.process_id
            WHERE processes.id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("commitment_id"), Unit::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let commitment_id_unit_hashmap: HashMap<String, Unit> =
            commitment_id_unit_tuples.iter().fold(
                HashMap::<String, Unit>::new(),
                |mut acc: HashMap<String, Unit>, (commitment_id, unit): &(String, Unit)| {
                    let _ = acc
                        .entry(commitment_id.to_owned())
                        .or_insert_with(|| unit.clone());
                    acc
                },
            );

        let commitment_id_agent_tuples = sqlx::query(
            "
            SELECT agents.id, name, unique_name, agents.inserted_at, commitments.id AS commitment_id
            FROM agents 
            JOIN commitments
            ON agents.id = commitments.assigned_agent_id
            JOIN processes
            ON processes.id = commitments.process_id
            WHERE processes.id IN (
                SELECT id FROM processes
                WHERE processes.plan_id = ?
            )",
        )
        .bind(&plan.id)
        .map(|row| (row.get("commitment_id"), Agent::from_row(row)))
        .fetch_all(&context.pool)
        .await?;
        let commitment_id_agent_hashmap: HashMap<String, Agent> =
            commitment_id_agent_tuples.iter().fold(
                HashMap::<String, Agent>::new(),
                |mut acc: HashMap<String, Agent>, (commitment_id, agent): &(String, Agent)| {
                    let _ = acc
                        .entry(commitment_id.to_owned())
                        .or_insert_with(|| agent.clone());
                    acc
                },
            );

        processes.iter_mut().for_each(|p| {
            p.labels = process_id_labels_hashmap
                .get(&p.id)
                .unwrap_or(&vec![])
                .clone();
            p.agents = process_id_agents_hashmap
                .get(&p.id)
                .unwrap_or(&vec![])
                .clone();

            let mut commitments = process_id_commitments_hashmap
                .get(&p.id)
                .unwrap_or(&vec![])
                .clone();
            commitments.iter_mut().for_each(|c| {
                c.action = commitment_id_action_hashmap.get(&c.id).cloned();
                c.resource_specification = commitment_id_resource_specification_hashmap
                    .get(&c.id)
                    .cloned();
                c.unit = commitment_id_unit_hashmap.get(&c.id).cloned();
                c.assigned_agent = commitment_id_agent_hashmap.get(&c.id).cloned();
            });
            p.commitments = commitments;
        });
        plan.processes = processes;
        Ok(plan)
    }

    #[graphql(description = "Get all labels for an agent")]
    async fn labels(context: &Context, agent_unique_name: String) -> FieldResult<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>(
            "SELECT * FROM labels WHERE labels.agent_unique_name = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_unique_name)
        .fetch_all(&context.pool)
        .await?;
        Ok(labels.to_vec())
    }

    #[graphql(description = "Get all actions")]
    async fn actions(context: &Context) -> FieldResult<Vec<Action>> {
        let actions =
            sqlx::query_as::<_, Action>("SELECT * FROM actions ORDER BY inserted_at DESC")
                .fetch_all(&context.pool)
                .await?;
        Ok(actions.to_vec())
    }

    #[graphql(description = "Get all units")]
    async fn units(context: &Context) -> FieldResult<Vec<Unit>> {
        let units = sqlx::query_as::<_, Unit>("SELECT * FROM units ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(units.to_vec())
    }

    #[graphql(description = "Get all Resource specifications for an agent")]
    async fn resource_specifications(
        context: &Context,
        agent_unique_name: String,
    ) -> FieldResult<Vec<ResourceSpecification>> {
        let resource_specifications = sqlx::query_as::<_, ResourceSpecification>(
            "SELECT * FROM resource_specifications WHERE resource_specifications.agent_unique_name = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_unique_name)
        .fetch_all(&context.pool)
        .await?;
        Ok(resource_specifications.to_vec())
    }
}
