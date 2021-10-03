#[doc = "Register `COMP_C4CSR` reader"]
pub struct R(crate::R<COMP_C4CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_C4CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_C4CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_C4CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_C4CSR` writer"]
pub struct W(crate::W<COMP_C4CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_C4CSR_SPEC>;
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
impl From<crate::W<COMP_C4CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_C4CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `COMP_DEGLITCH_EN` reader - COMP_DEGLITCH_EN"]
pub struct COMP_DEGLITCH_EN_R(crate::FieldReader<bool, bool>);
impl COMP_DEGLITCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP_DEGLITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_DEGLITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_DEGLITCH_EN` writer - COMP_DEGLITCH_EN"]
pub struct COMP_DEGLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_DEGLITCH_EN_W<'a> {
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
#[doc = "Field `INMSEL` reader - INMSEL"]
pub struct INMSEL_R(crate::FieldReader<u8, u8>);
impl INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INMSEL` writer - INMSEL"]
pub struct INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `INPSEL` reader - INPSEL"]
pub struct INPSEL_R(crate::FieldReader<bool, bool>);
impl INPSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPSEL` writer - INPSEL"]
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
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
#[doc = "Field `POL` reader - POL"]
pub struct POL_R(crate::FieldReader<bool, bool>);
impl POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL` writer - POL"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
#[doc = "Field `HYST` reader - HYST"]
pub struct HYST_R(crate::FieldReader<u8, u8>);
impl HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - HYST"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `BLANKSEL` reader - BLANKSEL"]
pub struct BLANKSEL_R(crate::FieldReader<u8, u8>);
impl BLANKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLANKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLANKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANKSEL` writer - BLANKSEL"]
pub struct BLANKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `BRGEN` reader - BRGEN"]
pub struct BRGEN_R(crate::FieldReader<bool, bool>);
impl BRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRGEN` writer - BRGEN"]
pub struct BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGEN_W<'a> {
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
#[doc = "Field `SCALEN` reader - SCALEN"]
pub struct SCALEN_R(crate::FieldReader<bool, bool>);
impl SCALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALEN` writer - SCALEN"]
pub struct SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `VALUE` reader - VALUE"]
pub struct VALUE_R(crate::FieldReader<bool, bool>);
impl VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - LOCK"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP_DEGLITCH_EN"]
    #[inline(always)]
    pub fn comp_deglitch_en(&self) -> COMP_DEGLITCH_EN_R {
        COMP_DEGLITCH_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 30 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - COMP_DEGLITCH_EN"]
    #[inline(always)]
    pub fn comp_deglitch_en(&mut self) -> COMP_DEGLITCH_EN_W {
        COMP_DEGLITCH_EN_W { w: self }
    }
    #[doc = "Bits 4:6 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W {
        INMSEL_W { w: self }
    }
    #[doc = "Bit 8 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    #[doc = "Bit 15 - POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bits 16:18 - HYST"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 19:21 - BLANKSEL"]
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W {
        BLANKSEL_W { w: self }
    }
    #[doc = "Bit 22 - BRGEN"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W {
        BRGEN_W { w: self }
    }
    #[doc = "Bit 23 - SCALEN"]
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W {
        SCALEN_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_c4csr](index.html) module"]
pub struct COMP_C4CSR_SPEC;
impl crate::RegisterSpec for COMP_C4CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_c4csr::R](R) reader structure"]
impl crate::Readable for COMP_C4CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_c4csr::W](W) writer structure"]
impl crate::Writable for COMP_C4CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_C4CSR to value 0"]
impl crate::Resettable for COMP_C4CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
