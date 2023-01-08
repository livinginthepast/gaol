use jail::JailError;
use rustler::Atom;

use crate::atoms;

pub fn to_atom(jail_error: JailError) -> Atom {
    match jail_error {
        JailError::CStringError(_) => atoms::cstring_error(),
        JailError::IoError(ref e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            atoms::permission_denied()
        }
        JailError::IoError(_) => atoms::io_error(),
        JailError::JailAttachError(_) => atoms::jail_attach_error(),
        JailError::JailGetError(_) => atoms::jail_get_error(),
        JailError::JailMaxAfIpsFailed(_) => atoms::jail_max_af_ips_failed(),
        JailError::JailRemoveFailed => atoms::jail_remove_failed(),
        JailError::JailSetError(_) => atoms::jail_set_error(),
        JailError::NoSuchParameter(_) => atoms::no_such_parameter(),
        JailError::ParameterLengthNaN(_) => atoms::parameter_length_nan(),
        JailError::ParameterStringLengthError(_) => atoms::parameter_string_length_error(),
        JailError::ParameterStructLengthError(_) => atoms::parameter_struct_length_error(),
        JailError::ParameterTunableError(_) => atoms::parameter_tunable_error(),
        JailError::ParameterTypeError(_) => atoms::parameter_type_error(),
        JailError::ParameterTypeUnsupported(_) => atoms::parameter_type_unsupported(),
        JailError::ParameterUnpackError => atoms::parameter_unpack_error(),
        JailError::PathNotGiven => atoms::path_not_given(),
        JailError::RctlError(_) => atoms::rctl_error(),
        JailError::SerializeFailed => atoms::serialize_failed(),
        JailError::SysctlError(_) => atoms::sysctl_error(),
        JailError::UnexpectedParameterType { .. } => atoms::unexpected_parameter_type(),
        JailError::UnnamedButLimited => atoms::unnamed_but_limited(),
    }
}
