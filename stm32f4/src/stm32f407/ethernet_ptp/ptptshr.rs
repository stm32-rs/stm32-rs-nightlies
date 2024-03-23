#[doc = "Register `PTPTSHR` reader"]
pub type R = crate::R<PTPTSHRrs>;
#[doc = "Field `STS` reader - STS"]
pub type STS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - STS"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(self.bits)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHRrs;
impl crate::RegisterSpec for PTPTSHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshr::R`](R) reader structure"]
impl crate::Readable for PTPTSHRrs {}
#[doc = "`reset()` method sets PTPTSHR to value 0"]
impl crate::Resettable for PTPTSHRrs {
    const RESET_VALUE: u32 = 0;
}
