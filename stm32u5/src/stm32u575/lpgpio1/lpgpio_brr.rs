#[doc = "Register `LPGPIO_BRR` reader"]
pub type R = crate::R<LPGPIO_BRRrs>;
#[doc = "Field `BRy0` reader - BRy0"]
pub type BRY0_R = crate::BitReader;
#[doc = "Field `BRy1` reader - BRy1"]
pub type BRY1_R = crate::BitReader;
#[doc = "Field `BRy2` reader - BRy2"]
pub type BRY2_R = crate::BitReader;
#[doc = "Field `BRy3` reader - BRy3"]
pub type BRY3_R = crate::BitReader;
#[doc = "Field `BRy4` reader - BRy4"]
pub type BRY4_R = crate::BitReader;
#[doc = "Field `BRy5` reader - BRy5"]
pub type BRY5_R = crate::BitReader;
#[doc = "Field `BRy6` reader - BRy6"]
pub type BRY6_R = crate::BitReader;
#[doc = "Field `BRy7` reader - BRy7"]
pub type BRY7_R = crate::BitReader;
#[doc = "Field `BRy8` reader - BRy8"]
pub type BRY8_R = crate::BitReader;
#[doc = "Field `BRy9` reader - BRy9"]
pub type BRY9_R = crate::BitReader;
#[doc = "Field `BRy10` reader - BRy10"]
pub type BRY10_R = crate::BitReader;
#[doc = "Field `BRy11` reader - BRy11"]
pub type BRY11_R = crate::BitReader;
#[doc = "Field `BRy12` reader - BRy12"]
pub type BRY12_R = crate::BitReader;
#[doc = "Field `BRy13` reader - BRy13"]
pub type BRY13_R = crate::BitReader;
#[doc = "Field `BRy14` reader - BRy14"]
pub type BRY14_R = crate::BitReader;
#[doc = "Field `BRy15` reader - BRy15"]
pub type BRY15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BRy0"]
    #[inline(always)]
    pub fn bry0(&self) -> BRY0_R {
        BRY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRy1"]
    #[inline(always)]
    pub fn bry1(&self) -> BRY1_R {
        BRY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRy2"]
    #[inline(always)]
    pub fn bry2(&self) -> BRY2_R {
        BRY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRy3"]
    #[inline(always)]
    pub fn bry3(&self) -> BRY3_R {
        BRY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRy4"]
    #[inline(always)]
    pub fn bry4(&self) -> BRY4_R {
        BRY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRy5"]
    #[inline(always)]
    pub fn bry5(&self) -> BRY5_R {
        BRY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRy6"]
    #[inline(always)]
    pub fn bry6(&self) -> BRY6_R {
        BRY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRy7"]
    #[inline(always)]
    pub fn bry7(&self) -> BRY7_R {
        BRY7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BRy8"]
    #[inline(always)]
    pub fn bry8(&self) -> BRY8_R {
        BRY8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRy9"]
    #[inline(always)]
    pub fn bry9(&self) -> BRY9_R {
        BRY9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRy10"]
    #[inline(always)]
    pub fn bry10(&self) -> BRY10_R {
        BRY10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRy11"]
    #[inline(always)]
    pub fn bry11(&self) -> BRY11_R {
        BRY11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRy12"]
    #[inline(always)]
    pub fn bry12(&self) -> BRY12_R {
        BRY12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRy13"]
    #[inline(always)]
    pub fn bry13(&self) -> BRY13_R {
        BRY13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BRy14"]
    #[inline(always)]
    pub fn bry14(&self) -> BRY14_R {
        BRY14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BRy15"]
    #[inline(always)]
    pub fn bry15(&self) -> BRY15_R {
        BRY15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "LPGPIO port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_brr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_BRRrs;
impl crate::RegisterSpec for LPGPIO_BRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpgpio_brr::R`](R) reader structure"]
impl crate::Readable for LPGPIO_BRRrs {}
#[doc = "`reset()` method sets LPGPIO_BRR to value 0"]
impl crate::Resettable for LPGPIO_BRRrs {
    const RESET_VALUE: u32 = 0;
}
