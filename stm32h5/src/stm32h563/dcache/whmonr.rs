#[doc = "Register `WHMONR` reader"]
pub type R = crate::R<WHMONRrs>;
#[doc = "Field `WHITMON` reader - cache write-hit monitor counter"]
pub type WHITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cache write-hit monitor counter"]
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
#[doc = "DCACHE write-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`whmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WHMONRrs;
impl crate::RegisterSpec for WHMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`whmonr::R`](R) reader structure"]
impl crate::Readable for WHMONRrs {}
#[doc = "`reset()` method sets WHMONR to value 0"]
impl crate::Resettable for WHMONRrs {
    const RESET_VALUE: u32 = 0;
}
