#[doc = "Register `APB4FZ2` reader"]
pub struct R(crate::R<APB4FZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4FZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4FZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4FZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB4FZ2` writer"]
pub struct W(crate::W<APB4FZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4FZ2_SPEC>;
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
impl From<crate::W<APB4FZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4FZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_I2C4` reader - I2C4 SMBUS timeout stop in debug"]
pub struct DBG_I2C4_R(crate::FieldReader<bool, bool>);
impl DBG_I2C4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C4` writer - I2C4 SMBUS timeout stop in debug"]
pub struct DBG_I2C4_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C4_W<'a> {
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
#[doc = "Field `DBG_LPTIM2` reader - LPTIM2 stop in debug"]
pub struct DBG_LPTIM2_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM2` writer - LPTIM2 stop in debug"]
pub struct DBG_LPTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM2_W<'a> {
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
#[doc = "Field `DBG_LPTIM3` reader - LPTIM2 stop in debug"]
pub struct DBG_LPTIM3_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM3` writer - LPTIM2 stop in debug"]
pub struct DBG_LPTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM3_W<'a> {
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
#[doc = "Field `DBG_LPTIM4` reader - LPTIM4 stop in debug"]
pub struct DBG_LPTIM4_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM4` writer - LPTIM4 stop in debug"]
pub struct DBG_LPTIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DBG_LPTIM5` reader - LPTIM5 stop in debug"]
pub struct DBG_LPTIM5_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM5` writer - LPTIM5 stop in debug"]
pub struct DBG_LPTIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DBG_RTC` reader - RTC stop in debug"]
pub struct DBG_RTC_R(crate::FieldReader<bool, bool>);
impl DBG_RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_RTC` writer - RTC stop in debug"]
pub struct DBG_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_W<'a> {
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
#[doc = "Field `DBG_WDGLSD1` reader - LS watchdog for D1 stop in debug"]
pub struct DBG_WDGLSD1_R(crate::FieldReader<bool, bool>);
impl DBG_WDGLSD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WDGLSD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_WDGLSD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_WDGLSD1` writer - LS watchdog for D1 stop in debug"]
pub struct DBG_WDGLSD1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WDGLSD1_W<'a> {
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
#[doc = "Field `DBG_WDGLSD2` reader - LS watchdog for D2 stop in debug"]
pub struct DBG_WDGLSD2_R(crate::FieldReader<bool, bool>);
impl DBG_WDGLSD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WDGLSD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_WDGLSD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_WDGLSD2` writer - LS watchdog for D2 stop in debug"]
pub struct DBG_WDGLSD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WDGLSD2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4(&self) -> DBG_I2C4_R {
        DBG_I2C4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2(&self) -> DBG_LPTIM2_R {
        DBG_LPTIM2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3(&self) -> DBG_LPTIM3_R {
        DBG_LPTIM3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4(&self) -> DBG_LPTIM4_R {
        DBG_LPTIM4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5(&self) -> DBG_LPTIM5_R {
        DBG_LPTIM5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc(&self) -> DBG_RTC_R {
        DBG_RTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LS watchdog for D1 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd1(&self) -> DBG_WDGLSD1_R {
        DBG_WDGLSD1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LS watchdog for D2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd2(&self) -> DBG_WDGLSD2_R {
        DBG_WDGLSD2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4(&mut self) -> DBG_I2C4_W {
        DBG_I2C4_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2(&mut self) -> DBG_LPTIM2_W {
        DBG_LPTIM2_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3(&mut self) -> DBG_LPTIM3_W {
        DBG_LPTIM3_W { w: self }
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4(&mut self) -> DBG_LPTIM4_W {
        DBG_LPTIM4_W { w: self }
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5(&mut self) -> DBG_LPTIM5_W {
        DBG_LPTIM5_W { w: self }
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc(&mut self) -> DBG_RTC_W {
        DBG_RTC_W { w: self }
    }
    #[doc = "Bit 18 - LS watchdog for D1 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd1(&mut self) -> DBG_WDGLSD1_W {
        DBG_WDGLSD1_W { w: self }
    }
    #[doc = "Bit 19 - LS watchdog for D2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd2(&mut self) -> DBG_WDGLSD2_W {
        DBG_WDGLSD2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB4 peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4fz2](index.html) module"]
pub struct APB4FZ2_SPEC;
impl crate::RegisterSpec for APB4FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb4fz2::R](R) reader structure"]
impl crate::Readable for APB4FZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb4fz2::W](W) writer structure"]
impl crate::Writable for APB4FZ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB4FZ2 to value 0"]
impl crate::Resettable for APB4FZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
