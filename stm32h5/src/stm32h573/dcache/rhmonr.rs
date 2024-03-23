#[doc = "Register `RHMONR` reader"]
pub type R = crate::R<RHMONRrs>;
#[doc = "Field `RHITMON` reader - cache read-hit monitor counter"]
pub type RHITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cache read-hit monitor counter"]
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
#[doc = "DCACHE read-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHMONRrs;
impl crate::RegisterSpec for RHMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhmonr::R`](R) reader structure"]
impl crate::Readable for RHMONRrs {}
#[doc = "`reset()` method sets RHMONR to value 0"]
impl crate::Resettable for RHMONRrs {
    const RESET_VALUE: u32 = 0;
}
