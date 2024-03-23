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
#[doc = "Field `MIS4` reader - MIS4"]
pub type MIS4_R = crate::BitReader;
#[doc = "Field `MIS5` reader - MIS5"]
pub type MIS5_R = crate::BitReader;
#[doc = "Field `MIS6` reader - MIS6"]
pub type MIS6_R = crate::BitReader;
#[doc = "Field `MIS7` reader - MIS7"]
pub type MIS7_R = crate::BitReader;
#[doc = "Field `MIS8` reader - MIS8"]
pub type MIS8_R = crate::BitReader;
#[doc = "Field `MIS9` reader - MIS9"]
pub type MIS9_R = crate::BitReader;
#[doc = "Field `MIS10` reader - MIS10"]
pub type MIS10_R = crate::BitReader;
#[doc = "Field `MIS11` reader - MIS11"]
pub type MIS11_R = crate::BitReader;
#[doc = "Field `MIS12` reader - MIS12"]
pub type MIS12_R = crate::BitReader;
#[doc = "Field `MIS13` reader - MIS13"]
pub type MIS13_R = crate::BitReader;
#[doc = "Field `MIS14` reader - MIS14"]
pub type MIS14_R = crate::BitReader;
#[doc = "Field `MIS15` reader - MIS15"]
pub type MIS15_R = crate::BitReader;
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
    #[doc = "Bit 4 - MIS4"]
    #[inline(always)]
    pub fn mis4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MIS5"]
    #[inline(always)]
    pub fn mis5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MIS6"]
    #[inline(always)]
    pub fn mis6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MIS7"]
    #[inline(always)]
    pub fn mis7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MIS8"]
    #[inline(always)]
    pub fn mis8(&self) -> MIS8_R {
        MIS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MIS9"]
    #[inline(always)]
    pub fn mis9(&self) -> MIS9_R {
        MIS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MIS10"]
    #[inline(always)]
    pub fn mis10(&self) -> MIS10_R {
        MIS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MIS11"]
    #[inline(always)]
    pub fn mis11(&self) -> MIS11_R {
        MIS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MIS12"]
    #[inline(always)]
    pub fn mis12(&self) -> MIS12_R {
        MIS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MIS13"]
    #[inline(always)]
    pub fn mis13(&self) -> MIS13_R {
        MIS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MIS14"]
    #[inline(always)]
    pub fn mis14(&self) -> MIS14_R {
        MIS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MIS15"]
    #[inline(always)]
    pub fn mis15(&self) -> MIS15_R {
        MIS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
