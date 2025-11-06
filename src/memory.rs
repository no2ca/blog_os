use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
};

use bootloader::bootinfo::MemoryMap;

/// boot時のメモリマップから利用可能なフレームを取得する
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

use bootloader::bootinfo::MemoryRegionType;
impl BootInfoFrameAllocator {
    /// bootloaderから渡されたマップを使用したアロケータを作成する
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// 利用可能な物理フレームのイテレータを返す
    ///
    /// 特定の型ではなくItemにPhysFrameをもつイテレータの実装が返されることを期待している
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        // ここで使用可能なフレームをフィルタリングする
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_address = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_address.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

///
/// エイリアシングを防ぐため呼び出しは一度にする
///
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

/// 物理メモリのオフセットを使用してレベル4ページテーブルの仮想アドレスを返す
///
/// エイリアシングを防ぐため呼び出しは一度にする
///
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
