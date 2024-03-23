#[doc = "Register `MMCTGFMSCCR` reader"]
pub type R = crate::R<MMCTGFMSCCRrs>;
#[doc = "Field `TGFMSCC` reader - Transmitted good frames more single collision counter"]
pub type TGFMSCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames more single collision counter"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TGFMSCC_R {
        TGFMSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfmsccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTGFMSCCRrs;
impl crate::RegisterSpec for MMCTGFMSCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfmsccr::R`](R) reader structure"]
impl crate::Readable for MMCTGFMSCCRrs {}
#[doc = "`reset()` method sets MMCTGFMSCCR to value 0"]
impl crate::Resettable for MMCTGFMSCCRrs {
    const RESET_VALUE: u32 = 0;
}
