#[doc = "Register `DCR1` reader"]
pub struct R(crate::R<DCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR1` writer"]
pub struct W(crate::W<DCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR1_SPEC>;
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
impl From<crate::W<DCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub struct CKMODE_R(crate::FieldReader<bool, bool>);
impl CKMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
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
#[doc = "Field `FRCK` reader - Free running clock"]
pub struct FRCK_R(crate::FieldReader<bool, bool>);
impl FRCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRCK` writer - Free running clock"]
pub struct FRCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCK_W<'a> {
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
#[doc = "Field `CSHT` reader - Chip-select high time"]
pub struct CSHT_R(crate::FieldReader<u8, u8>);
impl CSHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSHT` writer - Chip-select high time"]
pub struct CSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DEVSIZE` reader - Device size"]
pub struct DEVSIZE_R(crate::FieldReader<u8, u8>);
impl DEVSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVSIZE` writer - Device size"]
pub struct DEVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `MTYP` reader - Memory type"]
pub struct MTYP_R(crate::FieldReader<u8, u8>);
impl MTYP_R {
    pub(crate) fn new(bits: u8) -> Self {
        MTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTYP` writer - Memory type"]
pub struct MTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> MTYP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `DLYBYP` reader - Delay block bypass"]
pub struct DLYBYP_R(crate::FieldReader<bool, bool>);
impl DLYBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBYP` writer - Delay block bypass"]
pub struct DLYBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBYP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    pub fn frck(&mut self) -> FRCK_W {
        FRCK_W { w: self }
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W {
        CSHT_W { w: self }
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    pub fn devsize(&mut self) -> DEVSIZE_W {
        DEVSIZE_W { w: self }
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W {
        MTYP_W { w: self }
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&mut self) -> DLYBYP_W {
        DLYBYP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr1](index.html) module"]
pub struct DCR1_SPEC;
impl crate::RegisterSpec for DCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr1::R](R) reader structure"]
impl crate::Readable for DCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr1::W](W) writer structure"]
impl crate::Writable for DCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for DCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
