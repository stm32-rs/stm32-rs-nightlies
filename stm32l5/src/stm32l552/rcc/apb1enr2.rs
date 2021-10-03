#[doc = "Register `APB1ENR2` reader"]
pub struct R(crate::R<APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR2` writer"]
pub struct W(crate::W<APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR2_SPEC>;
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
impl From<crate::W<APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low power UART 1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable"]
pub struct LPUART1EN_R(crate::FieldReader<bool, LPUART1EN_A>);
impl LPUART1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::DISABLED,
            true => LPUART1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPUART1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LPUART1EN_A::ENABLED
    }
}
impl core::ops::Deref for LPUART1EN_R {
    type Target = crate::FieldReader<bool, LPUART1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable"]
pub struct LPUART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::ENABLED)
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
#[doc = "I2C4 clock enable"]
pub type I2C4EN_A = LPUART1EN_A;
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub type I2C4EN_R = LPUART1EN_R;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C4EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C4EN_A::ENABLED)
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
#[doc = "LPTIM2EN"]
pub type LPTIM2EN_A = LPUART1EN_A;
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub type LPTIM2EN_R = LPUART1EN_R;
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::ENABLED)
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
#[doc = "LPTIM3EN"]
pub type LPTIM3EN_A = LPUART1EN_A;
#[doc = "Field `LPTIM3EN` reader - LPTIM3EN"]
pub type LPTIM3EN_R = LPUART1EN_R;
#[doc = "Field `LPTIM3EN` writer - LPTIM3EN"]
pub struct LPTIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::ENABLED)
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
#[doc = "FDCAN1EN"]
pub type FDCAN1EN_A = LPUART1EN_A;
#[doc = "Field `FDCAN1EN` reader - FDCAN1EN"]
pub type FDCAN1EN_R = LPUART1EN_R;
#[doc = "Field `FDCAN1EN` writer - FDCAN1EN"]
pub struct FDCAN1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCAN1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FDCAN1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FDCAN1EN_A::ENABLED)
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
#[doc = "USBFSEN"]
pub type USBFSEN_A = LPUART1EN_A;
#[doc = "Field `USBFSEN` reader - USBFSEN"]
pub type USBFSEN_R = LPUART1EN_R;
#[doc = "Field `USBFSEN` writer - USBFSEN"]
pub struct USBFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBFSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBFSEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBFSEN_A::ENABLED)
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
#[doc = "UCPD1EN"]
pub type UCPD1EN_A = LPUART1EN_A;
#[doc = "Field `UCPD1EN` reader - UCPD1EN"]
pub type UCPD1EN_R = LPUART1EN_R;
#[doc = "Field `UCPD1EN` writer - UCPD1EN"]
pub struct UCPD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCPD1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UCPD1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UCPD1EN_A::ENABLED)
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
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1EN"]
    #[inline(always)]
    pub fn fdcan1en(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSEN"]
    #[inline(always)]
    pub fn usbfsen(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UCPD1EN"]
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W {
        LPUART1EN_W { w: self }
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    #[doc = "Bit 6 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W {
        LPTIM3EN_W { w: self }
    }
    #[doc = "Bit 9 - FDCAN1EN"]
    #[inline(always)]
    pub fn fdcan1en(&mut self) -> FDCAN1EN_W {
        FDCAN1EN_W { w: self }
    }
    #[doc = "Bit 21 - USBFSEN"]
    #[inline(always)]
    pub fn usbfsen(&mut self) -> USBFSEN_W {
        USBFSEN_W { w: self }
    }
    #[doc = "Bit 23 - UCPD1EN"]
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W {
        UCPD1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](index.html) module"]
pub struct APB1ENR2_SPEC;
impl crate::RegisterSpec for APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr2::R](R) reader structure"]
impl crate::Readable for APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](W) writer structure"]
impl crate::Writable for APB1ENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
