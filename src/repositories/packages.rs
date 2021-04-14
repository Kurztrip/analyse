use crate::data::package_models::Package;
use crate::repositories::DBConnection;
use crate::data::Coordinates;
use crate::logic::errors::LogicError;
use serde_json::Value;
use super::serde_json::{Map, Number};
use crate::repositories::database_models::{DatabasePackage, DatabaseCoordinates};


const COLLECTION:&str="packages";

pub fn add(conn: &DBConnection, package:Package) ->Result<Package,LogicError> {
    let id = conn.add_one_to_db(
        COLLECTION,
        DatabasePackage::from(package))?;
    get(conn, id)
}
pub fn get_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<Vec<Package>,LogicError>{
    let mut filter = Map::new();
    filter.insert("warehouse".to_string(),Value::Number(Number::from(warehouse_id)));
    let result :Vec<DatabasePackage>= conn.get_many_from_db(COLLECTION, Some(filter))?;
    Ok(result.iter().map(|p|p.to_owned().into()).collect())
}
pub fn get_all_near_from_warehouse(conn: &DBConnection, warehouse_id:i32, coordinates:&Coordinates) ->Result<Vec<Package>,LogicError>{
    let mut filter = Map::new();
    filter.insert("warehouse".to_string(),Value::Number(Number::from(warehouse_id)));
    conn.get_from_near(COLLECTION,
                       Some(filter),
                       "destination".to_string(),
                       DatabaseCoordinates::from(coordinates.clone())
    )
}
pub fn get_all(conn: &DBConnection) ->Result<Vec<Package>,LogicError>{
    let result:Vec<DatabasePackage>=conn.get_many_from_db(COLLECTION,None)?;
    Ok(result.iter().map(|p|p.to_owned().into()).collect())
}
pub fn get(conn: &DBConnection, package_id:i32) -> Result<Package,LogicError>{
    let mut filter = Map::new();
    filter.insert("id".to_string(),Value::Number(Number::from(package_id)));
    let result:DatabasePackage = conn.get_one_from_db(COLLECTION, Some(filter))?;
    Ok(result.into())
}
pub fn update(conn: &DBConnection, package:Package, package_id:i32) ->Result<Package,LogicError>{
    let result = conn.replace_in_db(
        COLLECTION,
        package_id,
        DatabasePackage::from(package)
    )?;
    Ok(result.into())
}
pub fn delete(conn: &DBConnection, package_id:i32) -> Result<i8,LogicError>{
    conn.delete_from_db(COLLECTION, package_id)
}