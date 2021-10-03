#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GIE` reader - Global interrupt enable"]
pub struct GIE_R(crate::FieldReader<bool, bool>);
impl GIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIE` writer - Global interrupt enable"]
pub struct GIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIE_W<'a> {
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
#[doc = "Field `GECCSEIE_` reader - Global ECC single error interrupt enable"]
pub struct GECCSEIE__R(crate::FieldReader<bool, bool>);
impl GECCSEIE__R {
    pub(crate) fn new(bits: bool) -> Self {
        GECCSEIE__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GECCSEIE__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GECCSEIE_` writer - Global ECC single error interrupt enable"]
pub struct GECCSEIE__W<'a> {
    w: &'a mut W,
}
impl<'a> GECCSEIE__W<'a> {
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
#[doc = "Field `GECCDEIE` reader - Global ECC double error interrupt enable"]
pub struct GECCDEIE_R(crate::FieldReader<bool, bool>);
impl GECCDEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GECCDEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GECCDEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GECCDEIE` writer - Global ECC double error interrupt enable"]
pub struct GECCDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GECCDEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GECCDEBWIE` reader - Global ECC double error on byte write (BW) interrupt enable"]
pub struct GECCDEBWIE_R(crate::FieldReader<bool, bool>);
impl GECCDEBWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GECCDEBWIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GECCDEBWIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GECCDEBWIE` writer - Global ECC double error on byte write (BW) interrupt enable"]
pub struct GECCDEBWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GECCDEBWIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&self) -> GECCSEIE__R {
        GECCSEIE__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W {
        GIE_W { w: self }
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&mut self) -> GECCSEIE__W {
        GECCSEIE__W { w: self }
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&mut self) -> GECCDEIE_W {
        GECCDEIE_W { w: self }
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W {
        GECCDEBWIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
