#[doc = "Register `ITLINE7` reader"]
pub type R = crate::R<ITLINE7rs>;
#[doc = "Field `EXTI4` reader - EXTI4"]
pub type EXTI4_R = crate::BitReader;
#[doc = "Field `EXTI5` reader - EXTI5"]
pub type EXTI5_R = crate::BitReader;
#[doc = "Field `EXTI6` reader - EXTI6"]
pub type EXTI6_R = crate::BitReader;
#[doc = "Field `EXTI7` reader - EXTI7"]
pub type EXTI7_R = crate::BitReader;
#[doc = "Field `EXTI8` reader - EXTI8"]
pub type EXTI8_R = crate::BitReader;
#[doc = "Field `EXTI9` reader - EXTI9"]
pub type EXTI9_R = crate::BitReader;
#[doc = "Field `EXTI10` reader - EXTI10"]
pub type EXTI10_R = crate::BitReader;
#[doc = "Field `EXTI11` reader - EXTI11"]
pub type EXTI11_R = crate::BitReader;
#[doc = "Field `EXTI12` reader - EXTI12"]
pub type EXTI12_R = crate::BitReader;
#[doc = "Field `EXTI13` reader - EXTI13"]
pub type EXTI13_R = crate::BitReader;
#[doc = "Field `EXTI14` reader - EXTI14"]
pub type EXTI14_R = crate::BitReader;
#[doc = "Field `EXTI15` reader - EXTI15"]
pub type EXTI15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "interrupt line 7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE7rs;
impl crate::RegisterSpec for ITLINE7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline7::R`](R) reader structure"]
impl crate::Readable for ITLINE7rs {}
#[doc = "`reset()` method sets ITLINE7 to value 0"]
impl crate::Resettable for ITLINE7rs {
    const RESET_VALUE: u32 = 0;
}
