#![allow(unused_variables)]

mod data;
use data::*;

use r2r::std_msgs::msg::ByteMultiArray as Message;
use r2r::WrappedTypesupport;
use rmw_sys::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use zenoh::prelude::sync::*;
use zenoh::SessionDeclarations;
type ROSMessage = <Message as WrappedTypesupport>::CStruct;

fn clone_cstr_pointer(ptr: *const c_char) -> *const c_char {
    let cstr = unsafe { CStr::from_ptr(ptr).to_owned() };
    Box::into_raw(cstr.into_boxed_c_str()) as *const _
}

fn topoic_to_keyexpr(topic_name: *const c_char) -> String {
    let ke = unsafe { CStr::from_ptr(clone_cstr_pointer(topic_name)) };
    format!("{}{}", RMW_ZENOH_TOPIC_PREFIX, ke.to_str().unwrap(),)
}

#[no_mangle]
static RMW_ZENOH_IDENTIFIER: &CStr = c"rmw_zenoh_rs";

#[no_mangle]
static RMW_ZENOH_TOPIC_PREFIX: &str = "rmw_zenoh_rs";

#[no_mangle]
pub extern "C" fn rmw_get_implementation_identifier() -> *const c_char {
    RMW_ZENOH_IDENTIFIER.as_ptr()
}

#[no_mangle]
pub extern "C" fn rmw_init_options_init(
    init_options: *mut rmw_init_options_t,
    allocator: rcutils_allocator_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_init_options_init");
    0
}

#[no_mangle]
pub extern "C" fn rmw_init_options_fini(init_options: *mut rmw_init_options_t) -> rmw_ret_t {
    0
}

#[no_mangle]
pub extern "C" fn rmw_init_options_copy(
    src: *const rmw_init_options_t,
    dst: *mut rmw_init_options_t,
) -> rmw_ret_t {
    0
}

#[no_mangle]
pub extern "C" fn rmw_init(
    options: *const rmw_init_options_t,
    context: *mut rmw_context_t,
) -> rmw_ret_t {
    let subscriber = tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_level(true)
        .with_target(true);

    let subscriber = subscriber.finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let context_impl = Box::new(rmw_context_impl_t {
        sess: zenoh::open(Config::default()).res().unwrap(),
    });

    unsafe {
        (*context).instance_id = options.read().instance_id;
        (*context).implementation_identifier = rmw_get_implementation_identifier();
        (*context).actual_domain_id = 0;
        (*context).impl_ = Box::into_raw(context_impl)
    }

    tracing::trace!("RMW Zenoh Initiated");
    0
}

#[no_mangle]
pub extern "C" fn rmw_create_node(
    context: *mut rmw_context_t,
    name: *const c_char,
    namespace_: *const c_char,
) -> *mut rmw_node_t {
    tracing::trace!("rmw_create_node");

    let node = Box::new(rmw_node_t {
        name: clone_cstr_pointer(name),
        namespace_: clone_cstr_pointer(namespace_),
        implementation_identifier: rmw_get_implementation_identifier(),
        context,
        data: std::ptr::null_mut(),
    });
    Box::into_raw(node)
}

#[no_mangle]
pub extern "C" fn rmw_borrow_loaned_message(
    publisher: *const rmw_publisher_t,
    type_support: *const rosidl_message_type_support_t,
    ros_message: *mut *mut ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_client_request_publisher_get_actual_qos(
    client: *const rmw_client_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_client_response_subscription_get_actual_qos(
    client: *const rmw_client_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    return rmw_client_request_publisher_get_actual_qos(client, qos);
}

#[no_mangle]
pub extern "C" fn rmw_client_set_on_new_response_callback(
    client: *mut rmw_client_t,
    callback: rmw_event_callback_t,
    user_data: *const ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_compare_gids_equal(
    gid1: *const rmw_gid_t,
    gid2: *const rmw_gid_t,
    result: *mut bool,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_context_fini(context: *mut rmw_context_t) -> rmw_ret_t {
    tracing::trace!("rmw_context_fini");
    0
}

#[no_mangle]
pub extern "C" fn rmw_count_clients(
    node: *const rmw_node_t,
    service_name: *const c_char,
    count: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_count_publishers(
    node: *const rmw_node_t,
    topic_name: *const c_char,
    count: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_count_services(
    node: *const rmw_node_t,
    service_name: *const c_char,
    count: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_count_subscribers(
    node: *const rmw_node_t,
    topic_name: *const c_char,
    count: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_create_client(
    node: *const rmw_node_t,
    type_support: *const rosidl_service_type_support_t,
    service_name: *const c_char,
    qos_policies: *const rmw_qos_profile_t,
) -> *mut rmw_client_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_create_guard_condition(
    context: *mut rmw_context_t,
) -> *mut rmw_guard_condition_t {
    tracing::trace!("rmw_create_guard_condition");
    let cond = Box::new(rmw_guard_condition_t {
        implementation_identifier: rmw_get_implementation_identifier(),
        context,
        data: std::ptr::null_mut(),
    });
    Box::into_raw(cond)
}

#[no_mangle]
pub extern "C" fn rmw_create_publisher(
    node: *const rmw_node_t,
    type_support: *const rosidl_message_type_support_t,
    topic_name: *const c_char,
    qos_profile: *const rmw_qos_profile_t,
    publisher_options: *const rmw_publisher_options_t,
) -> *mut rmw_publisher_t {
    tracing::trace!("rmw_create_publisher");

    let ke = topoic_to_keyexpr(topic_name);

    // declare a zenoh publisher
    let z_publisher = unsafe {
        (*(*(*node).context).impl_)
            .sess
            .declare_publisher(ke)
            .congestion_control(CongestionControl::Block)
            .res()
            .unwrap()
    };

    // store it in the PbulisherData
    let pub_data = Box::into_raw(Box::new(PublisherData {
        publisher: z_publisher,
    }));

    let publisher = Box::new(rmw_publisher_s {
        implementation_identifier: rmw_get_implementation_identifier(),
        topic_name: clone_cstr_pointer(topic_name),
        options: unsafe { publisher_options.read() },
        data: pub_data as _,
        can_loan_messages: false,
    });
    Box::into_raw(publisher)
}

#[no_mangle]
pub extern "C" fn rmw_create_service(
    node: *const rmw_node_t,
    type_support: *const rosidl_service_type_support_t,
    service_name: *const c_char,
    qos_profile: *const rmw_qos_profile_t,
) -> *mut rmw_service_t {
    tracing::trace!("rmw_create_service");
    let service = Box::new(rmw_service_s {
        implementation_identifier: rmw_get_implementation_identifier(),
        data: std::ptr::null_mut(),
        service_name: clone_cstr_pointer(service_name),
    });
    Box::into_raw(service)
}

#[no_mangle]
pub extern "C" fn rmw_create_wait_set(
    context: *mut rmw_context_t,
    max_conditions: usize,
) -> *mut rmw_wait_set_t {
    tracing::trace!("rmw_create_wait_set");
    let wait_set = Box::new(rmw_wait_set_t {
        implementation_identifier: rmw_get_implementation_identifier(),
        guard_conditions: &mut rmw_guard_conditions_s::default(),
        data: std::ptr::null_mut(),
    });
    Box::into_raw(wait_set)
}

#[no_mangle]
pub extern "C" fn rmw_destroy_client(
    node: *mut rmw_node_t,
    client: *mut rmw_client_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_destroy_guard_condition(
    guard_condition: *mut rmw_guard_condition_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_destroy_guard_condition");
    0
}

#[no_mangle]
pub extern "C" fn rmw_destroy_node(node: *mut rmw_node_t) -> rmw_ret_t {
    0
}

#[no_mangle]
pub extern "C" fn rmw_destroy_service(
    node: *mut rmw_node_t,
    service: *mut rmw_service_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_destroy_subscription(
    node: *mut rmw_node_t,
    subscription: *mut rmw_subscription_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_destroy_subscription");
    0
}

#[no_mangle]
pub extern "C" fn rmw_destroy_wait_set(wait_set: *mut rmw_wait_set_t) -> rmw_ret_t {
    tracing::trace!("rmw_destroy_wait_set");
    0
}

#[no_mangle]
pub extern "C" fn rmw_fini_subscription_allocation(
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_fini_publisher_allocation(
    allocation: *mut rmw_publisher_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_feature_supported(feature: rmw_feature_t) -> bool {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_event_set_callback(
    event: *mut rmw_event_t,
    callback: rmw_event_callback_t,
    user_data: *const ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_destroy_publisher(
    node: *mut rmw_node_t,
    publisher: *mut rmw_publisher_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_destroy_publisher");
    0
}

#[no_mangle]
pub extern "C" fn rmw_deserialize(
    serialized_message: *const rmw_serialized_message_t,
    type_support: *const rosidl_message_type_support_t,
    ros_message: *mut ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_create_subscription(
    node: *const rmw_node_t,
    type_support: *const rosidl_message_type_support_t,
    topic_name: *const c_char,
    qos_policies: *const rmw_qos_profile_t,
    subscription_options: *const rmw_subscription_options_t,
) -> *mut rmw_subscription_t {
    tracing::trace!("rmw_create_subscription");

    let ke = topoic_to_keyexpr(topic_name);

    // declare a zenoh subscriber
    let z_subscriber = unsafe {
        (*(*(*node).context).impl_)
            .sess
            .declare_subscriber(ke)
            // .with(flume::unbounded())
            .reliable()
            .res()
            .unwrap()
    };

    // store it in the SubscriberData
    let sub_data = Box::into_raw(Box::new(SubscriberData {
        subscriber: z_subscriber,
    }));

    let rmw_sub = rmw_subscription_t {
        implementation_identifier: rmw_get_implementation_identifier(),
        topic_name: clone_cstr_pointer(topic_name),
        can_loan_messages: false,
        is_cft_enabled: false,
        options: unsafe { subscription_options.read() },
        data: sub_data as _,
    };
    Box::into_raw(Box::new(rmw_sub))
}

#[no_mangle]
pub extern "C" fn rmw_get_serialized_message_size(
    type_support: *const rosidl_message_type_support_t,
    message_bounds: *const rosidl_runtime_c__Sequence__bound,
    size: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_serialization_format() -> *const c_char {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_publishers_info_by_topic(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    topic_name: *const c_char,
    no_mangle: bool,
    publishers_info: *mut rmw_topic_endpoint_info_array_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_get_publishers_info_by_topic");
    0
}

#[no_mangle]
pub extern "C" fn rmw_get_publisher_names_and_types_by_node(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    node_name: *const c_char,
    node_namespace: *const c_char,
    no_demangle: bool,
    topic_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_node_names_with_enclaves(
    node: *const rmw_node_t,
    node_names: *mut rcutils_string_array_t,
    node_namespaces: *mut rcutils_string_array_t,
    enclaves: *mut rcutils_string_array_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_node_names(
    node: *const rmw_node_t,
    node_names: *mut rcutils_string_array_t,
    node_namespaces: *mut rcutils_string_array_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_gid_for_publisher(
    publisher: *const rmw_publisher_t,
    gid: *mut rmw_gid_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_get_gid_for_publisher");
    0
}

#[no_mangle]
pub extern "C" fn rmw_get_gid_for_client(
    client: *const rmw_client_t,
    gid: *mut rmw_gid_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_client_names_and_types_by_node(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    node_name: *const c_char,
    node_namespace: *const c_char,
    service_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_wait(
    subscriptions: *mut rmw_subscriptions_t,
    guard_conditions: *mut rmw_guard_conditions_t,
    services: *mut rmw_services_t,
    clients: *mut rmw_clients_t,
    events: *mut rmw_events_t,
    wait_set: *mut rmw_wait_set_t,
    wait_timeout: *const rmw_time_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_wait");
    0
}

#[no_mangle]
pub extern "C" fn rmw_trigger_guard_condition(
    guard_condition: *const rmw_guard_condition_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_trigger_guard_condition");
    0
}

#[no_mangle]
pub extern "C" fn rmw_take_with_info(
    subscription: *const rmw_subscription_t,
    ros_message: *mut ::std::os::raw::c_void,
    taken: *mut bool,
    message_info: *mut rmw_message_info_t,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_take_with_info");

    let sub: &zenoh::subscriber::FlumeSubscriber = unsafe { std::mem::transmute((*subscription).data) };

    if !sub.is_empty() {
        let sample = sub.recv().unwrap();
        let msg: Message = bincode::deserialize(&sample.payload.contiguous()).unwrap();

        unsafe {
            let ros_msg = Message::create_msg();
            msg.copy_to_native(ros_msg.as_mut().expect("not null"));
            std::ptr::write(
                ros_message as *mut ROSMessage,
                std::ptr::read(ros_msg),
            );
            *taken = true;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn rmw_take_serialized_message_with_info(
    subscription: *const rmw_subscription_t,
    serialized_message: *mut rmw_serialized_message_t,
    taken: *mut bool,
    message_info: *mut rmw_message_info_t,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_serialized_message(
    subscription: *const rmw_subscription_t,
    serialized_message: *mut rmw_serialized_message_t,
    taken: *mut bool,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_response(
    client: *const rmw_client_t,
    request_header: *mut rmw_service_info_t,
    ros_response: *mut ::std::os::raw::c_void,
    taken: *mut bool,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_request(
    service: *const rmw_service_t,
    request_header: *mut rmw_service_info_t,
    ros_request: *mut ::std::os::raw::c_void,
    taken: *mut bool,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_loaned_message_with_info(
    subscription: *const rmw_subscription_t,
    loaned_message: *mut *mut ::std::os::raw::c_void,
    taken: *mut bool,
    message_info: *mut rmw_message_info_t,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_loaned_message(
    subscription: *const rmw_subscription_t,
    loaned_message: *mut *mut ::std::os::raw::c_void,
    taken: *mut bool,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_event(
    event_handle: *const rmw_event_t,
    event_info: *mut ::std::os::raw::c_void,
    taken: *mut bool,
) -> rmw_ret_t {
    tracing::trace!("rmw_take_event");
    unsafe {
        *taken = true;
    }
    0
}

#[no_mangle]
pub extern "C" fn rmw_take_dynamic_message(
    subscription: *const rmw_subscription_t,
    dynamic_message: *mut rosidl_dynamic_typesupport_dynamic_data_t,
    taken: *mut bool,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take_dynamic_message_with_info(
    subscription: *const rmw_subscription_t,
    dynamic_message: *mut rosidl_dynamic_typesupport_dynamic_data_t,
    taken: *mut bool,
    message_info: *mut rmw_message_info_t,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_take(
    subscription: *const rmw_subscription_t,
    ros_message: *mut ::std::os::raw::c_void,
    taken: *mut bool,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_subscription_set_on_new_message_callback(
    subscription: *mut rmw_subscription_t,
    callback: rmw_event_callback_t,
    user_data: *const ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_subscription_set_content_filter(
    subscription: *mut rmw_subscription_t,
    options: *const rmw_subscription_content_filter_options_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_subscription_get_content_filter(
    subscription: *const rmw_subscription_t,
    allocator: *mut rcutils_allocator_t,
    options: *mut rmw_subscription_content_filter_options_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_subscription_get_actual_qos(
    subscription: *const rmw_subscription_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_subscription_get_actual_qos");
    0
}

#[no_mangle]
pub extern "C" fn rmw_subscription_event_init(
    rmw_event: *mut rmw_event_t,
    subscription: *const rmw_subscription_t,
    event_type: rmw_event_type_t,
) -> rmw_ret_t {
    // Couldn't add event to wait set: event's implementation not init, at ./src/rcl/event.c:225, at ./src/rcl/wait.c:516
    unsafe {
        (*rmw_event).data = Box::into_raw(Box::new(12345)) as _;
        (*rmw_event).event_type = event_type;
    }
    tracing::trace!("rmw_subscription_event_init");
    0
}

#[no_mangle]
pub extern "C" fn rmw_subscription_count_matched_publishers(
    subscription: *const rmw_subscription_t,
    publisher_count: *mut usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_shutdown(context: *mut rmw_context_t) -> rmw_ret_t {
    tracing::trace!("RMW Zenoh is down.");
    0
}

#[no_mangle]
pub extern "C" fn rmw_set_log_severity(severity: rmw_log_severity_t) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_service_set_on_new_request_callback(
    service: *mut rmw_service_t,
    callback: rmw_event_callback_t,
    user_data: *const ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_service_server_is_available(
    node: *const rmw_node_t,
    client: *const rmw_client_t,
    is_available: *mut bool,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_service_response_publisher_get_actual_qos(
    service: *const rmw_service_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_service_response_publisher_get_actual_qos");
    0
}

#[no_mangle]
pub extern "C" fn rmw_service_request_subscription_get_actual_qos(
    service: *const rmw_service_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_service_request_subscription_get_actual_qos");
    0
}

#[no_mangle]
pub extern "C" fn rmw_serialize(
    ros_message: *const ::std::os::raw::c_void,
    type_support: *const rosidl_message_type_support_t,
    serialized_message: *mut rmw_serialized_message_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_serialization_support_init(
    serialization_lib_name: *const c_char,
    allocator: *mut rcutils_allocator_t,
    serialization_support: *mut rosidl_dynamic_typesupport_serialization_support_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_send_response(
    service: *const rmw_service_t,
    request_header: *mut rmw_request_id_t,
    ros_response: *mut ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_send_request(
    client: *const rmw_client_t,
    ros_request: *const ::std::os::raw::c_void,
    sequence_id: *mut i64,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_return_loaned_message_from_subscription(
    subscription: *const rmw_subscription_t,
    loaned_message: *mut ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_return_loaned_message_from_publisher(
    publisher: *const rmw_publisher_t,
    loaned_message: *mut ::std::os::raw::c_void,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_qos_profile_check_compatible(
    publisher_profile: rmw_qos_profile_t,
    subscription_profile: rmw_qos_profile_t,
    compatibility: *mut rmw_qos_compatibility_type_t,
    reason: *mut c_char,
    reason_size: usize,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publisher_wait_for_all_acked(
    publisher: *const rmw_publisher_t,
    wait_timeout: rmw_time_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publisher_get_actual_qos(
    publisher: *const rmw_publisher_t,
    qos: *mut rmw_qos_profile_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_publisher_get_actual_qos");
    0
}

#[no_mangle]
pub extern "C" fn rmw_publisher_event_init(
    rmw_event: *mut rmw_event_t,
    publisher: *const rmw_publisher_t,
    event_type: rmw_event_type_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_publisher_event_init");
    // Couldn't add event to wait set: event's implementation not init, at ./src/rcl/event.c:225, at ./src/rcl/wait.c:516
    unsafe {
        (*rmw_event).data = Box::into_raw(Box::new(12345)) as _;
        (*rmw_event).event_type = event_type;
    }
    0
}

#[no_mangle]
pub extern "C" fn rmw_publisher_count_matched_subscriptions(
    publisher: *const rmw_publisher_t,
    subscription_count: *mut usize,
) -> rmw_ret_t {
    tracing::trace!("rmw_publisher_count_matched_subscriptions");
    0
}

#[no_mangle]
pub extern "C" fn rmw_publisher_assert_liveliness(publisher: *const rmw_publisher_t) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publish_serialized_message(
    publisher: *const rmw_publisher_t,
    serialized_message: *const rmw_serialized_message_t,
    allocation: *mut rmw_publisher_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publish_loaned_message(
    publisher: *const rmw_publisher_t,
    ros_message: *mut ::std::os::raw::c_void,
    allocation: *mut rmw_publisher_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publish(
    publisher: *const rmw_publisher_t,
    ros_message: *const ::std::os::raw::c_void,
    allocation: *mut rmw_publisher_allocation_t,
) -> rmw_ret_t {
    tracing::trace!("rmw_publish");
    let pub_: &zenoh::publication::Publisher = unsafe { std::mem::transmute((*publisher).data) };

    let msg = unsafe {
        let msg: &ROSMessage = std::mem::transmute(ros_message);
        Message::from_native(msg)
    };

    let payload = bincode::serialize(&msg).unwrap();
    pub_.put(payload).res().unwrap();
    0
}

#[no_mangle]
pub extern "C" fn rmw_node_get_graph_guard_condition(
    node: *const rmw_node_t,
) -> *const rmw_guard_condition_t {
    &mut rmw_guard_condition_s::default()
}

#[no_mangle]
pub extern "C" fn rmw_init_subscription_allocation(
    type_support: *const rosidl_message_type_support_t,
    message_bounds: *const rosidl_runtime_c__Sequence__bound,
    allocation: *mut rmw_subscription_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_init_publisher_allocation(
    type_support: *const rosidl_message_type_support_t,
    message_bounds: *const rosidl_runtime_c__Sequence__bound,
    allocation: *mut rmw_publisher_allocation_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_topic_names_and_types(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    no_demangle: bool,
    topic_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_subscriptions_info_by_topic(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    topic_name: *const c_char,
    no_mangle: bool,
    subscriptions_info: *mut rmw_topic_endpoint_info_array_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_subscriber_names_and_types_by_node(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    node_name: *const c_char,
    node_namespace: *const c_char,
    no_demangle: bool,
    topic_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_service_names_and_types_by_node(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    node_name: *const c_char,
    node_namespace: *const c_char,
    service_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_get_service_names_and_types(
    node: *const rmw_node_t,
    allocator: *mut rcutils_allocator_t,
    service_names_and_types: *mut rmw_names_and_types_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_publisher_get_network_flow_endpoints(
    publisher: *const rmw_publisher_t,
    allocator: *mut rcutils_allocator_t,
    network_flow_endpoint_array: *mut rmw_network_flow_endpoint_array_t,
) -> rmw_ret_t {
    todo!()
}

#[no_mangle]
pub extern "C" fn rmw_subscription_get_network_flow_endpoints(
    subscription: *const rmw_subscription_t,
    allocator: *mut rcutils_allocator_t,
    network_flow_endpoint_array: *mut rmw_network_flow_endpoint_array_t,
) -> rmw_ret_t {
    todo!()
}
