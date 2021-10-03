#[doc = "Register `COMP5_CSR` reader"]
pub struct R(crate::R<COMP5_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP5_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP5_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP5_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP5_CSR` writer"]
pub struct W(crate::W<COMP5_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP5_CSR_SPEC>;
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
impl From<crate::W<COMP5_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP5_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP5EN` reader - Comparator 5 enable"]
pub struct COMP5EN_R(crate::FieldReader<bool, bool>);
impl COMP5EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP5EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5EN` writer - Comparator 5 enable"]
pub struct COMP5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5EN_W<'a> {
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
#[doc = "Field `COMP5MODE` reader - Comparator 5 mode"]
pub struct COMP5MODE_R(crate::FieldReader<u8, u8>);
impl COMP5MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP5MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5MODE` writer - Comparator 5 mode"]
pub struct COMP5MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `COMP5INMSEL` reader - Comparator 5 inverting input selection"]
pub struct COMP5INMSEL_R(crate::FieldReader<u8, u8>);
impl COMP5INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP5INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5INMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5INMSEL` writer - Comparator 5 inverting input selection"]
pub struct COMP5INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP5INPSEL` reader - Comparator 5 non inverted input"]
pub struct COMP5INPSEL_R(crate::FieldReader<bool, bool>);
impl COMP5INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP5INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5INPSEL` writer - Comparator 5 non inverted input"]
pub struct COMP5INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5INPSEL_W<'a> {
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
#[doc = "Field `COMP5OUTSEL` reader - Comparator 5 output selection"]
pub struct COMP5OUTSEL_R(crate::FieldReader<u8, u8>);
impl COMP5OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP5OUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5OUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5OUTSEL` writer - Comparator 5 output selection"]
pub struct COMP5OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `COMP5POL` reader - Comparator 5 output polarity"]
pub struct COMP5POL_R(crate::FieldReader<bool, bool>);
impl COMP5POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP5POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5POL` writer - Comparator 5 output polarity"]
pub struct COMP5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5POL_W<'a> {
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
#[doc = "Field `COMP5HYST` reader - Comparator 5 hysteresis"]
pub struct COMP5HYST_R(crate::FieldReader<u8, u8>);
impl COMP5HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP5HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5HYST` writer - Comparator 5 hysteresis"]
pub struct COMP5HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `COMP5_BLANKING` reader - Comparator 5 blanking source"]
pub struct COMP5_BLANKING_R(crate::FieldReader<u8, u8>);
impl COMP5_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP5_BLANKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5_BLANKING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5_BLANKING` writer - Comparator 5 blanking source"]
pub struct COMP5_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `COMP5OUT` reader - Comparator 5 output"]
pub struct COMP5OUT_R(crate::FieldReader<bool, bool>);
impl COMP5OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP5OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5LOCK` reader - Comparator 5 lock"]
pub struct COMP5LOCK_R(crate::FieldReader<bool, bool>);
impl COMP5LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP5LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP5LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP5LOCK` writer - Comparator 5 lock"]
pub struct COMP5LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&self) -> COMP5EN_R {
        COMP5EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&self) -> COMP5MODE_R {
        COMP5MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5inmsel(&self) -> COMP5INMSEL_R {
        COMP5INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input"]
    #[inline(always)]
    pub fn comp5inpsel(&self) -> COMP5INPSEL_R {
        COMP5INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5outsel(&self) -> COMP5OUTSEL_R {
        COMP5OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&self) -> COMP5POL_R {
        COMP5POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&self) -> COMP5HYST_R {
        COMP5HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&self) -> COMP5_BLANKING_R {
        COMP5_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 5 output"]
    #[inline(always)]
    pub fn comp5out(&self) -> COMP5OUT_R {
        COMP5OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&self) -> COMP5LOCK_R {
        COMP5LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&mut self) -> COMP5EN_W {
        COMP5EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&mut self) -> COMP5MODE_W {
        COMP5MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5inmsel(&mut self) -> COMP5INMSEL_W {
        COMP5INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input"]
    #[inline(always)]
    pub fn comp5inpsel(&mut self) -> COMP5INPSEL_W {
        COMP5INPSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5outsel(&mut self) -> COMP5OUTSEL_W {
        COMP5OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&mut self) -> COMP5POL_W {
        COMP5POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&mut self) -> COMP5HYST_W {
        COMP5HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&mut self) -> COMP5_BLANKING_W {
        COMP5_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&mut self) -> COMP5LOCK_W {
        COMP5LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp5_csr](index.html) module"]
pub struct COMP5_CSR_SPEC;
impl crate::RegisterSpec for COMP5_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp5_csr::R](R) reader structure"]
impl crate::Readable for COMP5_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp5_csr::W](W) writer structure"]
impl crate::Writable for COMP5_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP5_CSR to value 0"]
impl crate::Resettable for COMP5_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
