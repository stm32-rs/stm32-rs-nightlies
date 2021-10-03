#[doc = "Register `EXTSCR` reader"]
pub struct R(crate::R<EXTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTSCR` writer"]
pub struct W(crate::W<EXTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSCR_SPEC>;
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
impl From<crate::W<EXTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C2DS` reader - CPU2 deepsleep mode"]
pub struct C2DS_R(crate::FieldReader<bool, bool>);
impl C2DS_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2DS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1DS` reader - CPU1 deepsleep mode"]
pub struct C1DS_R(crate::FieldReader<bool, bool>);
impl C1DS_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1DS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRPF` reader - Critical Radio system phase"]
pub struct CRPF_R(crate::FieldReader<bool, bool>);
impl CRPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2STOPF` reader - System Stop flag for CPU2"]
pub struct C2STOPF_R(crate::FieldReader<bool, bool>);
impl C2STOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2STOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2STOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2SBF` reader - System Standby flag for CPU2"]
pub struct C2SBF_R(crate::FieldReader<bool, bool>);
impl C2SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2SBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2SBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1STOPF` reader - System Stop flag for CPU1"]
pub struct C1STOPF_R(crate::FieldReader<bool, bool>);
impl C1STOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1STOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1STOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1SBF` reader - System Standby flag for CPU1"]
pub struct C1SBF_R(crate::FieldReader<bool, bool>);
impl C1SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1SBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1SBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRPF` writer - Clear Critical Radio system phase"]
pub struct CCRPF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRPF_W<'a> {
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
#[doc = "Field `C2CSSF` writer - Clear CPU2 Stop Standby flags"]
pub struct C2CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> C2CSSF_W<'a> {
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
#[doc = "Field `C1CSSF` writer - Clear CPU1 Stop Standby flags"]
pub struct C1CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CSSF_W<'a> {
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
    #[doc = "Bit 15 - CPU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Critical Radio system phase"]
    #[inline(always)]
    pub fn crpf(&self) -> CRPF_R {
        CRPF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - System Stop flag for CPU2"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System Standby flag for CPU2"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - System Stop flag for CPU1"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - System Standby flag for CPU1"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clear Critical Radio system phase"]
    #[inline(always)]
    pub fn ccrpf(&mut self) -> CCRPF_W {
        CCRPF_W { w: self }
    }
    #[doc = "Bit 1 - Clear CPU2 Stop Standby flags"]
    #[inline(always)]
    pub fn c2cssf(&mut self) -> C2CSSF_W {
        C2CSSF_W { w: self }
    }
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    pub fn c1cssf(&mut self) -> C1CSSF_W {
        C1CSSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extscr](index.html) module"]
pub struct EXTSCR_SPEC;
impl crate::RegisterSpec for EXTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extscr::R](R) reader structure"]
impl crate::Readable for EXTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extscr::W](W) writer structure"]
impl crate::Writable for EXTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTSCR to value 0"]
impl crate::Resettable for EXTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
