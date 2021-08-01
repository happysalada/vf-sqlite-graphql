use super::{Action, Agent, Label, Plan, Process, Unit};
use crate::Context;
use juniper::{graphql_object, FieldResult};
use sqlx::Row;
use std::collections::HashMap;

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Agents")]
    async fn agents(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents.to_vec())
    }

    #[graphql(description = "Get all Plans for an agent")]
    async fn plans(context: &Context, agent_id: String) -> FieldResult<Vec<Plan>> {
        let plans =
            sqlx::query("SELECT plans.id, title, description, plans.inserted_at FROM plans JOIN plan_agents ON plan_agents.plan_id = plans.id WHERE plan_agents.agent_id = ? ORDER BY plans.inserted_at DESC")
                .bind(agent_id)
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
        let labels_process_id = sqlx::query(
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
        let labels_plan_id_map: HashMap<String, Vec<Label>> = labels_process_id.iter().fold(
            HashMap::<String, Vec<Label>>::new(),
            |mut acc: HashMap<String, Vec<Label>>, (process_id, label): &(String, Label)| {
                let labels = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                labels.push(label.clone());
                acc
            },
        );

        let agents_process_id = sqlx::query(
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
        let agents_plan_id_map: HashMap<String, Vec<Agent>> = agents_process_id.iter().fold(
            HashMap::<String, Vec<Agent>>::new(),
            |mut acc: HashMap<String, Vec<Agent>>, (process_id, agent): &(String, Agent)| {
                let agents = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                agents.push(agent.clone());
                acc
            },
        );

        processes.iter_mut().for_each(|p| {
            p.labels = labels_plan_id_map.get(&p.id).unwrap_or(&vec![]).clone();
            p.agents = agents_plan_id_map.get(&p.id).unwrap_or(&vec![]).clone();
        });
        plan.processes = processes;
        Ok(plan)
    }

    #[graphql(description = "Get all labels for an agent")]
    async fn labels(context: &Context, agent_id: String) -> FieldResult<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>(
            "SELECT * FROM labels WHERE labels.agent_id = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_id)
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
}
