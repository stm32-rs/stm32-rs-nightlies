#[doc = "Register `APB1RSTR2` reader"]
pub struct R(crate::R<APB1RSTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR2` writer"]
pub struct W(crate::W<APB1RSTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB1RSTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-power UART 1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<LPUART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub struct LPUART1RST_R(crate::FieldReader<bool, LPUART1RST_A>);
impl LPUART1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1RST_A> {
        match self.bits {
            true => Some(LPUART1RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == LPUART1RST_A::RESET
    }
}
impl core::ops::Deref for LPUART1RST_R {
    type Target = crate::FieldReader<bool, LPUART1RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub struct LPUART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPUART1RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "I2C4 reset"]
pub type I2C4RST_A = LPUART1RST_A;
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub type I2C4RST_R = LPUART1RST_R;
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C4RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Low-power timer 2 reset"]
pub type LPTIM2RST_A = LPUART1RST_A;
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub type LPTIM2RST_R = LPUART1RST_R;
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "LPTIM3RST"]
pub type LPTIM3RST_A = LPUART1RST_A;
#[doc = "Field `LPTIM3RST` reader - LPTIM3RST"]
pub type LPTIM3RST_R = LPUART1RST_R;
#[doc = "Field `LPTIM3RST` writer - LPTIM3RST"]
pub struct LPTIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM3RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "FDCAN1RST"]
pub type FDCAN1RST_A = LPUART1RST_A;
#[doc = "Field `FDCAN1RST` reader - FDCAN1RST"]
pub type FDCAN1RST_R = LPUART1RST_R;
#[doc = "Field `FDCAN1RST` writer - FDCAN1RST"]
pub struct FDCAN1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCAN1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FDCAN1RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "USBFSRST"]
pub type USBFSRST_A = LPUART1RST_A;
#[doc = "Field `USBFSRST` reader - USBFSRST"]
pub type USBFSRST_R = LPUART1RST_R;
#[doc = "Field `USBFSRST` writer - USBFSRST"]
pub struct USBFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USBFSRST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "UCPD1RST"]
pub type UCPD1RST_A = LPUART1RST_A;
#[doc = "Field `UCPD1RST` reader - UCPD1RST"]
pub type UCPD1RST_R = LPUART1RST_R;
#[doc = "Field `UCPD1RST` writer - UCPD1RST"]
pub struct UCPD1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCPD1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UCPD1RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1RST"]
    #[inline(always)]
    pub fn fdcan1rst(&self) -> FDCAN1RST_R {
        FDCAN1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSRST"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UCPD1RST"]
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W {
        LPUART1RST_W { w: self }
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    #[doc = "Bit 6 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W {
        LPTIM3RST_W { w: self }
    }
    #[doc = "Bit 9 - FDCAN1RST"]
    #[inline(always)]
    pub fn fdcan1rst(&mut self) -> FDCAN1RST_W {
        FDCAN1RST_W { w: self }
    }
    #[doc = "Bit 21 - USBFSRST"]
    #[inline(always)]
    pub fn usbfsrst(&mut self) -> USBFSRST_W {
        USBFSRST_W { w: self }
    }
    #[doc = "Bit 23 - UCPD1RST"]
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W {
        UCPD1RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr2](index.html) module"]
pub struct APB1RSTR2_SPEC;
impl crate::RegisterSpec for APB1RSTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr2::R](R) reader structure"]
impl crate::Readable for APB1RSTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr2::W](W) writer structure"]
impl crate::Writable for APB1RSTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for APB1RSTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}