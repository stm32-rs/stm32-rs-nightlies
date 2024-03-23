#[doc = "Register `MDIOS_SR` reader"]
pub type R = crate::R<MDIOS_SRrs>;
#[doc = "Field `PERF` reader - PERF"]
pub type PERF_R = crate::BitReader;
#[doc = "Field `SERF` reader - SERF"]
pub type SERF_R = crate::BitReader;
#[doc = "Field `TERF` reader - TERF"]
pub type TERF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PERF"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERF"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TERF"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MDIOS status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_SRrs;
impl crate::RegisterSpec for MDIOS_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_sr::R`](R) reader structure"]
impl crate::Readable for MDIOS_SRrs {}
#[doc = "`reset()` method sets MDIOS_SR to value 0"]
impl crate::Resettable for MDIOS_SRrs {
    const RESET_VALUE: u32 = 0;
}
