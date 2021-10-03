#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDL` reader - Programmable voltage detector lockup bit"]
pub struct PVDL_R(crate::FieldReader<bool, bool>);
impl PVDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDL` writer - Programmable voltage detector lockup bit"]
pub struct PVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDL_W<'a> {
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
#[doc = "Field `FLASHL` reader - FLASH double error lockup bit"]
pub struct FLASHL_R(crate::FieldReader<bool, bool>);
impl FLASHL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHL` writer - FLASH double error lockup bit"]
pub struct FLASHL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHL_W<'a> {
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
#[doc = "Field `CM7L` reader - CPU lockup bit"]
pub struct CM7L_R(crate::FieldReader<bool, bool>);
impl CM7L_R {
    pub(crate) fn new(bits: bool) -> Self {
        CM7L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM7L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM7L` writer - CPU lockup bit"]
pub struct CM7L_W<'a> {
    w: &'a mut W,
}
impl<'a> CM7L_W<'a> {
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
#[doc = "Field `BKRAML` reader - Backup RAM Double error lockup bit"]
pub struct BKRAML_R(crate::FieldReader<bool, bool>);
impl BKRAML_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKRAML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKRAML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKRAML` writer - Backup RAM Double error lockup bit"]
pub struct BKRAML_W<'a> {
    w: &'a mut W,
}
impl<'a> BKRAML_W<'a> {
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
#[doc = "Field `SRAM4L` reader - SRAM4 Double error lockup bit"]
pub struct SRAM4L_R(crate::FieldReader<bool, bool>);
impl SRAM4L_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM4L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM4L` writer - SRAM4 Double error lockup bit"]
pub struct SRAM4L_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4L_W<'a> {
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
#[doc = "Field `SRAM2L` reader - SRAM2 Double error lockup bit"]
pub struct SRAM2L_R(crate::FieldReader<bool, bool>);
impl SRAM2L_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2L` writer - SRAM2 Double error lockup bit"]
pub struct SRAM2L_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SRAM1L` reader - SRAM1 Double error lockup bit"]
pub struct SRAM1L_R(crate::FieldReader<bool, bool>);
impl SRAM1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1L` writer - SRAM1 Double error lockup bit"]
pub struct SRAM1L_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DTCML` reader - DTCM-RAM Double error lockup bit"]
pub struct DTCML_R(crate::FieldReader<bool, bool>);
impl DTCML_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCML` writer - DTCM-RAM Double error lockup bit"]
pub struct DTCML_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCML_W<'a> {
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
#[doc = "Field `ITCML` reader - ITCM-RAM Double error lockup bit"]
pub struct ITCML_R(crate::FieldReader<bool, bool>);
impl ITCML_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITCML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITCML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITCML` writer - ITCM-RAM Double error lockup bit"]
pub struct ITCML_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCML_W<'a> {
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
#[doc = "Field `AXIRAML` reader - AXISRAM Double error lockup bit"]
pub struct AXIRAML_R(crate::FieldReader<bool, bool>);
impl AXIRAML_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIRAML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAML` writer - AXISRAM Double error lockup bit"]
pub struct AXIRAML_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAML_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    pub fn axiraml(&self) -> AXIRAML_R {
        AXIRAML_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W {
        PVDL_W { w: self }
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    pub fn flashl(&mut self) -> FLASHL_W {
        FLASHL_W { w: self }
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    pub fn cm7l(&mut self) -> CM7L_W {
        CM7L_W { w: self }
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    pub fn bkraml(&mut self) -> BKRAML_W {
        BKRAML_W { w: self }
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    pub fn sram4l(&mut self) -> SRAM4L_W {
        SRAM4L_W { w: self }
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    pub fn sram2l(&mut self) -> SRAM2L_W {
        SRAM2L_W { w: self }
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    pub fn sram1l(&mut self) -> SRAM1L_W {
        SRAM1L_W { w: self }
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn dtcml(&mut self) -> DTCML_W {
        DTCML_W { w: self }
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn itcml(&mut self) -> ITCML_W {
        ITCML_W { w: self }
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    pub fn axiraml(&mut self) -> AXIRAML_W {
        AXIRAML_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer break lockup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
