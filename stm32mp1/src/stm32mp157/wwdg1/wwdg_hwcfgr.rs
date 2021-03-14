#[doc = "Reader of register WWDG_HWCFGR"]
pub type R = crate::R<u32, super::WWDG_HWCFGR>;
#[doc = "Reader of field `PREDIV`"]
pub type PREDIV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PREDIV"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0xffff) as u16)
    }
}
