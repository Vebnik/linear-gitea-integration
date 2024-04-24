use crate::error::Result;

use super::types::BranchData;

pub async fn extract_branch_data(label: String) -> Result<BranchData> {
    let labeles = label.split("/").collect::<Vec<&str>>();

    let title = labeles[2].replace("-", " ");
    let branch_name = labeles[1].split("-").collect::<Vec<&str>>();

    Ok(BranchData {
        title,
        team_key: branch_name[0].to_string(),
        number: branch_name[1].parse().unwrap()
    })
}