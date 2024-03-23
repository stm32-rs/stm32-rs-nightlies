#[doc = "Register `DCACHE_WHMONR` reader"]
pub type R = crate::R<DCACHE_WHMONRrs>;
#[doc = "Field `WHITMON` reader - WHITMON"]
pub type WHITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - WHITMON"]
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
#[doc = "write-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_whmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_WHMONRrs;
impl crate::RegisterSpec for DCACHE_WHMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_whmonr::R`](R) reader structure"]
impl crate::Readable for DCACHE_WHMONRrs {}
#[doc = "`reset()` method sets DCACHE_WHMONR to value 0"]
impl crate::Resettable for DCACHE_WHMONRrs {
    const RESET_VALUE: u32 = 0;
}
