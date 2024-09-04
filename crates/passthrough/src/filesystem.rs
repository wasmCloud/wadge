use exports::wasi::filesystem::types::DescriptorBorrow;

use wasi::filesystem::types::{Descriptor, DirectoryEntryStream, Filesize};

use crate::bindings::{exports, wasi};
use crate::Handler;

impl exports::wasi::filesystem::preopens::Guest for Handler {
    fn get_directories() -> Vec<(exports::wasi::filesystem::types::Descriptor, String)> {
        todo!()
    }
}

impl exports::wasi::filesystem::types::Guest for Handler {
    type Descriptor = Descriptor;
    type DirectoryEntryStream = DirectoryEntryStream;

    fn filesystem_error_code(
        err: exports::wasi::io::error::ErrorBorrow<'_>,
    ) -> Option<exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }
}

impl exports::wasi::filesystem::types::GuestDescriptor for Descriptor {
    fn read_via_stream(
        &self,
        offset: Filesize,
    ) -> Result<exports::wasi::io::streams::InputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        todo!()
    }

    fn write_via_stream(
        &self,
        offset: Filesize,
    ) -> Result<exports::wasi::io::streams::OutputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        todo!()
    }

    fn append_via_stream(
        &self,
    ) -> Result<exports::wasi::io::streams::OutputStream, exports::wasi::filesystem::types::ErrorCode>
    {
        todo!()
    }

    fn advise(
        &self,
        offset: Filesize,
        length: Filesize,
        advice: exports::wasi::filesystem::types::Advice,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn sync_data(&self) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn get_flags(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorFlags,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn get_type(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorType,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn set_size(&self, size: Filesize) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn set_times(
        &self,
        data_access_timestamp: exports::wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: exports::wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn read(
        &self,
        length: Filesize,
        offset: Filesize,
    ) -> Result<(Vec<u8>, bool), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn write(
        &self,
        buffer: Vec<u8>,
        offset: Filesize,
    ) -> Result<Filesize, exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn read_directory(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DirectoryEntryStream,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn sync(&self) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn create_directory_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn stat(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorStat,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn stat_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
    ) -> Result<
        exports::wasi::filesystem::types::DescriptorStat,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn set_times_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
        data_access_timestamp: exports::wasi::filesystem::types::NewTimestamp,
        data_modification_timestamp: exports::wasi::filesystem::types::NewTimestamp,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn link_at(
        &self,
        old_path_flags: exports::wasi::filesystem::types::PathFlags,
        old_path: String,
        new_descriptor: DescriptorBorrow<'_>,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn open_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
        open_flags: exports::wasi::filesystem::types::OpenFlags,
        flags: exports::wasi::filesystem::types::DescriptorFlags,
    ) -> Result<
        exports::wasi::filesystem::types::Descriptor,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn readlink_at(
        &self,
        path: String,
    ) -> Result<String, exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn remove_directory_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn rename_at(
        &self,
        old_path: String,
        new_descriptor: DescriptorBorrow<'_>,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn symlink_at(
        &self,
        old_path: String,
        new_path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn unlink_file_at(
        &self,
        path: String,
    ) -> Result<(), exports::wasi::filesystem::types::ErrorCode> {
        todo!()
    }

    fn is_same_object(&self, other: DescriptorBorrow<'_>) -> bool {
        todo!()
    }

    fn metadata_hash(
        &self,
    ) -> Result<
        exports::wasi::filesystem::types::MetadataHashValue,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }

    fn metadata_hash_at(
        &self,
        path_flags: exports::wasi::filesystem::types::PathFlags,
        path: String,
    ) -> Result<
        exports::wasi::filesystem::types::MetadataHashValue,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }
}

impl exports::wasi::filesystem::types::GuestDirectoryEntryStream for DirectoryEntryStream {
    fn read_directory_entry(
        &self,
    ) -> Result<
        Option<exports::wasi::filesystem::types::DirectoryEntry>,
        exports::wasi::filesystem::types::ErrorCode,
    > {
        todo!()
    }
}
