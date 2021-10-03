#[doc = "Register `COMP7_CSR` reader"]
pub struct R(crate::R<COMP7_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP7_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP7_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP7_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP7_CSR` writer"]
pub struct W(crate::W<COMP7_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP7_CSR_SPEC>;
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
impl From<crate::W<COMP7_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP7_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP7EN` reader - Comparator 7 enable"]
pub struct COMP7EN_R(crate::FieldReader<bool, bool>);
impl COMP7EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP7EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7EN` writer - Comparator 7 enable"]
pub struct COMP7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7EN_W<'a> {
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
#[doc = "Field `COMP7MODE` reader - Comparator 7 mode"]
pub struct COMP7MODE_R(crate::FieldReader<u8, u8>);
impl COMP7MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP7MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7MODE` writer - Comparator 7 mode"]
pub struct COMP7MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP7INMSEL` reader - Comparator 7 inverting input selection"]
pub struct COMP7INMSEL_R(crate::FieldReader<u8, u8>);
impl COMP7INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP7INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7INMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7INMSEL` writer - Comparator 7 inverting input selection"]
pub struct COMP7INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP7INPSEL` reader - Comparator 7 non inverted input"]
pub struct COMP7INPSEL_R(crate::FieldReader<bool, bool>);
impl COMP7INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP7INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7INPSEL` writer - Comparator 7 non inverted input"]
pub struct COMP7INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7INPSEL_W<'a> {
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
#[doc = "Field `COMP7OUTSEL` reader - Comparator 7 output selection"]
pub struct COMP7OUTSEL_R(crate::FieldReader<u8, u8>);
impl COMP7OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP7OUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7OUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7OUTSEL` writer - Comparator 7 output selection"]
pub struct COMP7OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `COMP7POL` reader - Comparator 7 output polarity"]
pub struct COMP7POL_R(crate::FieldReader<bool, bool>);
impl COMP7POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP7POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7POL` writer - Comparator 7 output polarity"]
pub struct COMP7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `COMP7HYST` reader - Comparator 7 hysteresis"]
pub struct COMP7HYST_R(crate::FieldReader<u8, u8>);
impl COMP7HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP7HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7HYST` writer - Comparator 7 hysteresis"]
pub struct COMP7HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP7_BLANKING` reader - Comparator 7 blanking source"]
pub struct COMP7_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP7_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP7_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7_BLANKING` writer - Comparator 7 blanking source"]
pub struct COMP7_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP7OUT` reader - Comparator 7 output"]
pub struct COMP7OUT_R(crate::FieldReader<bool, bool>);
impl COMP7OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP7OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7LOCK` reader - Comparator 7 lock"]
pub struct COMP7LOCK_R(crate::FieldReader<bool, bool>);
impl COMP7LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP7LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP7LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP7LOCK` writer - Comparator 7 lock"]
pub struct COMP7LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7LOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&self) -> COMP7EN_R {
        COMP7EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&self) -> COMP7MODE_R {
        COMP7MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7inmsel(&self) -> COMP7INMSEL_R {
        COMP7INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input"]
    #[inline(always)]
    pub fn comp7inpsel(&self) -> COMP7INPSEL_R {
        COMP7INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7outsel(&self) -> COMP7OUTSEL_R {
        COMP7OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&self) -> COMP7POL_R {
        COMP7POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&self) -> COMP7HYST_R {
        COMP7HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&self) -> COMP7_BLANKING_R {
        COMP7_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 7 output"]
    #[inline(always)]
    pub fn comp7out(&self) -> COMP7OUT_R {
        COMP7OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&self) -> COMP7LOCK_R {
        COMP7LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&mut self) -> COMP7EN_W {
        COMP7EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&mut self) -> COMP7MODE_W {
        COMP7MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7inmsel(&mut self) -> COMP7INMSEL_W {
        COMP7INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input"]
    #[inline(always)]
    pub fn comp7inpsel(&mut self) -> COMP7INPSEL_W {
        COMP7INPSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7outsel(&mut self) -> COMP7OUTSEL_W {
        COMP7OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&mut self) -> COMP7POL_W {
        COMP7POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&mut self) -> COMP7HYST_W {
        COMP7HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&mut self) -> COMP7_BLANKING_W {
        COMP7_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&mut self) -> COMP7LOCK_W {
        COMP7LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp7_csr](index.html) module"]
pub struct COMP7_CSR_SPEC;
impl crate::RegisterSpec for COMP7_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp7_csr::R](R) reader structure"]
impl crate::Readable for COMP7_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp7_csr::W](W) writer structure"]
impl crate::Writable for COMP7_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP7_CSR to value 0"]
impl crate::Resettable for COMP7_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
