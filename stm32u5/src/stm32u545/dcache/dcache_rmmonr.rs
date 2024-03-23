#[doc = "Register `DCACHE_RMMONR` reader"]
pub type R = crate::R<DCACHE_RMMONRrs>;
#[doc = "Field `MRISSMON` reader - RMISSMON"]
pub type MRISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RMISSMON"]
    #[inline(always)]
    pub fn mrissmon(&self) -> MRISSMON_R {
        MRISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DCACHE read-miss monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_rmmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_RMMONRrs;
impl crate::RegisterSpec for DCACHE_RMMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_rmmonr::R`](R) reader structure"]
impl crate::Readable for DCACHE_RMMONRrs {}
#[doc = "`reset()` method sets DCACHE_RMMONR to value 0"]
impl crate::Resettable for DCACHE_RMMONRrs {
    const RESET_VALUE: u32 = 0;
}
