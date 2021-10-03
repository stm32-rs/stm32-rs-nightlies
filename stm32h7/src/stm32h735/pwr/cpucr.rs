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
#[doc = "Field `PDDS_D1` reader - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub struct PDDS_D1_R(crate::FieldReader<bool, bool>);
impl PDDS_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDS_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS_D1` writer - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub struct PDDS_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D1_W<'a> {
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
#[doc = "Field `PDDS_D2` reader - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub struct PDDS_D2_R(crate::FieldReader<bool, bool>);
impl PDDS_D2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_D2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDS_D2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS_D2` writer - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub struct PDDS_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D2_W<'a> {
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
#[doc = "Field `PDDS_D3` reader - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub struct PDDS_D3_R(crate::FieldReader<bool, bool>);
impl PDDS_D3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_D3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDS_D3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS_D3` writer - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub struct PDDS_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_D3_W<'a> {
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
#[doc = "Field `SBF_D1` reader - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
pub struct SBF_D1_R(crate::FieldReader<bool, bool>);
impl SBF_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBF_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBF_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBF_D2` reader - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
pub struct SBF_D2_R(crate::FieldReader<bool, bool>);
impl SBF_D2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBF_D2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBF_D2_R {
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
#[doc = "Field `RUN_D3` reader - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub struct RUN_D3_R(crate::FieldReader<bool, bool>);
impl RUN_D3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN_D3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUN_D3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN_D3` writer - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub struct RUN_D3_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_D3_W<'a> {
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
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&self) -> PDDS_D1_R {
        PDDS_D1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&self) -> PDDS_D2_R {
        PDDS_D2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&self) -> PDDS_D3_R {
        PDDS_D3_R::new(((self.bits >> 2) & 0x01) != 0)
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
    #[doc = "Bit 7 - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d1(&self) -> SBF_D1_R {
        SBF_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d2(&self) -> SBF_D2_R {
        SBF_D2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&self) -> RUN_D3_R {
        RUN_D3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&mut self) -> PDDS_D1_W {
        PDDS_D1_W { w: self }
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&mut self) -> PDDS_D2_W {
        PDDS_D2_W { w: self }
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&mut self) -> PDDS_D3_W {
        PDDS_D3_W { w: self }
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W {
        CSSF_W { w: self }
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&mut self) -> RUN_D3_W {
        RUN_D3_W { w: self }
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
