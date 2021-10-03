#[doc = "Register `DLYB_CR` reader"]
pub struct R(crate::R<DLYB_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYB_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYB_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYB_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYB_CR` writer"]
pub struct W(crate::W<DLYB_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYB_CR_SPEC>;
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
impl From<crate::W<DLYB_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYB_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEN` reader - DEN"]
pub struct DEN_R(crate::FieldReader<bool, bool>);
impl DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEN` writer - DEN"]
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
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
#[doc = "Field `SEN` reader - SEN"]
pub struct SEN_R(crate::FieldReader<bool, bool>);
impl SEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEN` writer - SEN"]
pub struct SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEN"]
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    #[doc = "Bit 1 - SEN"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLYB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cr](index.html) module"]
pub struct DLYB_CR_SPEC;
impl crate::RegisterSpec for DLYB_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyb_cr::R](R) reader structure"]
impl crate::Readable for DLYB_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyb_cr::W](W) writer structure"]
impl crate::Writable for DLYB_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLYB_CR to value 0"]
impl crate::Resettable for DLYB_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
