#[doc = "Register `DEEFR3` reader"]
pub struct R(crate::R<DEEFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEEFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEEFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEEFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEEFR3` writer"]
pub struct W(crate::W<DEEFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEEFR3_SPEC>;
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
impl From<crate::W<DEEFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEEFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEVACNT` reader - External Event A counter"]
pub struct EEVACNT_R(crate::FieldReader<u8, u8>);
impl EEVACNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVACNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVACNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVACNT` writer - External Event A counter"]
pub struct EEVACNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `EEVASEL` reader - External Event A Selection"]
pub struct EEVASEL_R(crate::FieldReader<u8, u8>);
impl EEVASEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVASEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVASEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVASEL` writer - External Event A Selection"]
pub struct EEVASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVASEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `EEVARSTM` reader - External Event A Reset Mode"]
pub struct EEVARSTM_R(crate::FieldReader<bool, bool>);
impl EEVARSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEVARSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVARSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVARSTM` writer - External Event A Reset Mode"]
pub struct EEVARSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVARSTM_W<'a> {
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
#[doc = "Field `EEVACRES` reader - External Event A Counter Reset"]
pub struct EEVACRES_R(crate::FieldReader<bool, bool>);
impl EEVACRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEVACRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVACRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVACRES` writer - External Event A Counter Reset"]
pub struct EEVACRES_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACRES_W<'a> {
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
#[doc = "Field `EEVACE` reader - External Event A Counter Enable"]
pub struct EEVACE_R(crate::FieldReader<bool, bool>);
impl EEVACE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEVACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVACE` writer - External Event A Counter Enable"]
pub struct EEVACE_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACE_W<'a> {
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
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&mut self) -> EEVACNT_W {
        EEVACNT_W { w: self }
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&mut self) -> EEVASEL_W {
        EEVASEL_W { w: self }
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&mut self) -> EEVARSTM_W {
        EEVARSTM_W { w: self }
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&mut self) -> EEVACRES_W {
        EEVACRES_W { w: self }
    }
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&mut self) -> EEVACE_W {
        EEVACE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deefr3](index.html) module"]
pub struct DEEFR3_SPEC;
impl crate::RegisterSpec for DEEFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deefr3::R](R) reader structure"]
impl crate::Readable for DEEFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deefr3::W](W) writer structure"]
impl crate::Writable for DEEFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEEFR3 to value 0"]
impl crate::Resettable for DEEFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
