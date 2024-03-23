#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `IFEM` reader - Input FIFO empty"]
pub type IFEM_R = crate::BitReader;
#[doc = "Field `IFNF` reader - Input FIFO not full"]
pub type IFNF_R = crate::BitReader;
#[doc = "Field `OFNE` reader - Output FIFO not empty"]
pub type OFNE_R = crate::BitReader;
#[doc = "Field `OFFU` reader - Output FIFO full"]
pub type OFFU_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input FIFO empty"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input FIFO not full"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output FIFO not empty"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO full"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x03;
}
