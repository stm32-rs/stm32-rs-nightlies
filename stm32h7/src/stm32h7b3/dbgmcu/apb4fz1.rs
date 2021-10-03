#[doc = "Register `APB4FZ1` reader"]
pub struct R(crate::R<APB4FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB4FZ1` writer"]
pub struct W(crate::W<APB4FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4FZ1_SPEC>;
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
impl From<crate::W<APB4FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C4` reader - I2C4 SMBUS timeout stop in debug"]
pub struct I2C4_R(crate::FieldReader<bool, bool>);
impl I2C4_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4` writer - I2C4 SMBUS timeout stop in debug"]
pub struct I2C4_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LPTIM2` reader - LPTIM2 stop in debug"]
pub struct LPTIM2_R(crate::FieldReader<bool, bool>);
impl LPTIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2` writer - LPTIM2 stop in debug"]
pub struct LPTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2_W<'a> {
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
#[doc = "Field `LPTIM3` reader - LPTIM3 stop in debug"]
pub struct LPTIM3_R(crate::FieldReader<bool, bool>);
impl LPTIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3` writer - LPTIM3 stop in debug"]
pub struct LPTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RTC` reader - RTC stop in debug"]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - RTC stop in debug"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `WDGLSCD` reader - LS watchdog for CPU domain stop in debug"]
pub struct WDGLSCD_R(crate::FieldReader<bool, bool>);
impl WDGLSCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDGLSCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDGLSCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDGLSCD` writer - LS watchdog for CPU domain stop in debug"]
pub struct WDGLSCD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGLSCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c4(&self) -> I2C4_R {
        I2C4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LS watchdog for CPU domain stop in debug"]
    #[inline(always)]
    pub fn wdglscd(&self) -> WDGLSCD_R {
        WDGLSCD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2C4_W {
        I2C4_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn lptim2(&mut self) -> LPTIM2_W {
        LPTIM2_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn lptim3(&mut self) -> LPTIM3_W {
        LPTIM3_W { w: self }
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 18 - LS watchdog for CPU domain stop in debug"]
    #[inline(always)]
    pub fn wdglscd(&mut self) -> WDGLSCD_W {
        WDGLSCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB4 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4fz1](index.html) module"]
pub struct APB4FZ1_SPEC;
impl crate::RegisterSpec for APB4FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb4fz1::R](R) reader structure"]
impl crate::Readable for APB4FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb4fz1::W](W) writer structure"]
impl crate::Writable for APB4FZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB4FZ1 to value 0"]
impl crate::Resettable for APB4FZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
