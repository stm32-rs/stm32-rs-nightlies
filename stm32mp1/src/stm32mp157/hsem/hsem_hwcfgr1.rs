#[doc = "Reader of register HSEM_HWCFGR1"]
pub type R = crate::R<u32, super::HSEM_HWCFGR1>;
#[doc = "Reader of field `NBSEM`"]
pub type NBSEM_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBINT`"]
pub type NBINT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - NBSEM"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - NBINT"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
