use crate::Handler;

wit_bindgen::generate!({
    world: "passthrough",
    path: "../../wit",
    generate_all,
});

export!(Handler);

//impl From<wasi::filesystem::types::DescriptorType> for DescriptorType {
//    fn from(value: wasi::filesystem::types::DescriptorType) -> Self {
//        match value {
//            wasi::filesystem::types::DescriptorType::Unknown => DescriptorType::Unknown,
//            wasi::filesystem::types::DescriptorType::BlockDevice => DescriptorType::BlockDevice,
//            wasi::filesystem::types::DescriptorType::CharacterDevice => {
//                DescriptorType::CharacterDevice
//            }
//            wasi::filesystem::types::DescriptorType::Directory => DescriptorType::Directory,
//            wasi::filesystem::types::DescriptorType::Fifo => DescriptorType::Fifo,
//            wasi::filesystem::types::DescriptorType::SymbolicLink => DescriptorType::SymbolicLink,
//            wasi::filesystem::types::DescriptorType::RegularFile => DescriptorType::RegularFile,
//            wasi::filesystem::types::DescriptorType::Socket => DescriptorType::Socket,
//        }
//    }
//}
//
//impl From<wasi::filesystem::types::ErrorCode> for ErrorCode {
//    fn from(value: wasi::filesystem::types::ErrorCode) -> Self {
//        match value {
//            wasi::filesystem::types::ErrorCode::Access => ErrorCode::Access,
//            wasi::filesystem::types::ErrorCode::WouldBlock => ErrorCode::WouldBlock,
//            wasi::filesystem::types::ErrorCode::Already => ErrorCode::Already,
//            wasi::filesystem::types::ErrorCode::BadDescriptor => ErrorCode::BadDescriptor,
//            wasi::filesystem::types::ErrorCode::Busy => ErrorCode::Busy,
//            wasi::filesystem::types::ErrorCode::Deadlock => ErrorCode::Deadlock,
//            wasi::filesystem::types::ErrorCode::Quota => ErrorCode::Quota,
//            wasi::filesystem::types::ErrorCode::Exist => ErrorCode::Exist,
//            wasi::filesystem::types::ErrorCode::FileTooLarge => ErrorCode::FileTooLarge,
//            wasi::filesystem::types::ErrorCode::IllegalByteSequence => {
//                ErrorCode::IllegalByteSequence
//            }
//            wasi::filesystem::types::ErrorCode::InProgress => ErrorCode::InProgress,
//            wasi::filesystem::types::ErrorCode::Interrupted => ErrorCode::Interrupted,
//            wasi::filesystem::types::ErrorCode::Invalid => ErrorCode::Invalid,
//            wasi::filesystem::types::ErrorCode::Io => ErrorCode::Io,
//            wasi::filesystem::types::ErrorCode::IsDirectory => ErrorCode::IsDirectory,
//            wasi::filesystem::types::ErrorCode::Loop => ErrorCode::Loop,
//            wasi::filesystem::types::ErrorCode::TooManyLinks => ErrorCode::TooManyLinks,
//            wasi::filesystem::types::ErrorCode::MessageSize => ErrorCode::MessageSize,
//            wasi::filesystem::types::ErrorCode::NameTooLong => ErrorCode::NameTooLong,
//            wasi::filesystem::types::ErrorCode::NoDevice => ErrorCode::NoDevice,
//            wasi::filesystem::types::ErrorCode::NoEntry => ErrorCode::NoEntry,
//            wasi::filesystem::types::ErrorCode::NoLock => ErrorCode::NoLock,
//            wasi::filesystem::types::ErrorCode::InsufficientMemory => ErrorCode::InsufficientMemory,
//            wasi::filesystem::types::ErrorCode::InsufficientSpace => ErrorCode::InsufficientSpace,
//            wasi::filesystem::types::ErrorCode::NotDirectory => ErrorCode::NotDirectory,
//            wasi::filesystem::types::ErrorCode::NotEmpty => ErrorCode::NotEmpty,
//            wasi::filesystem::types::ErrorCode::NotRecoverable => ErrorCode::NotRecoverable,
//            wasi::filesystem::types::ErrorCode::Unsupported => ErrorCode::Unsupported,
//            wasi::filesystem::types::ErrorCode::NoTty => ErrorCode::NoTty,
//            wasi::filesystem::types::ErrorCode::NoSuchDevice => ErrorCode::NoSuchDevice,
//            wasi::filesystem::types::ErrorCode::Overflow => ErrorCode::Overflow,
//            wasi::filesystem::types::ErrorCode::NotPermitted => ErrorCode::NotPermitted,
//            wasi::filesystem::types::ErrorCode::Pipe => ErrorCode::Pipe,
//            wasi::filesystem::types::ErrorCode::ReadOnly => ErrorCode::ReadOnly,
//            wasi::filesystem::types::ErrorCode::InvalidSeek => ErrorCode::InvalidSeek,
//            wasi::filesystem::types::ErrorCode::TextFileBusy => ErrorCode::TextFileBusy,
//            wasi::filesystem::types::ErrorCode::CrossDevice => ErrorCode::CrossDevice,
//        }
//    }
//}
//
//impl From<wasi::filesystem::types::DescriptorStat> for DescriptorStat {
//    fn from(value: wasi::filesystem::types::DescriptorStat) -> Self {
//        DescriptorStat {
//            type_: value.type_.into(),
//            link_count: value.link_count,
//            size: value.size,
//            data_access_timestamp: value.data_modification_timestamp,
//            data_modification_timestamp: value.data_modification_timestamp,
//            status_change_timestamp: value.status_change_timestamp,
//        }
//    }
//}
//
//impl From<wasi::filesystem::types::MetadataHashValue> for MetadataHashValue {
//    fn from(value: wasi::filesystem::types::MetadataHashValue) -> Self {
//        MetadataHashValue {
//            upper: value.upper,
//            lower: value.lower,
//        }
//    }
//}
//
