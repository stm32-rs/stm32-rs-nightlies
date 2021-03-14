#[doc = "Reader of register SAI_HWCFGR"]
pub type R = crate::R<u32, super::SAI_HWCFGR>;
#[doc = "Reader of field `FIFO_SIZE`"]
pub type FIFO_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPDIF_PDM`"]
pub type SPDIF_PDM_R = crate::R<u8, u8>;
#[doc = "Reader of field `OPTION_REGOUT`"]
pub type OPTION_REGOUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - FIFO_SIZE"]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - SPDIF_PDM"]
    #[inline(always)]
    pub fn spdif_pdm(&self) -> SPDIF_PDM_R {
        SPDIF_PDM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:19 - OPTION_REGOUT"]
    #[inline(always)]
    pub fn option_regout(&self) -> OPTION_REGOUT_R {
        OPTION_REGOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
