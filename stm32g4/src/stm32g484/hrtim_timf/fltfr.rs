#[doc = "Register `FLTFR` reader"]
pub struct R(crate::R<FLTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTFR` writer"]
pub struct W(crate::W<FLTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTFR_SPEC>;
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
impl From<crate::W<FLTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub struct FLTLCK_R(crate::FieldReader<bool, bool>);
impl FLTLCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLTLCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTLCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub struct FLTLCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTLCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `FLT6EN` reader - Fault 6 enable"]
pub struct FLT6EN_R(crate::FieldReader<bool, bool>);
impl FLT6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6EN` writer - Fault 6 enable"]
pub struct FLT6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub struct FLT5EN_R(crate::FieldReader<bool, bool>);
impl FLT5EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub struct FLT5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5EN_W<'a> {
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
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub struct FLT4EN_R(crate::FieldReader<bool, bool>);
impl FLT4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub struct FLT4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4EN_W<'a> {
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
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub struct FLT3EN_R(crate::FieldReader<bool, bool>);
impl FLT3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub struct FLT3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3EN_W<'a> {
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
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub struct FLT2EN_R(crate::FieldReader<bool, bool>);
impl FLT2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub struct FLT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2EN_W<'a> {
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
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub struct FLT1EN_R(crate::FieldReader<bool, bool>);
impl FLT1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub struct FLT1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1EN_W<'a> {
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
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    pub fn flt6en(&self) -> FLT6EN_R {
        FLT6EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&mut self) -> FLTLCK_W {
        FLTLCK_W { w: self }
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    pub fn flt6en(&mut self) -> FLT6EN_W {
        FLT6EN_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&mut self) -> FLT5EN_W {
        FLT5EN_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&mut self) -> FLT4EN_W {
        FLT4EN_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> FLT3EN_W {
        FLT3EN_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> FLT2EN_W {
        FLT2EN_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> FLT1EN_W {
        FLT1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltfr](index.html) module"]
pub struct FLTFR_SPEC;
impl crate::RegisterSpec for FLTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltfr::R](R) reader structure"]
impl crate::Readable for FLTFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltfr::W](W) writer structure"]
impl crate::Writable for FLTFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTFR to value 0"]
impl crate::Resettable for FLTFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
