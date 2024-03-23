#[doc = "Register `PTPPPSCR` reader"]
pub type R = crate::R<PTPPPSCRrs>;
#[doc = "Field `PPSFREQ` reader - PPS frequency selection"]
pub type PPSFREQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PPS frequency selection"]
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
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
