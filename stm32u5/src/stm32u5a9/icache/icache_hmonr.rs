#[doc = "Register `ICACHE_HMONR` reader"]
pub type R = crate::R<ICACHE_HMONRrs>;
#[doc = "Field `HITMON` reader - HITMON"]
pub type HITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HITMON"]
    #[inline(always)]
    pub fn hitmon(&self) -> HITMON_R {
        HITMON_R::new(self.bits)
    }
}
#[doc = "ICACHE hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_hmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_HMONRrs;
impl crate::RegisterSpec for ICACHE_HMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_hmonr::R`](R) reader structure"]
impl crate::Readable for ICACHE_HMONRrs {}
#[doc = "`reset()` method sets ICACHE_HMONR to value 0"]
impl crate::Resettable for ICACHE_HMONRrs {
    const RESET_VALUE: u32 = 0;
}
