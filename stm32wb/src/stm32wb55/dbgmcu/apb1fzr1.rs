#[doc = "Register `APB1FZR1` reader"]
pub struct R(crate::R<APB1FZR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR1` writer"]
pub struct W(crate::W<APB1FZR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR1_SPEC>;
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
impl From<crate::W<APB1FZR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub struct DBG_TIMER2_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIMER2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIMER2_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIMER2_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub struct DBG_TIMER2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER2_STOP_W<'a> {
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
#[doc = "Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted"]
pub struct DBG_RTC_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_RTC_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RTC_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_RTC_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted"]
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
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
#[doc = "Field `DBG_WWDG_STOP` reader - WWDG counter stopped when core is halted"]
pub struct DBG_WWDG_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_WWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_WWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - WWDG counter stopped when core is halted"]
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
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
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG counter stopped when core is halted"]
pub struct DBG_IWDG_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_IWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_IWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG counter stopped when core is halted"]
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
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
#[doc = "Field `DBG_I2C1_STOP` reader - Debug I2C1 SMBUS timeout stopped when Core is halted"]
pub struct DBG_I2C1_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_I2C1_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C1_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C1_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C1_STOP` writer - Debug I2C1 SMBUS timeout stopped when Core is halted"]
pub struct DBG_I2C1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_STOP_W<'a> {
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
#[doc = "Field `DBG_I2C3_STOP` reader - Debug I2C3 SMBUS timeout stopped when core is halted"]
pub struct DBG_I2C3_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_I2C3_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C3_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C3_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C3_STOP` writer - Debug I2C3 SMBUS timeout stopped when core is halted"]
pub struct DBG_I2C3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C3_STOP_W<'a> {
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
#[doc = "Field `DBG_LPTIM1_STOP` reader - Debug LPTIM1 stopped when Core is halted"]
pub struct DBG_LPTIM1_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM1_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM1_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM1_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - Debug LPTIM1 stopped when Core is halted"]
pub struct DBG_LPTIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM1_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Debug I2C1 SMBUS timeout stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Debug I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W {
        DBG_TIMER2_STOP_W { w: self }
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    #[doc = "Bit 11 - WWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    #[doc = "Bit 12 - IWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 21 - Debug I2C1 SMBUS timeout stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W {
        DBG_I2C1_STOP_W { w: self }
    }
    #[doc = "Bit 23 - Debug I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W {
        DBG_I2C3_STOP_W { w: self }
    }
    #[doc = "Bit 31 - Debug LPTIM1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W {
        DBG_LPTIM1_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 Low Freeze Register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr1](index.html) module"]
pub struct APB1FZR1_SPEC;
impl crate::RegisterSpec for APB1FZR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr1::R](R) reader structure"]
impl crate::Readable for APB1FZR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr1::W](W) writer structure"]
impl crate::Writable for APB1FZR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1FZR1 to value 0"]
impl crate::Resettable for APB1FZR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}