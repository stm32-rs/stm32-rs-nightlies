#[doc = "Register `HASH_CR` reader"]
pub struct R(crate::R<HASH_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CR` writer"]
pub struct W(crate::W<HASH_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CR_SPEC>;
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
impl From<crate::W<HASH_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` writer - INIT"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
#[doc = "Field `DMAE` reader - DMAE"]
pub struct DMAE_R(crate::FieldReader<bool, bool>);
impl DMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAE` writer - DMAE"]
pub struct DMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAE_W<'a> {
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
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub struct DATATYPE_R(crate::FieldReader<u8, u8>);
impl DATATYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATATYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `MODE` reader - MODE"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - MODE"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ALGO0` reader - ALGO0"]
pub struct ALGO0_R(crate::FieldReader<bool, bool>);
impl ALGO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALGO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGO0` writer - ALGO0"]
pub struct ALGO0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGO0_W<'a> {
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
#[doc = "Field `NBW` reader - NBW"]
pub struct NBW_R(crate::FieldReader<u8, u8>);
impl NBW_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINNE` reader - DINNE"]
pub struct DINNE_R(crate::FieldReader<bool, bool>);
impl DINNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DINNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMAT` reader - MDMAT"]
pub struct MDMAT_R(crate::FieldReader<bool, bool>);
impl MDMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMAT` writer - MDMAT"]
pub struct MDMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DMAA` writer - DMAA"]
pub struct DMAA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `LKEY` reader - LKEY"]
pub struct LKEY_R(crate::FieldReader<bool, bool>);
impl LKEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LKEY` writer - LKEY"]
pub struct LKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> LKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ALGO1` reader - ALGO1"]
pub struct ALGO1_R(crate::FieldReader<bool, bool>);
impl ALGO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALGO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGO1` writer - ALGO1"]
pub struct ALGO1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    pub fn algo0(&self) -> ALGO0_R {
        ALGO0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - NBW"]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DINNE"]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    pub fn algo1(&self) -> ALGO1_R {
        ALGO1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W {
        DMAE_W { w: self }
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    pub fn algo0(&mut self) -> ALGO0_W {
        ALGO0_W { w: self }
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    pub fn mdmat(&mut self) -> MDMAT_W {
        MDMAT_W { w: self }
    }
    #[doc = "Bit 14 - DMAA"]
    #[inline(always)]
    pub fn dmaa(&mut self) -> DMAA_W {
        DMAA_W { w: self }
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W {
        LKEY_W { w: self }
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    pub fn algo1(&mut self) -> ALGO1_W {
        ALGO1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_cr](index.html) module"]
pub struct HASH_CR_SPEC;
impl crate::RegisterSpec for HASH_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_cr::R](R) reader structure"]
impl crate::Readable for HASH_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_cr::W](W) writer structure"]
impl crate::Writable for HASH_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_CR to value 0"]
impl crate::Resettable for HASH_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
