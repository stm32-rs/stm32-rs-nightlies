#[doc = "Reader of register APB1SMENR2"]
pub type R = crate::R<u32, super::APB1SMENR2>;
#[doc = "Writer for register APB1SMENR2"]
pub type W = crate::W<u32, super::APB1SMENR2>;
#[doc = "Register APB1SMENR2 `reset()`'s with value 0x61"]
impl crate::ResetValue for super::APB1SMENR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x61
    }
}
#[doc = "Reader of field `LPTIM3SMEN`"]
pub type LPTIM3SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3SMEN`"]
pub struct LPTIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPTIM2SMEN`"]
pub type LPTIM2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2SMEN`"]
pub struct LPTIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `LPUART1SMEN`"]
pub type LPUART1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1SMEN`"]
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
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
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W {
        LPTIM3SMEN_W { w: self }
    }
    #[doc = "Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes."]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
}
