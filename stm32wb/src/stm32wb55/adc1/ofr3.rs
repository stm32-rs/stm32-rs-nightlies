#[doc = "Register `OFR3` reader"]
pub struct R(crate::R<OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR3` writer"]
pub struct W(crate::W<OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR3_SPEC>;
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
impl From<crate::W<OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET3_EN` reader - ADC offset number 3 enable"]
pub struct OFFSET3_EN_R(crate::FieldReader<bool, bool>);
impl OFFSET3_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFSET3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET3_EN` writer - ADC offset number 3 enable"]
pub struct OFFSET3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_EN_W<'a> {
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
#[doc = "Field `OFFSET3_CH` reader - ADC offset number 3 channel selection"]
pub struct OFFSET3_CH_R(crate::FieldReader<u8, u8>);
impl OFFSET3_CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET3_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET3_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET3_CH` writer - ADC offset number 3 channel selection"]
pub struct OFFSET3_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `OFFSET3` reader - ADC offset number 3 offset level"]
pub struct OFFSET3_R(crate::FieldReader<u16, u16>);
impl OFFSET3_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET3` writer - ADC offset number 3 offset level"]
pub struct OFFSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - ADC offset number 3 enable"]
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - ADC offset number 3 channel selection"]
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - ADC offset number 3 offset level"]
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - ADC offset number 3 enable"]
    #[inline(always)]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W {
        OFFSET3_EN_W { w: self }
    }
    #[doc = "Bits 26:30 - ADC offset number 3 channel selection"]
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W {
        OFFSET3_CH_W { w: self }
    }
    #[doc = "Bits 0:11 - ADC offset number 3 offset level"]
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W {
        OFFSET3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC offset number 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr3](index.html) module"]
pub struct OFR3_SPEC;
impl crate::RegisterSpec for OFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr3::R](R) reader structure"]
impl crate::Readable for OFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr3::W](W) writer structure"]
impl crate::Writable for OFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR3 to value 0"]
impl crate::Resettable for OFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
