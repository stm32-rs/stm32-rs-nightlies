#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `BUSYF` reader - BUSYF"]
pub type BUSYF_R = crate::BitReader;
#[doc = "Field `BSYENDF` reader - BSYENDF"]
pub type BSYENDF_R = crate::BitReader;
#[doc = "Field `ERRF` reader - ERRF"]
pub type ERRF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BUSYF"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BSYENDF"]
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRF"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ICACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
