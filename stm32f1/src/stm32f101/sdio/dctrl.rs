#[doc = "Register `DCTRL` reader"]
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTRL` writer"]
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub struct DTEN_R(crate::FieldReader<bool, bool>);
impl DTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN` writer - DTEN"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
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
#[doc = "Field `DTDIR` reader - DTDIR"]
pub struct DTDIR_R(crate::FieldReader<bool, bool>);
impl DTDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTDIR` writer - DTDIR"]
pub struct DTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDIR_W<'a> {
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
#[doc = "Field `DTMODE` reader - DTMODE"]
pub struct DTMODE_R(crate::FieldReader<bool, bool>);
impl DTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTMODE` writer - DTMODE"]
pub struct DTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTMODE_W<'a> {
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
#[doc = "Field `DMAEN` reader - DMAEN"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub struct DBLOCKSIZE_R(crate::FieldReader<u8, u8>);
impl DBLOCKSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBLOCKSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBLOCKSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub struct DBLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLOCKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PWSTART` reader - PWSTART"]
pub struct PWSTART_R(crate::FieldReader<bool, bool>);
impl PWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWSTART` writer - PWSTART"]
pub struct PWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> PWSTART_W<'a> {
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
#[doc = "Field `PWSTOP` reader - PWSTOP"]
pub struct PWSTOP_R(crate::FieldReader<bool, bool>);
impl PWSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWSTOP` writer - PWSTOP"]
pub struct PWSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWSTOP_W<'a> {
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
#[doc = "Field `RWMOD` reader - RWMOD"]
pub struct RWMOD_R(crate::FieldReader<bool, bool>);
impl RWMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWMOD` writer - RWMOD"]
pub struct RWMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RWMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub struct SDIOEN_R(crate::FieldReader<bool, bool>);
impl SDIOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub struct SDIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&self) -> PWSTART_R {
        PWSTART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&self) -> PWSTOP_R {
        PWSTOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W {
        DTDIR_W { w: self }
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W {
        DTMODE_W { w: self }
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W {
        DBLOCKSIZE_W { w: self }
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&mut self) -> PWSTART_W {
        PWSTART_W { w: self }
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&mut self) -> PWSTOP_W {
        PWSTOP_W { w: self }
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W {
        RWMOD_W { w: self }
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](index.html) module"]
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctrl::R](R) reader structure"]
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctrl::W](W) writer structure"]
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
