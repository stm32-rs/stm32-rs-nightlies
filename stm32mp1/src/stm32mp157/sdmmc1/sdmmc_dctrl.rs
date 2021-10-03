#[doc = "Register `SDMMC_DCTRL` reader"]
pub struct R(crate::R<SDMMC_DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_DCTRL` writer"]
pub struct W(crate::W<SDMMC_DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_DCTRL_SPEC>;
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
impl From<crate::W<SDMMC_DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_DCTRL_SPEC>) -> Self {
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
pub struct DTMODE_R(crate::FieldReader<u8, u8>);
impl DTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
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
#[doc = "Field `RWSTART` reader - RWSTART"]
pub struct RWSTART_R(crate::FieldReader<bool, bool>);
impl RWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWSTART` writer - RWSTART"]
pub struct RWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RWSTART_W<'a> {
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
#[doc = "Field `RWSTOP` reader - RWSTOP"]
pub struct RWSTOP_R(crate::FieldReader<bool, bool>);
impl RWSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWSTOP` writer - RWSTOP"]
pub struct RWSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWSTOP_W<'a> {
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
#[doc = "Field `BOOTACKEN` reader - BOOTACKEN"]
pub struct BOOTACKEN_R(crate::FieldReader<bool, bool>);
impl BOOTACKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOTACKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTACKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTACKEN` writer - BOOTACKEN"]
pub struct BOOTACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACKEN_W<'a> {
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
#[doc = "Field `FIFORST` reader - FIFORST"]
pub struct FIFORST_R(crate::FieldReader<bool, bool>);
impl FIFORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFORST` writer - FIFORST"]
pub struct FIFORST_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORST_W<'a> {
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
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 0x01) != 0)
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
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 0x01) != 0)
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
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W {
        DTMODE_W { w: self }
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W {
        DBLOCKSIZE_W { w: self }
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W {
        RWSTART_W { w: self }
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W {
        RWSTOP_W { w: self }
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
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    pub fn bootacken(&mut self) -> BOOTACKEN_W {
        BOOTACKEN_W { w: self }
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    pub fn fiforst(&mut self) -> FIFORST_W {
        FIFORST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dctrl](index.html) module"]
pub struct SDMMC_DCTRL_SPEC;
impl crate::RegisterSpec for SDMMC_DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dctrl::R](R) reader structure"]
impl crate::Readable for SDMMC_DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_dctrl::W](W) writer structure"]
impl crate::Writable for SDMMC_DCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_DCTRL to value 0"]
impl crate::Resettable for SDMMC_DCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
