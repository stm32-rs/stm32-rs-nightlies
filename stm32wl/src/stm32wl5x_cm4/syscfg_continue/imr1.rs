#[doc = "Reader of register IMR1"]
pub type R = crate::R<u32, super::IMR1>;
#[doc = "Writer for register IMR1"]
pub type W = crate::W<u32, super::IMR1>;
#[doc = "Register IMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI15IM`"]
pub type EXTI15IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15IM`"]
pub struct EXTI15IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `EXTI14IM`"]
pub type EXTI14IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14IM`"]
pub struct EXTI14IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `EXTI13IM`"]
pub type EXTI13IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13IM`"]
pub struct EXTI13IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `EXTI12IM`"]
pub type EXTI12IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12IM`"]
pub struct EXTI12IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `EXTI11IM`"]
pub type EXTI11IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11IM`"]
pub struct EXTI11IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `EXTI10IM`"]
pub type EXTI10IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10IM`"]
pub struct EXTI10IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `EXTI9IM`"]
pub type EXTI9IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9IM`"]
pub struct EXTI9IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EXTI8IM`"]
pub type EXTI8IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8IM`"]
pub struct EXTI8IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EXTI7IM`"]
pub type EXTI7IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7IM`"]
pub struct EXTI7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `EXTI6IM`"]
pub type EXTI6IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6IM`"]
pub struct EXTI6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EXTI5IM`"]
pub type EXTI5IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5IM`"]
pub struct EXTI5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTCSSRUIM`"]
pub type RTCSSRUIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCSSRUIM`"]
pub struct RTCSSRUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSRUIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCSTAMPTAMPLSECSSIM`"]
pub type RTCSTAMPTAMPLSECSSIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCSTAMPTAMPLSECSSIM`"]
pub struct RTCSTAMPTAMPLSECSSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSTAMPTAMPLSECSSIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Peripheral EXTI15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti15im(&self) -> EXTI15IM_R {
        EXTI15IM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral EXTI14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti14im(&self) -> EXTI14IM_R {
        EXTI14IM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral EXTI13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti13im(&self) -> EXTI13IM_R {
        EXTI13IM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral EXTI12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti12im(&self) -> EXTI12IM_R {
        EXTI12IM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral EXTI11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti11im(&self) -> EXTI11IM_R {
        EXTI11IM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral EXTI10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti10im(&self) -> EXTI10IM_R {
        EXTI10IM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral EXTI9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti9im(&self) -> EXTI9IM_R {
        EXTI9IM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral EXTI8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti8im(&self) -> EXTI8IM_R {
        EXTI8IM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral EXTI7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti7im(&self) -> EXTI7IM_R {
        EXTI7IM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral EXTI6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti6im(&self) -> EXTI6IM_R {
        EXTI6IM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral EXTI5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti5im(&self) -> EXTI5IM_R {
        EXTI5IM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&self) -> RTCSSRUIM_R {
        RTCSSRUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&self) -> RTCSTAMPTAMPLSECSSIM_R {
        RTCSTAMPTAMPLSECSSIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Peripheral EXTI15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti15im(&mut self) -> EXTI15IM_W {
        EXTI15IM_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral EXTI14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti14im(&mut self) -> EXTI14IM_W {
        EXTI14IM_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral EXTI13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti13im(&mut self) -> EXTI13IM_W {
        EXTI13IM_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral EXTI12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti12im(&mut self) -> EXTI12IM_W {
        EXTI12IM_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral EXTI11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti11im(&mut self) -> EXTI11IM_W {
        EXTI11IM_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral EXTI10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti10im(&mut self) -> EXTI10IM_W {
        EXTI10IM_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral EXTI9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti9im(&mut self) -> EXTI9IM_W {
        EXTI9IM_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral EXTI8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti8im(&mut self) -> EXTI8IM_W {
        EXTI8IM_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral EXTI7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti7im(&mut self) -> EXTI7IM_W {
        EXTI7IM_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral EXTI6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti6im(&mut self) -> EXTI6IM_W {
        EXTI6IM_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral EXTI5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exti5im(&mut self) -> EXTI5IM_W {
        EXTI5IM_W { w: self }
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&mut self) -> RTCSSRUIM_W {
        RTCSSRUIM_W { w: self }
    }
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&mut self) -> RTCSTAMPTAMPLSECSSIM_W {
        RTCSTAMPTAMPLSECSSIM_W { w: self }
    }
}
