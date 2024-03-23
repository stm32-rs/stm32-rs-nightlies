#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `BUSY` reader - PKA operation in progress"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PROCENDF` reader - PKA end of operation flag"]
pub type PROCENDF_R = crate::BitReader;
#[doc = "Field `RAMERRF` reader - PKA ram error flag"]
pub type RAMERRF_R = crate::BitReader;
#[doc = "Field `ADDRERRF` reader - address er flag"]
pub type ADDRERRF_R = crate::BitReader;
impl R {
    #[doc = "Bit 16 - PKA operation in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PKA end of operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - PKA ram error flag"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - address er flag"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "PKA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
