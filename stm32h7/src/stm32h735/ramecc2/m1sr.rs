#[doc = "Register `M1SR` reader"]
pub struct R(crate::R<M1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M1SR` writer"]
pub struct W(crate::W<M1SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M1SR_SPEC>;
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
impl From<crate::W<M1SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M1SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEDCF` reader - ECC single error detected and corrected flag"]
pub struct SEDCF_R(crate::FieldReader<bool, bool>);
impl SEDCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEDCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEDCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEDCF` writer - ECC single error detected and corrected flag"]
pub struct SEDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDCF_W<'a> {
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
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub struct DEDF_R(crate::FieldReader<bool, bool>);
impl DEDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub struct DEDF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDF_W<'a> {
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
#[doc = "Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag"]
pub struct DEBWDF_R(crate::FieldReader<bool, bool>);
impl DEBWDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBWDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBWDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag"]
pub struct DEBWDF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBWDF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W {
        SEDCF_W { w: self }
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W {
        DEDF_W { w: self }
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W {
        DEBWDF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1sr](index.html) module"]
pub struct M1SR_SPEC;
impl crate::RegisterSpec for M1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m1sr::R](R) reader structure"]
impl crate::Readable for M1SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m1sr::W](W) writer structure"]
impl crate::Writable for M1SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M1SR to value 0"]
impl crate::Resettable for M1SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
