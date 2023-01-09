rustler::atoms! {
    ok,
    error,

    // jail
    hostname,
    jid,
    name,
    params,
    path,
    native,

    // errors
    cstring_error,
    io_error,
    jail_attach_error,
    jail_get_error,
    jail_max_af_ips_failed,
    jail_remove_failed,
    jail_set_error,
    no_such_file_or_directory,
    no_such_parameter,
    not_found,
    parameter_length_nan,
    parameter_string_length_error,
    parameter_struct_length_error,
    parameter_tunable_error,
    parameter_type_error,
    parameter_type_unsupported,
    parameter_unpack_error,
    path_not_given,
    permission_denied,
    rctl_error,
    serialize_failed,
    sysctl_error,
    unexpected_parameter_type,
    unnamed_but_limited,
}
