#[doc = "Register `MDIOS_HWCFGR` reader"]
pub type R = crate::R<MDIOS_HWCFGRrs>;
#[doc = "Field `NBREG` reader - NBREG"]
pub type NBREG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NBREG"]
    #[inline(always)]
    pub fn nbreg(&self) -> NBREG_R {
        NBREG_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "MDIOS HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_HWCFGRrs;
impl crate::RegisterSpec for MDIOS_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_hwcfgr::R`](R) reader structure"]
impl crate::Readable for MDIOS_HWCFGRrs {}
#[doc = "`reset()` method sets MDIOS_HWCFGR to value 0x20"]
impl crate::Resettable for MDIOS_HWCFGRrs {
    const RESET_VALUE: u32 = 0x20;
}
