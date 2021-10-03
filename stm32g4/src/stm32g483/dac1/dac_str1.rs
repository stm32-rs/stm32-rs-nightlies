#[doc = "Register `DAC_STR1` reader"]
pub struct R(crate::R<DAC_STR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_STR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_STR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STR1` writer"]
pub struct W(crate::W<DAC_STR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STR1_SPEC>;
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
impl From<crate::W<DAC_STR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_STR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRSTDATA1` reader - DAC Channel 1 Sawtooth reset value"]
pub struct STRSTDATA1_R(crate::FieldReader<u16, u16>);
impl STRSTDATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        STRSTDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRSTDATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRSTDATA1` writer - DAC Channel 1 Sawtooth reset value"]
pub struct STRSTDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `STDIR1` reader - DAC Channel1 Sawtooth direction setting"]
pub struct STDIR1_R(crate::FieldReader<bool, bool>);
impl STDIR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STDIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STDIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STDIR1` writer - DAC Channel1 Sawtooth direction setting"]
pub struct STDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> STDIR1_W<'a> {
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
#[doc = "Field `STINCDATA1` reader - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub struct STINCDATA1_R(crate::FieldReader<u16, u16>);
impl STINCDATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        STINCDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STINCDATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STINCDATA1` writer - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub struct STINCDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&self) -> STRSTDATA1_R {
        STRSTDATA1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&self) -> STDIR1_R {
        STDIR1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&self) -> STINCDATA1_R {
        STINCDATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&mut self) -> STRSTDATA1_W {
        STRSTDATA1_W { w: self }
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&mut self) -> STDIR1_W {
        STDIR1_W { w: self }
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&mut self) -> STINCDATA1_W {
        STINCDATA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sawtooth register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_str1](index.html) module"]
pub struct DAC_STR1_SPEC;
impl crate::RegisterSpec for DAC_STR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_str1::R](R) reader structure"]
impl crate::Readable for DAC_STR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_str1::W](W) writer structure"]
impl crate::Writable for DAC_STR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_STR1 to value 0"]
impl crate::Resettable for DAC_STR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
