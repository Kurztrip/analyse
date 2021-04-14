use crate::repositories;
use crate::repositories::DBConnection;
use crate::data::package_models::Package;
use crate::logic::errors::LogicError;

pub fn generate_route(conn:&DBConnection, warehouse_id:i32)->Result<(),LogicError>{
    let warehouse = repositories::warehouses::get(conn,warehouse_id)?;
    let trucks = repositories::trucks::get_all_from_warehouse(conn,warehouse_id)?;
    for truck in trucks{
        let mut packages : Vec<Package> = Vec::new();
        let mut weight=truck.weight_capacity();
        let mut volume=truck.volume_capacity();
        let mut next = repositories::packages::get_all_near_from_warehouse(conn,warehouse_id,warehouse.location())?;
        while weight>0.1 && volume>0.5 && next.len()>0{
            if weight-&next[0].weight()>0.0 && volume-&next[0].volume()>0.0{
                packages.push(next[0].clone());
                repositories::trucks::add_to_route(conn,next[0].clone(),truck.id())?;
                // repositories::packages::delete(conn,next[0].id());
                weight=weight-&next[0].weight();
                volume=volume-&next[0].volume();
                next = repositories::packages::get_all_near_from_warehouse(conn,warehouse_id,next[0].destination())?;
            }else{
                next.remove(0);
            }
        }
        println!("{:?}",packages);
    }
    Ok(())
}