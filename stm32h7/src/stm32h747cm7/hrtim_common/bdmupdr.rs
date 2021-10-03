#[doc = "Register `BDMUPDR` reader"]
pub struct R(crate::R<BDMUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDMUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDMUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDMUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDMUPDR` writer"]
pub struct W(crate::W<BDMUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMUPDR_SPEC>;
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
impl From<crate::W<BDMUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCMP4` reader - MCMP4"]
pub struct MCMP4_R(crate::FieldReader<bool, bool>);
impl MCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP4` writer - MCMP4"]
pub struct MCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `MCMP3` reader - MCMP3"]
pub struct MCMP3_R(crate::FieldReader<bool, bool>);
impl MCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP3` writer - MCMP3"]
pub struct MCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3_W<'a> {
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
#[doc = "Field `MCMP2` reader - MCMP2"]
pub struct MCMP2_R(crate::FieldReader<bool, bool>);
impl MCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP2` writer - MCMP2"]
pub struct MCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2_W<'a> {
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
#[doc = "Field `MCMP1` reader - MCMP1"]
pub struct MCMP1_R(crate::FieldReader<bool, bool>);
impl MCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP1` writer - MCMP1"]
pub struct MCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1_W<'a> {
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
#[doc = "Field `MREP` reader - MREP"]
pub struct MREP_R(crate::FieldReader<bool, bool>);
impl MREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREP` writer - MREP"]
pub struct MREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MREP_W<'a> {
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
#[doc = "Field `MPER` reader - MPER"]
pub struct MPER_R(crate::FieldReader<bool, bool>);
impl MPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPER` writer - MPER"]
pub struct MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MPER_W<'a> {
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
#[doc = "Field `MCNT` reader - MCNT"]
pub struct MCNT_R(crate::FieldReader<bool, bool>);
impl MCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCNT` writer - MCNT"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
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
#[doc = "Field `MDIER` reader - MDIER"]
pub struct MDIER_R(crate::FieldReader<bool, bool>);
impl MDIER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDIER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIER` writer - MDIER"]
pub struct MDIER_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIER_W<'a> {
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
#[doc = "Field `MICR` reader - MICR"]
pub struct MICR_R(crate::FieldReader<bool, bool>);
impl MICR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICR` writer - MICR"]
pub struct MICR_W<'a> {
    w: &'a mut W,
}
impl<'a> MICR_W<'a> {
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
#[doc = "Field `MCR` reader - MCR"]
pub struct MCR_R(crate::FieldReader<bool, bool>);
impl MCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCR` writer - MCR"]
pub struct MCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_W<'a> {
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
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&mut self) -> MCMP4_W {
        MCMP4_W { w: self }
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W {
        MCMP3_W { w: self }
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W {
        MCMP2_W { w: self }
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W {
        MCMP1_W { w: self }
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W {
        MREP_W { w: self }
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W {
        MPER_W { w: self }
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&mut self) -> MDIER_W {
        MDIER_W { w: self }
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&mut self) -> MICR_W {
        MICR_W { w: self }
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W {
        MCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BDMUPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmupdr](index.html) module"]
pub struct BDMUPDR_SPEC;
impl crate::RegisterSpec for BDMUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdmupdr::R](R) reader structure"]
impl crate::Readable for BDMUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdmupdr::W](W) writer structure"]
impl crate::Writable for BDMUPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDMUPDR to value 0"]
impl crate::Resettable for BDMUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
