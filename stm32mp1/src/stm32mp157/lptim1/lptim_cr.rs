#[doc = "Register `LPTIM_CR` reader"]
pub struct R(crate::R<LPTIM_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CR` writer"]
pub struct W(crate::W<LPTIM_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CR_SPEC>;
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
impl From<crate::W<LPTIM_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - ENABLE"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - ENABLE"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `SNGSTRT` reader - SNGSTRT"]
pub struct SNGSTRT_R(crate::FieldReader<bool, bool>);
impl SNGSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNGSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNGSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNGSTRT` writer - SNGSTRT"]
pub struct SNGSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SNGSTRT_W<'a> {
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
#[doc = "Field `CNTSTRT` reader - CNTSTRT"]
pub struct CNTSTRT_R(crate::FieldReader<bool, bool>);
impl CNTSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTSTRT` writer - CNTSTRT"]
pub struct CNTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTRT_W<'a> {
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
#[doc = "Field `COUNTRST` reader - COUNTRST"]
pub struct COUNTRST_R(crate::FieldReader<bool, bool>);
impl COUNTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTRST` writer - COUNTRST"]
pub struct COUNTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTRST_W<'a> {
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
#[doc = "Field `RSTARE` reader - RSTARE"]
pub struct RSTARE_R(crate::FieldReader<bool, bool>);
impl RSTARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTARE` writer - RSTARE"]
pub struct RSTARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTARE_W<'a> {
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
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W {
        SNGSTRT_W { w: self }
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W {
        CNTSTRT_W { w: self }
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    pub fn countrst(&mut self) -> COUNTRST_W {
        COUNTRST_W { w: self }
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    pub fn rstare(&mut self) -> RSTARE_W {
        RSTARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cr](index.html) module"]
pub struct LPTIM_CR_SPEC;
impl crate::RegisterSpec for LPTIM_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cr::R](R) reader structure"]
impl crate::Readable for LPTIM_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cr::W](W) writer structure"]
impl crate::Writable for LPTIM_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_CR to value 0"]
impl crate::Resettable for LPTIM_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
