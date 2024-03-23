#[doc = "Register `ICACHE_MMONR` reader"]
pub type R = crate::R<ICACHE_MMONRrs>;
#[doc = "Field `MISSMON` reader - MISSMON"]
pub type MISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MISSMON"]
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ICACHE miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_mmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_MMONRrs;
impl crate::RegisterSpec for ICACHE_MMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_mmonr::R`](R) reader structure"]
impl crate::Readable for ICACHE_MMONRrs {}
#[doc = "`reset()` method sets ICACHE_MMONR to value 0"]
impl crate::Resettable for ICACHE_MMONRrs {
    const RESET_VALUE: u32 = 0;
}
