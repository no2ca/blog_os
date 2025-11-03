use x86_64::{VirtAddr, structures::paging::{OffsetPageTable, PageTable}};

use bootloader::bootinfo::MemoryMap;

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

use bootloader::bootinfo::MemoryRegionType;
impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
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
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
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

use x86_64::{
    PhysAddr,
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator}
};

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    
    let frame = PhysFrame::containing_address(PhysAddr::new(0xdeadbeef));
    let flags = Flags::PRESENT | Flags::WRITABLE;
    
    let map_to_result = unsafe {
        // TODO:
        // 使用されているフレームに書き込んでいる
        // テスト用の実装
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

// ダミーのアロケータ
// 追加のマッピングを作る必要がないときには役立つかも
pub struct EmptyFrameAllocator;

// 未使用のフレームのみを生成する必要があるためunsafe
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
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