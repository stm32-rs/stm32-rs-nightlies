#[doc = "Register `SDMMC_CMDR` reader"]
pub struct R(crate::R<SDMMC_CMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_CMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_CMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_CMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_CMDR` writer"]
pub struct W(crate::W<SDMMC_CMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_CMDR_SPEC>;
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
impl From<crate::W<SDMMC_CMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_CMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - CMDINDEX"]
pub struct CMDINDEX_R(crate::FieldReader<u8, u8>);
impl CMDINDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDINDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDINDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDINDEX` writer - CMDINDEX"]
pub struct CMDINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CMDTRANS` reader - CMDTRANS"]
pub struct CMDTRANS_R(crate::FieldReader<bool, bool>);
impl CMDTRANS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDTRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDTRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDTRANS` writer - CMDTRANS"]
pub struct CMDTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTRANS_W<'a> {
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
#[doc = "Field `CMDSTOP` reader - CMDSTOP"]
pub struct CMDSTOP_R(crate::FieldReader<bool, bool>);
impl CMDSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSTOP` writer - CMDSTOP"]
pub struct CMDSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSTOP_W<'a> {
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
#[doc = "Field `WAITRESP` reader - WAITRESP"]
pub struct WAITRESP_R(crate::FieldReader<u8, u8>);
impl WAITRESP_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITRESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITRESP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITRESP` writer - WAITRESP"]
pub struct WAITRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITRESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `WAITINT` reader - WAITINT"]
pub struct WAITINT_R(crate::FieldReader<bool, bool>);
impl WAITINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAITINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITINT` writer - WAITINT"]
pub struct WAITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITINT_W<'a> {
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
#[doc = "Field `WAITPEND` reader - WAITPEND"]
pub struct WAITPEND_R(crate::FieldReader<bool, bool>);
impl WAITPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAITPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITPEND` writer - WAITPEND"]
pub struct WAITPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPEND_W<'a> {
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
#[doc = "Field `CPSMEN` reader - CPSMEN"]
pub struct CPSMEN_R(crate::FieldReader<bool, bool>);
impl CPSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPSMEN` writer - CPSMEN"]
pub struct CPSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSMEN_W<'a> {
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
#[doc = "Field `DTHOLD` reader - DTHOLD"]
pub struct DTHOLD_R(crate::FieldReader<bool, bool>);
impl DTHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTHOLD` writer - DTHOLD"]
pub struct DTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHOLD_W<'a> {
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
#[doc = "Field `BOOTMODE` reader - BOOTMODE"]
pub struct BOOTMODE_R(crate::FieldReader<bool, bool>);
impl BOOTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTMODE` writer - BOOTMODE"]
pub struct BOOTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTMODE_W<'a> {
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
#[doc = "Field `BOOTEN` reader - BOOTEN"]
pub struct BOOTEN_R(crate::FieldReader<bool, bool>);
impl BOOTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTEN` writer - BOOTEN"]
pub struct BOOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTEN_W<'a> {
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
#[doc = "Field `CMDSUSPEND` reader - CMDSUSPEND"]
pub struct CMDSUSPEND_R(crate::FieldReader<bool, bool>);
impl CMDSUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSUSPEND` writer - CMDSUSPEND"]
pub struct CMDSUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSUSPEND_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W {
        CMDINDEX_W { w: self }
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W {
        CMDTRANS_W { w: self }
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&mut self) -> CMDSTOP_W {
        CMDSTOP_W { w: self }
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W {
        WAITRESP_W { w: self }
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W {
        WAITINT_W { w: self }
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W {
        WAITPEND_W { w: self }
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W {
        CPSMEN_W { w: self }
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&mut self) -> DTHOLD_W {
        DTHOLD_W { w: self }
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W {
        BOOTMODE_W { w: self }
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W {
        BOOTEN_W { w: self }
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W {
        CMDSUSPEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_cmdr](index.html) module"]
pub struct SDMMC_CMDR_SPEC;
impl crate::RegisterSpec for SDMMC_CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_cmdr::R](R) reader structure"]
impl crate::Readable for SDMMC_CMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_cmdr::W](W) writer structure"]
impl crate::Writable for SDMMC_CMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_CMDR to value 0"]
impl crate::Resettable for SDMMC_CMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
