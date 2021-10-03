#[doc = "Register `DMAISR` reader"]
pub struct R(crate::R<DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAISR` writer"]
pub struct W(crate::W<DMAISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAISR_SPEC>;
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
impl From<crate::W<DMAISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub struct MACIS_R(crate::FieldReader<bool, bool>);
impl MACIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MACIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACIS` writer - MAC Interrupt Status"]
pub struct MACIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MACIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub struct MTLIS_R(crate::FieldReader<bool, bool>);
impl MTLIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTLIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTLIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTLIS` writer - MTL Interrupt Status"]
pub struct MTLIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MTLIS_W<'a> {
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
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub struct DC0IS_R(crate::FieldReader<bool, bool>);
impl DC0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC0IS` writer - DMA Channel Interrupt Status"]
pub struct DC0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC0IS_W<'a> {
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
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&mut self) -> MACIS_W {
        MACIS_W { w: self }
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&mut self) -> MTLIS_W {
        MTLIS_W { w: self }
    }
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&mut self) -> DC0IS_W {
        DC0IS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](index.html) module"]
pub struct DMAISR_SPEC;
impl crate::RegisterSpec for DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaisr::R](R) reader structure"]
impl crate::Readable for DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaisr::W](W) writer structure"]
impl crate::Writable for DMAISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
