#[doc = "Reader of register ITLINE7"]
pub type R = crate::R<u32, super::ITLINE7>;
#[doc = "Reader of field `EXTI4`"]
pub type EXTI4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI5`"]
pub type EXTI5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI6`"]
pub type EXTI6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI7`"]
pub type EXTI7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI8`"]
pub type EXTI8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI9`"]
pub type EXTI9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI10`"]
pub type EXTI10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI11`"]
pub type EXTI11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI12`"]
pub type EXTI12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI13`"]
pub type EXTI13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI14`"]
pub type EXTI14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI15`"]
pub type EXTI15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
