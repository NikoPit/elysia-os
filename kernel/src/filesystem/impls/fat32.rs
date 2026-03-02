use fatfs::{IoBase, Read, ReadWriteSeek, Seek, Write};

use crate::{
    filesystem::{
        block_device::{BlockDevice, BlockDeviceError},
        vfs::FileSystem,
    },
    keyboard::block_device::initrd::RAMDISK,
    s_println,
};

pub struct FAT32 {
    fs: fatfs::FileSystem<RamDiskReader>,
}

#[derive(Debug)]
pub struct RamDiskReader {
    pos: u64,
    cache: [u8; 1024],
}

impl IoBase for RamDiskReader {
    type Error = BlockDeviceError;
}

impl Read for RamDiskReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let ramdisk = RAMDISK.get().unwrap();
        let n = ramdisk.read_by_bytes(self.pos as usize, buf)?;

        self.pos += n as u64;

        Ok(n)
    }
}

impl Write for RamDiskReader {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Err(BlockDeviceError::Readonly)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Seek for RamDiskReader {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, Self::Error> {
        let ramdisk = RAMDISK.get().unwrap();

        let new_pos: i64 = match pos {
            fatfs::SeekFrom::Start(s) => s as i64,
            fatfs::SeekFrom::Current(c) => self.pos as i64 + c,
            fatfs::SeekFrom::End(e) => ramdisk.total_bytes() as i64 + e,
        };

        if new_pos < 0 || new_pos > ramdisk.total_bytes() as i64 {
            return Err(BlockDeviceError::Other);
        }

        self.pos = new_pos as u64;
        Ok(self.pos)
    }
}

impl FileSystem for FAT32 {
    fn init(&mut self) -> crate::filesystem::vfs::FSResult<()> {
        Ok(())
    }
}
