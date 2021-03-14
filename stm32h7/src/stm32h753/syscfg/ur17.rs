#[doc = "Reader of register UR17"]
pub type R = crate::R<u32, super::UR17>;
#[doc = "Reader of field `IO_HSLV`"]
pub type IO_HSLV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - I/O high speed / low voltage"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 0x01) != 0)
    }
}
