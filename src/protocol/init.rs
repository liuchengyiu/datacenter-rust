use super::register::{
    register_func
};
use super::deals::{
    version,
    model,
    register,
    data
};


pub fn register_static_func() {
    register_func("+/get/request/database/version", 
                  &version::index::response_version);
    register_func("+/set/request/database/model",
                  &model::index::response_model_set);
    register_func("+/get/request/database/model",
                  &model::index::response_model_get);
    register_func("+/get/request/database/modelschema",
                  &model::index::response_model_schema);
    register_func("+/action/request/database/deletemodel",
                  &model::index::response_model_delete);
    register_func("+/set/request/database/register",
                  &register::index::response_device_register);
    register_func("+/get/request/database/register",
                  &register::index::response_device_register_get);
    register_func("+/get/request/database/guid",
                  &register::index::response_device_guid_get);
    register_func("+/action/request/database/unregister",
                  &register::index::response_device_cancel_register);
    register_func("+/notify/event/database/+/+",
                  &data::realtime::response_realtime_set);
    register_func("+/get/request/database/realtime",
                  &data::realtime::response_realtime_get);
}

