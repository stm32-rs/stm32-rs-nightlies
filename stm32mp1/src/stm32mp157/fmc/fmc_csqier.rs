#[doc = "Register `FMC_CSQIER` reader"]
pub struct R(crate::R<FMC_CSQIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQIER` writer"]
pub struct W(crate::W<FMC_CSQIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQIER_SPEC>;
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
impl From<crate::W<FMC_CSQIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCIE` reader - TCIE"]
pub struct TCIE_R(crate::FieldReader<bool, bool>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - TCIE"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
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
#[doc = "Field `SCIE` reader - SCIE"]
pub struct SCIE_R(crate::FieldReader<bool, bool>);
impl SCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCIE` writer - SCIE"]
pub struct SCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIE_W<'a> {
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
#[doc = "Field `SEIE` reader - SEIE"]
pub struct SEIE_R(crate::FieldReader<bool, bool>);
impl SEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEIE` writer - SEIE"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
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
#[doc = "Field `SUEIE` reader - SUEIE"]
pub struct SUEIE_R(crate::FieldReader<bool, bool>);
impl SUEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUEIE` writer - SUEIE"]
pub struct SUEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUEIE_W<'a> {
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
#[doc = "Field `CMDTCIE` reader - CMDTCIE"]
pub struct CMDTCIE_R(crate::FieldReader<bool, bool>);
impl CMDTCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDTCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDTCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDTCIE` writer - CMDTCIE"]
pub struct CMDTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTCIE_W<'a> {
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
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    pub fn sueie(&self) -> SUEIE_R {
        SUEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    pub fn cmdtcie(&self) -> CMDTCIE_R {
        CMDTCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    pub fn scie(&mut self) -> SCIE_W {
        SCIE_W { w: self }
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    pub fn sueie(&mut self) -> SUEIE_W {
        SUEIE_W { w: self }
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    pub fn cmdtcie(&mut self) -> CMDTCIE_W {
        CMDTCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqier](index.html) module"]
pub struct FMC_CSQIER_SPEC;
impl crate::RegisterSpec for FMC_CSQIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqier::R](R) reader structure"]
impl crate::Readable for FMC_CSQIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqier::W](W) writer structure"]
impl crate::Writable for FMC_CSQIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQIER to value 0"]
impl crate::Resettable for FMC_CSQIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
