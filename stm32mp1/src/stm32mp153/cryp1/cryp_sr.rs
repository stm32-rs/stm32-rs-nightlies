#[doc = "Register `CRYP_SR` reader"]
pub type R = crate::R<CRYP_SRrs>;
#[doc = "Field `IFEM` reader - IFEM"]
pub type IFEM_R = crate::BitReader;
#[doc = "Field `IFNF` reader - IFNF"]
pub type IFNF_R = crate::BitReader;
#[doc = "Field `OFNE` reader - OFNE"]
pub type OFNE_R = crate::BitReader;
#[doc = "Field `OFFU` reader - OFFU"]
pub type OFFU_R = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IFEM"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IFNF"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OFNE"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OFFU"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CRYP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_SRrs;
impl crate::RegisterSpec for CRYP_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_sr::R`](R) reader structure"]
impl crate::Readable for CRYP_SRrs {}
#[doc = "`reset()` method sets CRYP_SR to value 0x03"]
impl crate::Resettable for CRYP_SRrs {
    const RESET_VALUE: u32 = 0x03;
}
