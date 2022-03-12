use super::{
    Action, Agent, AgentRelationship, Commitment, Label, Plan, Process, ResourceSpecification, Unit,
};

use async_graphql::{Context, Object, Result};
use sqlx::{sqlite::SqlitePool, Row};
use std::collections::{HashMap, HashSet};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn agents<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Agent>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(agents.to_vec())
    }

    async fn individuals<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Agent>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents WHERE agents.agent_type == 'Individual' ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(agents.to_vec())
    }

    async fn organizations<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Agent>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents WHERE agents.agent_type == 'Organization' ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(agents.to_vec())
    }

    async fn agent_relations<'ctx>(
        &self,
        context: &Context<'ctx>,
        agent_id: String,
    ) -> Result<Vec<AgentRelationship>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let mut relations = sqlx::query("
            SELECT agent_relations.id, subject_id, object_id, agent_relation_types.name AS agent_relation_type_name FROM agent_relations
            JOIN agent_relation_types ON agent_relation_types.id = agent_relations.agent_relation_type_id 
            WHERE subject_id = ? OR object_id = ?
            ORDER BY agent_relations.inserted_at DESC
        ")
            .bind(&agent_id)
            .bind(&agent_id)
            .map(AgentRelationship::from_row)
            .fetch_all(pool)
            .await?;
        let agent_id_set: HashSet<String> = relations.iter().fold(
            HashSet::<String>::new(),
            |mut acc: HashSet<String>, relationship: &AgentRelationship| {
                acc.insert(relationship.subject_id.to_owned());
                acc.insert(relationship.object_id.to_owned());
                acc
            },
        );
        let agent_ids = agent_id_set.into_iter().collect::<Vec<_>>().join("', '");
        let sql = format!(
            "SELECT * FROM agents WHERE id IN ('{}') ORDER BY inserted_at DESC",
            &agent_ids
        );
        let agents = sqlx::query_as::<_, Agent>(&sql)
            .bind(agent_ids)
            .fetch_all(pool)
            .await?;
        let agents_hashmap: HashMap<&String, &Agent> = agents.iter().fold(
            HashMap::<&String, &Agent>::new(),
            |mut acc: HashMap<&String, &Agent>, agent: &Agent| {
                acc.entry(&agent.id).or_insert_with(|| agent);
                acc
            },
        );
        relations.iter_mut().for_each(|p| {
            p.subject = agents_hashmap
                .get(&p.subject_id)
                .expect(&format!(
                    "subject {} missing for relationship id {}",
                    &p.subject_id, &p.id
                ))
                .to_owned()
                .to_owned();
            p.object = agents_hashmap
                .get(&p.object_id)
                .expect(&format!(
                    "object {} missing for relationship id {}",
                    &p.object_id, &p.id
                ))
                .to_owned()
                .to_owned();
        });
        Ok(relations.to_vec())
    }

    async fn plans<'ctx>(&self, context: &Context<'ctx>, agent_id: String) -> Result<Vec<Plan>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let plans =
            sqlx::query("SELECT plans.id, title, description, plans.inserted_at FROM plans JOIN plan_agents ON plan_agents.plan_id = plans.id WHERE plan_agents.agent_id = ? ORDER BY plans.inserted_at DESC")
                .bind(agent_id)
                .map(Plan::from_row)
                .fetch_all(pool)
                .await?;
        Ok(plans.to_vec())
    }

    async fn plan<'ctx>(&self, context: &Context<'ctx>, plan_id: String) -> Result<Plan> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let mut plan = sqlx::query("SELECT * FROM plans WHERE plans.id = ?")
            .bind(plan_id)
            .map(Plan::from_row)
            .fetch_one(pool)
            .await?;
        let mut processes = sqlx::query("SELECT * FROM processes WHERE processes.plan_id = ? ")
            .bind(&plan.id)
            .map(Process::from_row)
            .fetch_all(pool)
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
        .fetch_all(pool)
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
            SELECT agents.id, name, unique_name, agent_type, process_id
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
        .fetch_all(pool)
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
        .fetch_all(pool)
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
        .fetch_all(pool)
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
        .fetch_all(pool)
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
        .fetch_all(pool)
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
        .fetch_all(pool)
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

    async fn labels<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Label>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let labels = sqlx::query_as::<_, Label>("SELECT * FROM labels ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(labels.to_vec())
    }

    async fn actions<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Action>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let actions =
            sqlx::query_as::<_, Action>("SELECT * FROM actions ORDER BY inserted_at DESC")
                .fetch_all(pool)
                .await?;
        Ok(actions.to_vec())
    }

    async fn units<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Unit>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let units = sqlx::query_as::<_, Unit>("SELECT * FROM units ORDER BY inserted_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(units.to_vec())
    }

    async fn resource_specifications<'ctx>(
        &self,
        context: &Context<'ctx>,
    ) -> Result<Vec<ResourceSpecification>> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let resource_specifications = sqlx::query_as::<_, ResourceSpecification>(
            "SELECT * FROM resource_specifications ORDER BY inserted_at DESC",
        )
        .fetch_all(pool)
        .await?;
        Ok(resource_specifications.to_vec())
    }
}
