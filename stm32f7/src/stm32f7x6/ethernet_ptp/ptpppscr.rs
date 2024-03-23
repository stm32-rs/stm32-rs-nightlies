#[doc = "Register `PTPPPSCR` reader"]
pub type R = crate::R<PTPPPSCRrs>;
#[doc = "Field `TSSO` reader - TSSO"]
pub type TSSO_R = crate::BitReader;
#[doc = "Field `TSTTR` reader - TSTTR"]
pub type TSTTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TSSO"]
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSTTR"]
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPPPSCRrs;
impl crate::RegisterSpec for PTPPPSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpppscr::R`](R) reader structure"]
impl crate::Readable for PTPPPSCRrs {}
#[doc = "`reset()` method sets PTPPPSCR to value 0"]
impl crate::Resettable for PTPPPSCRrs {
    const RESET_VALUE: u32 = 0;
}
