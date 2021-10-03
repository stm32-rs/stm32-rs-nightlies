#[doc = "Register `PWR_MPUCR` reader"]
pub struct R(crate::R<PWR_MPUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_MPUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_MPUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_MPUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_MPUCR` writer"]
pub struct W(crate::W<PWR_MPUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_MPUCR_SPEC>;
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
impl From<crate::W<PWR_MPUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_MPUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDS` reader - PDDS"]
pub struct PDDS_R(crate::FieldReader<bool, bool>);
impl PDDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS` writer - PDDS"]
pub struct PDDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_W<'a> {
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
#[doc = "Field `CSTBYDIS` reader - CSTBYDIS"]
pub struct CSTBYDIS_R(crate::FieldReader<bool, bool>);
impl CSTBYDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTBYDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTBYDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTBYDIS` writer - CSTBYDIS"]
pub struct CSTBYDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTBYDIS_W<'a> {
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
#[doc = "Field `STOPF` reader - STOPF"]
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
#[doc = "Field `SBF` reader - SBF"]
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
#[doc = "Field `SBFMPU` reader - SBFMPU"]
pub struct SBFMPU_R(crate::FieldReader<bool, bool>);
impl SBFMPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBFMPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBFMPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSF` reader - CSSF"]
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
#[doc = "Field `CSSF` writer - CSSF"]
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
#[doc = "Field `STANDBYWFIL2` reader - STANDBYWFIL2"]
pub struct STANDBYWFIL2_R(crate::FieldReader<bool, bool>);
impl STANDBYWFIL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBYWFIL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBYWFIL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    pub fn cstbydis(&self) -> CSTBYDIS_R {
        CSTBYDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SBFMPU"]
    #[inline(always)]
    pub fn sbfmpu(&self) -> SBFMPU_R {
        SBFMPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - STANDBYWFIL2"]
    #[inline(always)]
    pub fn standbywfil2(&self) -> STANDBYWFIL2_R {
        STANDBYWFIL2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W { w: self }
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    pub fn cstbydis(&mut self) -> CSTBYDIS_W {
        CSTBYDIS_W { w: self }
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W {
        CSSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mpucr](index.html) module"]
pub struct PWR_MPUCR_SPEC;
impl crate::RegisterSpec for PWR_MPUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_mpucr::R](R) reader structure"]
impl crate::Readable for PWR_MPUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_mpucr::W](W) writer structure"]
impl crate::Writable for PWR_MPUCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_MPUCR to value 0"]
impl crate::Resettable for PWR_MPUCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
