#[doc = "Register `OFR4` reader"]
pub struct R(crate::R<OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR4` writer"]
pub struct W(crate::W<OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR4_SPEC>;
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
impl From<crate::W<OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET4_EN` reader - OFFSET4_EN"]
pub struct OFFSET4_EN_R(crate::FieldReader<bool, bool>);
impl OFFSET4_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFSET4_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET4_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET4_EN` writer - OFFSET4_EN"]
pub struct OFFSET4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_EN_W<'a> {
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
#[doc = "Field `OFFSET4_CH` reader - OFFSET4_CH"]
pub struct OFFSET4_CH_R(crate::FieldReader<u8, u8>);
impl OFFSET4_CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET4_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET4_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET4_CH` writer - OFFSET4_CH"]
pub struct OFFSET4_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `OFFSET4` reader - OFFSET4"]
pub struct OFFSET4_R(crate::FieldReader<u16, u16>);
impl OFFSET4_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET4` writer - OFFSET4"]
pub struct OFFSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - OFFSET4_EN"]
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - OFFSET4_EN"]
    #[inline(always)]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W {
        OFFSET4_EN_W { w: self }
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W {
        OFFSET4_CH_W { w: self }
    }
    #[doc = "Bits 0:11 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W {
        OFFSET4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr4](index.html) module"]
pub struct OFR4_SPEC;
impl crate::RegisterSpec for OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr4::R](R) reader structure"]
impl crate::Readable for OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr4::W](W) writer structure"]
impl crate::Writable for OFR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
