#[doc = "Register `FMC_BCHIER` reader"]
pub struct R(crate::R<FMC_BCHIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_BCHIER` writer"]
pub struct W(crate::W<FMC_BCHIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BCHIER_SPEC>;
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
impl From<crate::W<FMC_BCHIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BCHIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUEIE` reader - DUEIE"]
pub struct DUEIE_R(crate::FieldReader<bool, bool>);
impl DUEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUEIE` writer - DUEIE"]
pub struct DUEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUEIE_W<'a> {
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
#[doc = "Field `DERIE` reader - DERIE"]
pub struct DERIE_R(crate::FieldReader<bool, bool>);
impl DERIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DERIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERIE` writer - DERIE"]
pub struct DERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERIE_W<'a> {
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
#[doc = "Field `DEFIE` reader - DEFIE"]
pub struct DEFIE_R(crate::FieldReader<bool, bool>);
impl DEFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFIE` writer - DEFIE"]
pub struct DEFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFIE_W<'a> {
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
#[doc = "Field `DSRIE` reader - DSRIE"]
pub struct DSRIE_R(crate::FieldReader<bool, bool>);
impl DSRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRIE` writer - DSRIE"]
pub struct DSRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRIE_W<'a> {
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
#[doc = "Field `EPBRIE` reader - EPBRIE"]
pub struct EPBRIE_R(crate::FieldReader<bool, bool>);
impl EPBRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPBRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPBRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPBRIE` writer - EPBRIE"]
pub struct EPBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBRIE_W<'a> {
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
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    pub fn dueie(&self) -> DUEIE_R {
        DUEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    pub fn derie(&self) -> DERIE_R {
        DERIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    pub fn dsrie(&self) -> DSRIE_R {
        DSRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    pub fn epbrie(&self) -> EPBRIE_R {
        EPBRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    pub fn dueie(&mut self) -> DUEIE_W {
        DUEIE_W { w: self }
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    pub fn derie(&mut self) -> DERIE_W {
        DERIE_W { w: self }
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    pub fn defie(&mut self) -> DEFIE_W {
        DEFIE_W { w: self }
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    pub fn dsrie(&mut self) -> DSRIE_W {
        DSRIE_W { w: self }
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    pub fn epbrie(&mut self) -> EPBRIE_W {
        EPBRIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC BCH Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchier](index.html) module"]
pub struct FMC_BCHIER_SPEC;
impl crate::RegisterSpec for FMC_BCHIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchier::R](R) reader structure"]
impl crate::Readable for FMC_BCHIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_bchier::W](W) writer structure"]
impl crate::Writable for FMC_BCHIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_BCHIER to value 0"]
impl crate::Resettable for FMC_BCHIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
