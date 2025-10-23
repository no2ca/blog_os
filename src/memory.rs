use x86_64::{PhysAddr, VirtAddr, structures::paging::PageTable};

/// 物理メモリのオフセットを使用してレベル4ページテーブルの仮想アドレスを返す
///
/// エイリアシングを防ぐため呼び出しは一度にする
///
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

/// この関数は`unsafe fn`の中で呼び出されたときのみを想定している
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    
    // アドレスからそれぞれのテーブルフレームのインデックスを計算している
    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;
    
    // frame は l4 -> l3 -> l2 -> l1 -> phys と4回変わる
    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};
        
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported\n"),
        };
    }
    
    // 最後にページのオフセット部分を取得して足す
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
