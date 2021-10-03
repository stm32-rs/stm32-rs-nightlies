#[doc = "Register `I2C_TIMEOUTR` reader"]
pub struct R(crate::R<I2C_TIMEOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TIMEOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TIMEOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TIMEOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TIMEOUTR` writer"]
pub struct W(crate::W<I2C_TIMEOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TIMEOUTR_SPEC>;
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
impl From<crate::W<I2C_TIMEOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TIMEOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUTA` reader - TIMEOUTA"]
pub struct TIMEOUTA_R(crate::FieldReader<u16, u16>);
impl TIMEOUTA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUTA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUTA` writer - TIMEOUTA"]
pub struct TIMEOUTA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `TIDLE` reader - TIDLE"]
pub struct TIDLE_R(crate::FieldReader<bool, bool>);
impl TIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIDLE` writer - TIDLE"]
pub struct TIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIDLE_W<'a> {
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
#[doc = "Field `TIMOUTEN` reader - TIMOUTEN"]
pub struct TIMOUTEN_R(crate::FieldReader<bool, bool>);
impl TIMOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMOUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMOUTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMOUTEN` writer - TIMOUTEN"]
pub struct TIMOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TIMEOUTB` reader - TIMEOUTB"]
pub struct TIMEOUTB_R(crate::FieldReader<u16, u16>);
impl TIMEOUTB_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUTB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUTB` writer - TIMEOUTB"]
pub struct TIMEOUTB_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `TEXTEN` reader - TEXTEN"]
pub struct TEXTEN_R(crate::FieldReader<bool, bool>);
impl TEXTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEXTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEXTEN` writer - TEXTEN"]
pub struct TEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXTEN_W<'a> {
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
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W {
        TIMEOUTA_W { w: self }
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W {
        TIDLE_W { w: self }
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    pub fn timouten(&mut self) -> TIMOUTEN_W {
        TIMOUTEN_W { w: self }
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W {
        TIMEOUTB_W { w: self }
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W {
        TEXTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timeoutr](index.html) module"]
pub struct I2C_TIMEOUTR_SPEC;
impl crate::RegisterSpec for I2C_TIMEOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_timeoutr::R](R) reader structure"]
impl crate::Readable for I2C_TIMEOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_timeoutr::W](W) writer structure"]
impl crate::Writable for I2C_TIMEOUTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TIMEOUTR to value 0"]
impl crate::Resettable for I2C_TIMEOUTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
