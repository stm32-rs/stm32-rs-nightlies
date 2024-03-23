#[doc = "Register `WMMONR` reader"]
pub type R = crate::R<WMMONRrs>;
#[doc = "Field `WMISSMON` reader - cache write-miss monitor counter"]
pub type WMISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - cache write-miss monitor counter"]
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DCACHE write-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wmmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WMMONRrs;
impl crate::RegisterSpec for WMMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wmmonr::R`](R) reader structure"]
impl crate::Readable for WMMONRrs {}
#[doc = "`reset()` method sets WMMONR to value 0"]
impl crate::Resettable for WMMONRrs {
    const RESET_VALUE: u32 = 0;
}
