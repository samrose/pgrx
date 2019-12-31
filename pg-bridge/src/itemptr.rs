use crate::{pg_sys, DatumCompatible, PgBox};

/// ## Safety
///
/// This function s unsafe becuase it does not check that the specified ItemPointerData pointer
/// might be null
#[inline]
pub unsafe fn unsaitem_pointer_get_block_number(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::BlockNumber {
    assert!(item_pointer_is_valid(ctid));
    item_pointer_get_block_number_no_check(ctid)
}

/// ## Safety
///
/// This function s unsafe becuase it does not check that the specified ItemPointerData pointer
/// might be null
#[inline]
pub unsafe fn item_pointer_get_offset_number(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::OffsetNumber {
    assert!(item_pointer_is_valid(ctid));
    item_pointer_get_offset_number_no_check(ctid)
}

/// ## Safety
///
/// This function s unsafe becuase it does not check that the specified ItemPointerData pointer
/// might be null
#[inline]
pub unsafe fn item_pointer_get_block_number_no_check(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::BlockNumber {
    let block_id = (*ctid).ip_blkid;
    (((block_id.bi_hi as u32) << 16) | (block_id.bi_lo as u32)) as pg_sys::BlockNumber
}

/// ## Safety
///
/// This function s unsafe becuase it does not check that the specified ItemPointerData pointer
/// might be null
#[inline]
pub unsafe fn item_pointer_get_offset_number_no_check(
    ctid: *const pg_sys::ItemPointerData,
) -> pg_sys::OffsetNumber {
    (*ctid).ip_posid
}

#[allow(clippy::not_unsafe_ptr_arg_deref)] // this is okay b/c we guard against ctid being null
#[inline]
pub fn item_pointer_is_valid(ctid: *const pg_sys::ItemPointerData) -> bool {
    if ctid.is_null() {
        false
    } else {
        unsafe { *ctid }.ip_posid != pg_sys::InvalidOffsetNumber
    }
}

#[inline]
pub fn new_item_pointer(
    blockno: pg_sys::BlockNumber,
    offno: pg_sys::OffsetNumber,
) -> PgBox<pg_sys::ItemPointerData> {
    let mut tid = pg_sys::ItemPointerData::alloc();
    tid.ip_blkid.bi_hi = (blockno >> 16) as u16;
    tid.ip_blkid.bi_lo = (blockno & 0xffff) as u16;
    tid.ip_posid = offno;
    tid
}