#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub struct ENCODER_MODE_R(crate::FieldReader<u8, u8>);
impl ENCODER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENCODER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCODER_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub struct ENCODER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODER_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `TI1_RMP` reader - Input capture 1 remap"]
pub struct TI1_RMP_R(crate::FieldReader<bool, bool>);
impl TI1_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI1_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI1_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1_RMP` writer - Input capture 1 remap"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
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
impl R {
    #[doc = "Bits 1:2 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Input capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W {
        ENCODER_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Input capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
