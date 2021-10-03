#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSLEEP_CD` reader - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLEEP_CD_R(crate::FieldReader<bool, bool>);
impl DBGSLEEP_CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSLEEP_CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSLEEP_CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSLEEP_CD` writer - Allow D1 domain debug in Sleep mode"]
pub struct DBGSLEEP_CD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSLEEP_CD_W<'a> {
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
#[doc = "Field `DBGSTOP_CD` reader - Allow D1 domain debug in Stop mode"]
pub struct DBGSTOP_CD_R(crate::FieldReader<bool, bool>);
impl DBGSTOP_CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTOP_CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTOP_CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTOP_CD` writer - Allow D1 domain debug in Stop mode"]
pub struct DBGSTOP_CD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTOP_CD_W<'a> {
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
#[doc = "Field `DBGSTBY_CD` reader - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBY_CD_R(crate::FieldReader<bool, bool>);
impl DBGSTBY_CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBY_CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBY_CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBY_CD` writer - Allow D1 domain debug in Standby mode"]
pub struct DBGSTBY_CD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBY_CD_W<'a> {
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
#[doc = "Field `DBGSTOP_SRD` reader - debug in SmartRun domain Stop mode"]
pub struct DBGSTOP_SRD_R(crate::FieldReader<bool, bool>);
impl DBGSTOP_SRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTOP_SRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTOP_SRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTOP_SRD` writer - debug in SmartRun domain Stop mode"]
pub struct DBGSTOP_SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTOP_SRD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DBGSTBY_SRD` reader - debug in SmartRun domain Standby mode"]
pub struct DBGSTBY_SRD_R(crate::FieldReader<bool, bool>);
impl DBGSTBY_SRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSTBY_SRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSTBY_SRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSTBY_SRD` writer - debug in SmartRun domain Standby mode"]
pub struct DBGSTBY_SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBY_SRD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub struct TRACECLKEN_R(crate::FieldReader<bool, bool>);
impl TRACECLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACECLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACECLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub struct TRACECLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CDDBGCKEN` reader - CPU domain debug clock enable"]
pub struct CDDBGCKEN_R(crate::FieldReader<bool, bool>);
impl CDDBGCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDDBGCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDDBGCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDDBGCKEN` writer - CPU domain debug clock enable"]
pub struct CDDBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDDBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SRDDBGCKEN` reader - SmartRun domain debug clock enable"]
pub struct SRDDBGCKEN_R(crate::FieldReader<bool, bool>);
impl SRDDBGCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRDDBGCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRDDBGCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRDDBGCKEN` writer - SmartRun domain debug clock enable"]
pub struct SRDDBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRDDBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub struct TRGOEN_R(crate::FieldReader<bool, bool>);
impl TRGOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub struct TRGOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_cd(&self) -> DBGSLEEP_CD_R {
        DBGSLEEP_CD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_cd(&self) -> DBGSTOP_CD_R {
        DBGSTOP_CD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_cd(&self) -> DBGSTBY_CD_R {
        DBGSTBY_CD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    pub fn dbgstop_srd(&self) -> DBGSTOP_SRD_R {
        DBGSTOP_SRD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    pub fn dbgstby_srd(&self) -> DBGSTBY_SRD_R {
        DBGSTBY_SRD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    pub fn cddbgcken(&self) -> CDDBGCKEN_R {
        CDDBGCKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    pub fn srddbgcken(&self) -> SRDDBGCKEN_R {
        SRDDBGCKEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_cd(&mut self) -> DBGSLEEP_CD_W {
        DBGSLEEP_CD_W { w: self }
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_cd(&mut self) -> DBGSTOP_CD_W {
        DBGSTOP_CD_W { w: self }
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_cd(&mut self) -> DBGSTBY_CD_W {
        DBGSTBY_CD_W { w: self }
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    pub fn dbgstop_srd(&mut self) -> DBGSTOP_SRD_W {
        DBGSTOP_SRD_W { w: self }
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    pub fn dbgstby_srd(&mut self) -> DBGSTBY_SRD_W {
        DBGSTBY_SRD_W { w: self }
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W {
        TRACECLKEN_W { w: self }
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    pub fn cddbgcken(&mut self) -> CDDBGCKEN_W {
        CDDBGCKEN_W { w: self }
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    pub fn srddbgcken(&mut self) -> SRDDBGCKEN_W {
        SRDDBGCKEN_W { w: self }
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W {
        TRGOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
