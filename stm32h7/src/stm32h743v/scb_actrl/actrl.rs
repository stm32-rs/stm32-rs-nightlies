#[doc = "Register `ACTRL` reader"]
pub struct R(crate::R<ACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTRL` writer"]
pub struct W(crate::W<ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRL_SPEC>;
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
impl From<crate::W<ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISFOLD` reader - DISFOLD"]
pub struct DISFOLD_R(crate::FieldReader<bool, bool>);
impl DISFOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISFOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISFOLD` writer - DISFOLD"]
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
#[doc = "Field `FPEXCODIS` reader - FPEXCODIS"]
pub struct FPEXCODIS_R(crate::FieldReader<bool, bool>);
impl FPEXCODIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPEXCODIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPEXCODIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPEXCODIS` writer - FPEXCODIS"]
pub struct FPEXCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEXCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DISRAMODE` reader - DISRAMODE"]
pub struct DISRAMODE_R(crate::FieldReader<bool, bool>);
impl DISRAMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISRAMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISRAMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISRAMODE` writer - DISRAMODE"]
pub struct DISRAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISRAMODE_W<'a> {
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
#[doc = "Field `DISITMATBFLUSH` reader - DISITMATBFLUSH"]
pub struct DISITMATBFLUSH_R(crate::FieldReader<bool, bool>);
impl DISITMATBFLUSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISITMATBFLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISITMATBFLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISITMATBFLUSH` writer - DISITMATBFLUSH"]
pub struct DISITMATBFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> DISITMATBFLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W {
        FPEXCODIS_W { w: self }
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    pub fn disramode(&mut self) -> DISRAMODE_W {
        DISRAMODE_W { w: self }
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W {
        DISITMATBFLUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrl](index.html) module"]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actrl::R](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actrl::W](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
