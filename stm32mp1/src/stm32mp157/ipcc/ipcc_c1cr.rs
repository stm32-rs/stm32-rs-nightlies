#[doc = "Register `IPCC_C1CR` reader"]
pub struct R(crate::R<IPCC_C1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCC_C1CR` writer"]
pub struct W(crate::W<IPCC_C1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C1CR_SPEC>;
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
impl From<crate::W<IPCC_C1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOIE` reader - RXOIE"]
pub struct RXOIE_R(crate::FieldReader<bool, bool>);
impl RXOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIE` writer - RXOIE"]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
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
#[doc = "Field `TXFIE` reader - TXFIE"]
pub struct TXFIE_R(crate::FieldReader<bool, bool>);
impl TXFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIE` writer - TXFIE"]
pub struct TXFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W {
        TXFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1cr](index.html) module"]
pub struct IPCC_C1CR_SPEC;
impl crate::RegisterSpec for IPCC_C1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c1cr::R](R) reader structure"]
impl crate::Readable for IPCC_C1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcc_c1cr::W](W) writer structure"]
impl crate::Writable for IPCC_C1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPCC_C1CR to value 0"]
impl crate::Resettable for IPCC_C1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
