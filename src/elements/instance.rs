use std::{fs, string::ToString};
use crate::elements::data_enum::*;

struct Instance{
    name:String,
    comment:String,
    dimension:u32,
    capacity: u32,
    edge_weight_type: EdgeWeightType,
    edge_weight_format: EdgeWeightFormat,
    edge_data_format: EdgeDataFormat,
    node_coord_type: NodeCoordType,
    display_data_type: DisplayDataType,
    nodes_coords:Vec<[f32;2]>
}



impl Instance{
    pub fn new(file_name:String)->Self{
        let s = Instance{
            name: "".to_string(),
            comment: "".to_string(),
            dimension: 0,
            capacity: 0,
            edge_weight_type: EdgeWeightType::None,
            edge_weight_format: EdgeWeightFormat::None,
            edge_data_format: EdgeDataFormat::None,
            display_data_type: DisplayDataType::NoDisplay,
            node_coord_type: NodeCoordType::NoCoords,
            nodes_coords: Vec::new(),
        };

        return s;
    }

    pub fn load_file(file_path:String){
        fs::read_to_string(file_path);
    }
}

