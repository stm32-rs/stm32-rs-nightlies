#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `PERF` reader - Preamble error flag"]
pub type PERF_R = crate::BitReader;
#[doc = "Field `SERF` reader - Start error flag"]
pub type SERF_R = crate::BitReader;
#[doc = "Field `TERF` reader - Turnaround error flag"]
pub type TERF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Preamble error flag"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start error flag"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Turnaround error flag"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MDIOS status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
