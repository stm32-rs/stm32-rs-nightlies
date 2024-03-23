#[doc = "Register `DCACHE_RHMONR` reader"]
pub type R = crate::R<DCACHE_RHMONRrs>;
#[doc = "Field `RHITMON` reader - RHITMON"]
pub type RHITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RHITMON"]
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
#[doc = "DCACHE read-hit monitor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_rhmonr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_RHMONRrs;
impl crate::RegisterSpec for DCACHE_RHMONRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_rhmonr::R`](R) reader structure"]
impl crate::Readable for DCACHE_RHMONRrs {}
#[doc = "`reset()` method sets DCACHE_RHMONR to value 0"]
impl crate::Resettable for DCACHE_RHMONRrs {
    const RESET_VALUE: u32 = 0;
}
