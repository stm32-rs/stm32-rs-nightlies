#[doc = "Reader of register MDIOS_HWCFGR"]
pub type R = crate::R<u32, super::MDIOS_HWCFGR>;
#[doc = "Reader of field `NBREG`"]
pub type NBREG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - NBREG"]
    #[inline(always)]
    pub fn nbreg(&self) -> NBREG_R {
        NBREG_R::new((self.bits & 0xff) as u8)
    }
}
