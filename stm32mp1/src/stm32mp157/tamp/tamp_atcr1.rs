#[doc = "Register `TAMP_ATCR1` reader"]
pub struct R(crate::R<TAMP_ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_ATCR1` writer"]
pub struct W(crate::W<TAMP_ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ATCR1_SPEC>;
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
impl From<crate::W<TAMP_ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1AM` reader - TAMP1AM"]
pub struct TAMP1AM_R(crate::FieldReader<bool, bool>);
impl TAMP1AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1AM` writer - TAMP1AM"]
pub struct TAMP1AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1AM_W<'a> {
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
#[doc = "Field `TAMP2AM` reader - TAMP2AM"]
pub struct TAMP2AM_R(crate::FieldReader<bool, bool>);
impl TAMP2AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2AM` writer - TAMP2AM"]
pub struct TAMP2AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2AM_W<'a> {
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
#[doc = "Field `TAMP3AM` reader - TAMP3AM"]
pub struct TAMP3AM_R(crate::FieldReader<bool, bool>);
impl TAMP3AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3AM` writer - TAMP3AM"]
pub struct TAMP3AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3AM_W<'a> {
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
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub struct ATOSEL1_R(crate::FieldReader<u8, u8>);
impl ATOSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub struct ATOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub struct ATOSEL2_R(crate::FieldReader<u8, u8>);
impl ATOSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub struct ATOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub struct ATOSEL3_R(crate::FieldReader<u8, u8>);
impl ATOSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub struct ATOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ATCKSEL` reader - ATCKSEL"]
pub struct ATCKSEL_R(crate::FieldReader<u8, u8>);
impl ATCKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATCKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATCKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATCKSEL` writer - ATCKSEL"]
pub struct ATCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `ATPER` reader - ATPER"]
pub struct ATPER_R(crate::FieldReader<u8, u8>);
impl ATPER_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATPER` writer - ATPER"]
pub struct ATPER_W<'a> {
    w: &'a mut W,
}
impl<'a> ATPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `ATOSHARE` reader - ATOSHARE"]
pub struct ATOSHARE_R(crate::FieldReader<bool, bool>);
impl ATOSHARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATOSHARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSHARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSHARE` writer - ATOSHARE"]
pub struct ATOSHARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSHARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `FLTEN` reader - FLTEN"]
pub struct FLTEN_R(crate::FieldReader<bool, bool>);
impl FLTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTEN` writer - FLTEN"]
pub struct FLTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTEN_W<'a> {
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
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - ATPER"]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&mut self) -> TAMP1AM_W {
        TAMP1AM_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&mut self) -> TAMP2AM_W {
        TAMP2AM_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&mut self) -> TAMP3AM_W {
        TAMP3AM_W { w: self }
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W {
        ATOSEL1_W { w: self }
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W {
        ATOSEL2_W { w: self }
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W {
        ATOSEL3_W { w: self }
    }
    #[doc = "Bits 16:18 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&mut self) -> ATCKSEL_W {
        ATCKSEL_W { w: self }
    }
    #[doc = "Bits 24:26 - ATPER"]
    #[inline(always)]
    pub fn atper(&mut self) -> ATPER_W {
        ATPER_W { w: self }
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&mut self) -> ATOSHARE_W {
        ATOSHARE_W { w: self }
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W {
        FLTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atcr1](index.html) module"]
pub struct TAMP_ATCR1_SPEC;
impl crate::RegisterSpec for TAMP_ATCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_atcr1::R](R) reader structure"]
impl crate::Readable for TAMP_ATCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_atcr1::W](W) writer structure"]
impl crate::Writable for TAMP_ATCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_ATCR1 to value 0x0007_0000"]
impl crate::Resettable for TAMP_ATCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0000
    }
}
