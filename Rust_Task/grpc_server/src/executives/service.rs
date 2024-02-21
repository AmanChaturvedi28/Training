use super::{executive::ExecutiveData, model::{GRPCExecutive, EXECUTIVE_DATA}};

pub async fn read_executive(id: i32) -> Option<String> {
    let executives = EXECUTIVE_DATA.read().unwrap();
    let executive = executives.iter().find(|&s| s.id == id);
    match executive {
        Some(executive) => Some(serde_json::to_string(executive).unwrap()),
        None => None,
    }
}

pub async fn read_all_executive() -> Option<String> {
    let executives = EXECUTIVE_DATA.read().unwrap();
    if executives.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&*executives).unwrap())
    }
}

pub async fn delete_executive(id: i32) -> Option<String> {
    let mut executives = EXECUTIVE_DATA.write().unwrap();
    if let Some(index) = executives.iter().position(|s| s.id == id) {
        let deleted_executive = executives.remove(index);
        Some(serde_json::to_string(&deleted_executive).unwrap())
    } else {
        None
    }
}

pub async fn create_executive(data: ExecutiveData) -> Result<String, String> {
    let mut executives = EXECUTIVE_DATA.write().unwrap();

    if executives.iter().any(|executive| executive.id == data.id) {
        return Err("Executive with the same ID already exists".to_string());
    }

    let grpc_executive = map_structure(data);

    let str_data = serde_json::to_string(&grpc_executive).unwrap();

    executives.push(grpc_executive);

    Ok(str_data)
}

pub async fn update_executive(updated_data: ExecutiveData) -> Result<String, String> {
    let mut executives = EXECUTIVE_DATA.write().unwrap();
    if let Some(index) = executives.iter().position(|s| s.id == updated_data.id) {
        executives[index].name = updated_data.name;
        executives[index].skills = updated_data.skills;
        executives[index].status = updated_data.status;
        executives[index].language = updated_data.language;

        Ok(serde_json::to_string(&executives[index]).unwrap())
    } else {
        if executives.iter().any(|executive| executive.id == updated_data.id) {
            return Err("Employee with the same ID already exists".to_string());
        }

        let grpc_executive = map_structure(updated_data);

        let str_data = serde_json::to_string(&grpc_executive).unwrap();

        executives.push(grpc_executive);

        Ok(str_data)
    }
}

fn map_structure(data: ExecutiveData) -> GRPCExecutive {
    let data = GRPCExecutive {
        id: data.id,
        name: data.name,
        skills: data.skills,
        status: data.status,
        language: data.language
    };
    data
}