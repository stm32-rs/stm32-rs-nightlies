#[doc = "Register `APB1SMENR2` reader"]
pub struct R(crate::R<APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1SMENR2` writer"]
pub struct W(crate::W<APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR2_SPEC>;
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
impl From<crate::W<APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub struct LPUART1SMEN_R(crate::FieldReader<bool, bool>);
impl LPUART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes"]
pub struct I2C4SMEN_R(crate::FieldReader<bool, bool>);
impl I2C4SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes"]
pub struct I2C4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SMEN_W<'a> {
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
#[doc = "Field `LPTIM2SMEN` reader - LPTIM2SMEN"]
pub struct LPTIM2SMEN_R(crate::FieldReader<bool, bool>);
impl LPTIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2SMEN` writer - LPTIM2SMEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LPTIM3SMEN` reader - LPTIM3SMEN"]
pub struct LPTIM3SMEN_R(crate::FieldReader<bool, bool>);
impl LPTIM3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3SMEN` writer - LPTIM3SMEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FDCAN1SMEN` reader - FDCAN1SMEN"]
pub struct FDCAN1SMEN_R(crate::FieldReader<bool, bool>);
impl FDCAN1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCAN1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCAN1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCAN1SMEN` writer - FDCAN1SMEN"]
pub struct FDCAN1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1SMEN_W<'a> {
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
#[doc = "Field `USBFSSMEN` reader - USBFSSMEN"]
pub struct USBFSSMEN_R(crate::FieldReader<bool, bool>);
impl USBFSSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFSSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFSSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSSMEN` writer - USBFSSMEN"]
pub struct USBFSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSSMEN_W<'a> {
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
#[doc = "Field `UCPD1SMEN` reader - UCPD1SMEN"]
pub struct UCPD1SMEN_R(crate::FieldReader<bool, bool>);
impl UCPD1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPD1SMEN` writer - UCPD1SMEN"]
pub struct UCPD1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1SMEN_W<'a> {
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
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3SMEN"]
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1SMEN"]
    #[inline(always)]
    pub fn fdcan1smen(&self) -> FDCAN1SMEN_R {
        FDCAN1SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSSMEN"]
    #[inline(always)]
    pub fn usbfssmen(&self) -> USBFSSMEN_R {
        USBFSSMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UCPD1SMEN"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W {
        I2C4SMEN_W { w: self }
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    #[doc = "Bit 6 - LPTIM3SMEN"]
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W {
        LPTIM3SMEN_W { w: self }
    }
    #[doc = "Bit 9 - FDCAN1SMEN"]
    #[inline(always)]
    pub fn fdcan1smen(&mut self) -> FDCAN1SMEN_W {
        FDCAN1SMEN_W { w: self }
    }
    #[doc = "Bit 21 - USBFSSMEN"]
    #[inline(always)]
    pub fn usbfssmen(&mut self) -> USBFSSMEN_W {
        USBFSSMEN_W { w: self }
    }
    #[doc = "Bit 23 - UCPD1SMEN"]
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W {
        UCPD1SMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1smenr2](index.html) module"]
pub struct APB1SMENR2_SPEC;
impl crate::RegisterSpec for APB1SMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1smenr2::R](R) reader structure"]
impl crate::Readable for APB1SMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1smenr2::W](W) writer structure"]
impl crate::Writable for APB1SMENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x00a0_0223"]
impl crate::Resettable for APB1SMENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00a0_0223
    }
}
