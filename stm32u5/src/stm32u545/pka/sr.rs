#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `INITOK` reader - INITOK"]
pub type INITOK_R = crate::BitReader;
#[doc = "Field `BUSY` reader - PKA operation is in progress"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PROCENDF` reader - PKA End of Operation flag"]
pub type PROCENDF_R = crate::BitReader;
#[doc = "Field `RAMERRF` reader - RAMERRF"]
pub type RAMERRF_R = crate::BitReader;
#[doc = "Field `ADDRERRF` reader - ADDRERRF"]
pub type ADDRERRF_R = crate::BitReader;
#[doc = "Field `OPERRF` reader - OPERRF"]
pub type OPERRF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INITOK"]
    #[inline(always)]
    pub fn initok(&self) -> INITOK_R {
        INITOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - PKA operation is in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAMERRF"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADDRERRF"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPERRF"]
    #[inline(always)]
    pub fn operrf(&self) -> OPERRF_R {
        OPERRF_R::new(((self.bits >> 21) & 1) != 0)
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
