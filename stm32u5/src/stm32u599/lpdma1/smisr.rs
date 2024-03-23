#[doc = "Register `SMISR` reader"]
pub type R = crate::R<SMISRrs>;
#[doc = "Field `MIS0` reader - MIS0"]
pub type MIS0_R = crate::BitReader;
#[doc = "Field `MIS1` reader - MIS1"]
pub type MIS1_R = crate::BitReader;
#[doc = "Field `MIS2` reader - MIS2"]
pub type MIS2_R = crate::BitReader;
#[doc = "Field `MIS3` reader - MIS3"]
pub type MIS3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MIS0"]
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIS1"]
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MIS2"]
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MIS3"]
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LPDMA secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smisr::R`](R) reader structure"]
impl crate::Readable for SMISRrs {}
#[doc = "`reset()` method sets SMISR to value 0"]
impl crate::Resettable for SMISRrs {
    const RESET_VALUE: u32 = 0;
}
