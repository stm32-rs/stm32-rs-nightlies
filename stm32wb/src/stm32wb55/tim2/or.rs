#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI4_RMP` reader - Input capture 4 remap"]
pub struct TI4_RMP_R(crate::FieldReader<u8, u8>);
impl TI4_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI4_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI4_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI4_RMP` writer - Input capture 4 remap"]
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ETR_RMP` reader - External trigger remap"]
pub struct ETR_RMP_R(crate::FieldReader<bool, bool>);
impl ETR_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETR_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_RMP` writer - External trigger remap"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
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
#[doc = "Field `ITR_RMP` reader - Internal trigger remap"]
pub struct ITR_RMP_R(crate::FieldReader<bool, bool>);
impl ITR_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITR_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITR_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITR_RMP` writer - Internal trigger remap"]
pub struct ITR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ITR_RMP_W<'a> {
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
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Internal trigger remap"]
    #[inline(always)]
    pub fn itr_rmp(&self) -> ITR_RMP_R {
        ITR_RMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bit 0 - Internal trigger remap"]
    #[inline(always)]
    pub fn itr_rmp(&mut self) -> ITR_RMP_W {
        ITR_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
