#[doc = "Register `CPUCR` reader"]
pub struct R(crate::R<CPUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCR` writer"]
pub struct W(crate::W<CPUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCR_SPEC>;
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
impl From<crate::W<CPUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETDS_CD` reader - RETDS_CD"]
pub struct RETDS_CD_R(crate::FieldReader<bool, bool>);
impl RETDS_CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETDS_CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETDS_CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETDS_CD` writer - RETDS_CD"]
pub struct RETDS_CD_W<'a> {
    w: &'a mut W,
}
impl<'a> RETDS_CD_W<'a> {
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
#[doc = "Field `PDDS_SRD` reader - PDDS_SRD"]
pub struct PDDS_SRD_R(crate::FieldReader<bool, bool>);
impl PDDS_SRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_SRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDS_SRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS_SRD` writer - PDDS_SRD"]
pub struct PDDS_SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_SRD_W<'a> {
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
#[doc = "Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
pub struct STOPF_R(crate::FieldReader<bool, bool>);
impl STOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
pub struct SBF_R(crate::FieldReader<bool, bool>);
impl SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub struct CSSF_R(crate::FieldReader<bool, bool>);
impl CSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub struct CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RUN_SRD` reader - RUN_SRD"]
pub struct RUN_SRD_R(crate::FieldReader<bool, bool>);
impl RUN_SRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN_SRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUN_SRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN_SRD` writer - RUN_SRD"]
pub struct RUN_SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_SRD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    pub fn retds_cd(&self) -> RETDS_CD_R {
        RETDS_CD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    pub fn pdds_srd(&self) -> PDDS_SRD_R {
        PDDS_SRD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    pub fn run_srd(&self) -> RUN_SRD_R {
        RUN_SRD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    pub fn retds_cd(&mut self) -> RETDS_CD_W {
        RETDS_CD_W { w: self }
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    pub fn pdds_srd(&mut self) -> PDDS_SRD_W {
        PDDS_SRD_W { w: self }
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W {
        CSSF_W { w: self }
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    pub fn run_srd(&mut self) -> RUN_SRD_W {
        RUN_SRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows controlling CPU1 power.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpucr](index.html) module"]
pub struct CPUCR_SPEC;
impl crate::RegisterSpec for CPUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpucr::R](R) reader structure"]
impl crate::Readable for CPUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpucr::W](W) writer structure"]
impl crate::Writable for CPUCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUCR to value 0"]
impl crate::Resettable for CPUCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
