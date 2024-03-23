#[doc = "Register `DCACHE_WMMONR` reader"]
pub type R = crate::R<DCACHE_WMMONRrs>;
#[doc = "Field `WMISSMON` reader - WMISSMON"]
pub type WMISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - WMISSMON"]
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "write-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_wmmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_WMMONRrs;
impl crate::RegisterSpec for DCACHE_WMMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_wmmonr::R`](R) reader structure"]
impl crate::Readable for DCACHE_WMMONRrs {}
#[doc = "`reset()` method sets DCACHE_WMMONR to value 0"]
impl crate::Resettable for DCACHE_WMMONRrs {
    const RESET_VALUE: u32 = 0;
}
