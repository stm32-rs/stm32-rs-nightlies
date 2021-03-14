#[doc = "Reader of register QUADSPI_HWCFGR"]
pub type R = crate::R<u32, super::QUADSPI_HWCFGR>;
#[doc = "Reader of field `FIFOSIZE`"]
pub type FIFOSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOPTR`"]
pub type FIFOPTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRESCVAL`"]
pub type PRESCVAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `IDLENGTH`"]
pub type IDLENGTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFOPTR"]
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PRESCVAL"]
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IDLENGTH"]
    #[inline(always)]
    pub fn idlength(&self) -> IDLENGTH_R {
        IDLENGTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
