#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `BUSYF` reader - full invalidate busy flag"]
pub type BUSYF_R = crate::BitReader;
#[doc = "Field `BSYENDF` reader - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
pub type BSYENDF_R = crate::BitReader;
#[doc = "Field `ERRF` reader - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
pub type ERRF_R = crate::BitReader;
#[doc = "Field `BUSYCMDF` reader - command busy flag"]
pub type BUSYCMDF_R = crate::BitReader;
#[doc = "Field `CMDENDF` reader - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
pub type CMDENDF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - full invalidate busy flag"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1."]
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1."]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - command busy flag"]
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1."]
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DCACHE status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
