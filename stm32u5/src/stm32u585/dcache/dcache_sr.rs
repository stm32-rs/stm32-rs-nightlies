#[doc = "Register `DCACHE_SR` reader"]
pub type R = crate::R<DCACHE_SRrs>;
#[doc = "Field `BUSYF` reader - BUSYF"]
pub type BUSYF_R = crate::BitReader;
#[doc = "Field `BSYENDF` reader - BSYENDF"]
pub type BSYENDF_R = crate::BitReader;
#[doc = "Field `ERRF` reader - ERRF"]
pub type ERRF_R = crate::BitReader;
#[doc = "Field `BUSYCMDF` reader - BUSYCMDF"]
pub type BUSYCMDF_R = crate::BitReader;
#[doc = "Field `CMDENDF` reader - CMDENDF"]
pub type CMDENDF_R = crate::BitReader;
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
    #[doc = "Bit 3 - BUSYCMDF"]
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDENDF"]
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_SRrs;
impl crate::RegisterSpec for DCACHE_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_sr::R`](R) reader structure"]
impl crate::Readable for DCACHE_SRrs {}
#[doc = "`reset()` method sets DCACHE_SR to value 0x01"]
impl crate::Resettable for DCACHE_SRrs {
    const RESET_VALUE: u32 = 0x01;
}
