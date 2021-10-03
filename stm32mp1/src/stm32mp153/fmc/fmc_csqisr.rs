#[doc = "Register `FMC_CSQISR` reader"]
pub struct R(crate::R<FMC_CSQISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQISR` writer"]
pub struct W(crate::W<FMC_CSQISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQISR_SPEC>;
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
impl From<crate::W<FMC_CSQISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCF` reader - TCF"]
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` writer - TCF"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
#[doc = "Field `SCF` reader - SCF"]
pub struct SCF_R(crate::FieldReader<bool, bool>);
impl SCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCF` writer - SCF"]
pub struct SCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCF_W<'a> {
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
#[doc = "Field `SEF` reader - SEF"]
pub struct SEF_R(crate::FieldReader<bool, bool>);
impl SEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEF` writer - SEF"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
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
#[doc = "Field `SUEF` reader - SUEF"]
pub struct SUEF_R(crate::FieldReader<bool, bool>);
impl SUEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUEF` writer - SUEF"]
pub struct SUEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SUEF_W<'a> {
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
#[doc = "Field `CMDTCF` reader - CMDTCF"]
pub struct CMDTCF_R(crate::FieldReader<bool, bool>);
impl CMDTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDTCF` writer - CMDTCF"]
pub struct CMDTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    pub fn suef(&self) -> SUEF_R {
        SUEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    pub fn cmdtcf(&self) -> CMDTCF_R {
        CMDTCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    pub fn scf(&mut self) -> SCF_W {
        SCF_W { w: self }
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    pub fn suef(&mut self) -> SUEF_W {
        SUEF_W { w: self }
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    pub fn cmdtcf(&mut self) -> CMDTCF_W {
        CMDTCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqisr](index.html) module"]
pub struct FMC_CSQISR_SPEC;
impl crate::RegisterSpec for FMC_CSQISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqisr::R](R) reader structure"]
impl crate::Readable for FMC_CSQISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqisr::W](W) writer structure"]
impl crate::Writable for FMC_CSQISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQISR to value 0"]
impl crate::Resettable for FMC_CSQISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
