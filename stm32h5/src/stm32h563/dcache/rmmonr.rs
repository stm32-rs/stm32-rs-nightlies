#[doc = "Register `RMMONR` reader"]
pub type R = crate::R<RMMONRrs>;
#[doc = "Field `RMISSMON` reader - cache read-miss monitor counter"]
pub type RMISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - cache read-miss monitor counter"]
    #[inline(always)]
    pub fn rmissmon(&self) -> RMISSMON_R {
        RMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DCACHE read-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMMONRrs;
impl crate::RegisterSpec for RMMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmmonr::R`](R) reader structure"]
impl crate::Readable for RMMONRrs {}
#[doc = "`reset()` method sets RMMONR to value 0"]
impl crate::Resettable for RMMONRrs {
    const RESET_VALUE: u32 = 0;
}
