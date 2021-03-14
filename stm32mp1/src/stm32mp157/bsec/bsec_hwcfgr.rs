#[doc = "Reader of register BSEC_HWCFGR"]
pub type R = crate::R<u32, super::BSEC_HWCFGR>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ECC_USE`"]
pub type ECC_USE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECC_USE"]
    #[inline(always)]
    pub fn ecc_use(&self) -> ECC_USE_R {
        ECC_USE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
