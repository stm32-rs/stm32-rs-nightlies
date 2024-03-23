#[doc = "Register `SMISR` reader"]
pub type R = crate::R<SMISRrs>;
#[doc = "Field `MIS0` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS0_R = crate::BitReader;
#[doc = "Field `MIS1` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS1_R = crate::BitReader;
#[doc = "Field `MIS2` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS2_R = crate::BitReader;
#[doc = "Field `MIS3` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS3_R = crate::BitReader;
#[doc = "Field `MIS4` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS4_R = crate::BitReader;
#[doc = "Field `MIS5` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS5_R = crate::BitReader;
#[doc = "Field `MIS6` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS6_R = crate::BitReader;
#[doc = "Field `MIS7` reader - masked interrupt status of the secure channel x (x = 7 to 0)"]
pub type MIS7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - masked interrupt status of the secure channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn mis7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "GPDMA secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
