#[get("/<organization_id>/projects")]
pub fn projects(organization_id: &str) -> String {
    format!(
        "Hello from Shield, this is Projects belongs to org_id: {}",
        organization_id
    )
}

#[get("/<organization_id>/projects/<project_id>")]
pub fn project(organization_id: &str, project_id: &str) -> String {
    format!(
        "Hello from Shield, Project id: {} belongs to org_id: {}",
        project_id, organization_id
    )
}
