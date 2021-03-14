#[doc = "Reader of register DSI_HWCFGR"]
pub type R = crate::R<u32, super::DSI_HWCFGR>;
#[doc = "Reader of field `TECHNO`"]
pub type TECHNO_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOSIZE`"]
pub type FIFOSIZE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - TECHNO"]
    #[inline(always)]
    pub fn techno(&self) -> TECHNO_R {
        TECHNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
