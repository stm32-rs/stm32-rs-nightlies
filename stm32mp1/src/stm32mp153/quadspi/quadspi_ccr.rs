#[doc = "Register `QUADSPI_CCR` reader"]
pub struct R(crate::R<QUADSPI_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_CCR` writer"]
pub struct W(crate::W<QUADSPI_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_CCR_SPEC>;
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
impl From<crate::W<QUADSPI_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub struct INSTRUCTION_R(crate::FieldReader<u8, u8>);
impl INSTRUCTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSTRUCTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTRUCTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `IMODE` reader - IMODE"]
pub struct IMODE_R(crate::FieldReader<u8, u8>);
impl IMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMODE` writer - IMODE"]
pub struct IMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ADMODE` reader - ADMODE"]
pub struct ADMODE_R(crate::FieldReader<u8, u8>);
impl ADMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMODE` writer - ADMODE"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `ADSIZE` reader - ADSIZE"]
pub struct ADSIZE_R(crate::FieldReader<u8, u8>);
impl ADSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSIZE` writer - ADSIZE"]
pub struct ADSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ABMODE` reader - ABMODE"]
pub struct ABMODE_R(crate::FieldReader<u8, u8>);
impl ABMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ABMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABMODE` writer - ABMODE"]
pub struct ABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `ABSIZE` reader - ABSIZE"]
pub struct ABSIZE_R(crate::FieldReader<u8, u8>);
impl ABSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ABSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABSIZE` writer - ABSIZE"]
pub struct ABSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DCYC` reader - DCYC"]
pub struct DCYC_R(crate::FieldReader<u8, u8>);
impl DCYC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCYC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCYC` writer - DCYC"]
pub struct DCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `DMODE` reader - DMODE"]
pub struct DMODE_R(crate::FieldReader<u8, u8>);
impl DMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMODE` writer - DMODE"]
pub struct DMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `FMODE` reader - FMODE"]
pub struct FMODE_R(crate::FieldReader<u8, u8>);
impl FMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMODE` writer - FMODE"]
pub struct FMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `SIOO` reader - SIOO"]
pub struct SIOO_R(crate::FieldReader<bool, bool>);
impl SIOO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIOO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIOO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIOO` writer - SIOO"]
pub struct SIOO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIOO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `FRCM` reader - FRCM"]
pub struct FRCM_R(crate::FieldReader<bool, bool>);
impl FRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRCM` writer - FRCM"]
pub struct FRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DHHC` reader - DHHC"]
pub struct DHHC_R(crate::FieldReader<bool, bool>);
impl DHHC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHHC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHHC` writer - DHHC"]
pub struct DHHC_W<'a> {
    w: &'a mut W,
}
impl<'a> DHHC_W<'a> {
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
#[doc = "Field `DDRM` reader - DDRM"]
pub struct DDRM_R(crate::FieldReader<bool, bool>);
impl DDRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRM` writer - DDRM"]
pub struct DDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRM_W<'a> {
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
    #[doc = "Bits 0:7 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - IMODE"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - DMODE"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FMODE"]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - SIOO"]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - FRCM"]
    #[inline(always)]
    pub fn frcm(&self) -> FRCM_R {
        FRCM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DHHC"]
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DDRM"]
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
    #[doc = "Bits 8:9 - IMODE"]
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W {
        IMODE_W { w: self }
    }
    #[doc = "Bits 10:11 - ADMODE"]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W {
        ADSIZE_W { w: self }
    }
    #[doc = "Bits 14:15 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W {
        ABMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W {
        ABSIZE_W { w: self }
    }
    #[doc = "Bits 18:22 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W {
        DCYC_W { w: self }
    }
    #[doc = "Bits 24:25 - DMODE"]
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W {
        DMODE_W { w: self }
    }
    #[doc = "Bits 26:27 - FMODE"]
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W {
        FMODE_W { w: self }
    }
    #[doc = "Bit 28 - SIOO"]
    #[inline(always)]
    pub fn sioo(&mut self) -> SIOO_W {
        SIOO_W { w: self }
    }
    #[doc = "Bit 29 - FRCM"]
    #[inline(always)]
    pub fn frcm(&mut self) -> FRCM_W {
        FRCM_W { w: self }
    }
    #[doc = "Bit 30 - DHHC"]
    #[inline(always)]
    pub fn dhhc(&mut self) -> DHHC_W {
        DHHC_W { w: self }
    }
    #[doc = "Bit 31 - DDRM"]
    #[inline(always)]
    pub fn ddrm(&mut self) -> DDRM_W {
        DDRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_ccr](index.html) module"]
pub struct QUADSPI_CCR_SPEC;
impl crate::RegisterSpec for QUADSPI_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_ccr::R](R) reader structure"]
impl crate::Readable for QUADSPI_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_ccr::W](W) writer structure"]
impl crate::Writable for QUADSPI_CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUADSPI_CCR to value 0"]
impl crate::Resettable for QUADSPI_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
