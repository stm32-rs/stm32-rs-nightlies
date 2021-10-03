#[doc = "Register `MDMA_C4TCR` reader"]
pub struct R(crate::R<MDMA_C4TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C4TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C4TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C4TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C4TCR` writer"]
pub struct W(crate::W<MDMA_C4TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C4TCR_SPEC>;
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
impl From<crate::W<MDMA_C4TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C4TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINC` reader - SINC"]
pub struct SINC_R(crate::FieldReader<u8, u8>);
impl SINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINC` writer - SINC"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DINC` reader - DINC"]
pub struct DINC_R(crate::FieldReader<u8, u8>);
impl DINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINC` writer - DINC"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SSIZE` reader - SSIZE"]
pub struct SSIZE_R(crate::FieldReader<u8, u8>);
impl SSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIZE` writer - SSIZE"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DSIZE` reader - DSIZE"]
pub struct DSIZE_R(crate::FieldReader<u8, u8>);
impl DSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIZE` writer - DSIZE"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `SINCOS` reader - SINCOS"]
pub struct SINCOS_R(crate::FieldReader<u8, u8>);
impl SINCOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINCOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINCOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINCOS` writer - SINCOS"]
pub struct SINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SINCOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DINCOS` reader - DINCOS"]
pub struct DINCOS_R(crate::FieldReader<u8, u8>);
impl DINCOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DINCOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DINCOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINCOS` writer - DINCOS"]
pub struct DINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DINCOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `SBURST` reader - SBURST"]
pub struct SBURST_R(crate::FieldReader<u8, u8>);
impl SBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBURST` writer - SBURST"]
pub struct SBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> SBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DBURST` reader - DBURST"]
pub struct DBURST_R(crate::FieldReader<u8, u8>);
impl DBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBURST` writer - DBURST"]
pub struct DBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `TLEN` reader - TLEN"]
pub struct TLEN_R(crate::FieldReader<u8, u8>);
impl TLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLEN` writer - TLEN"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 18)) | ((value as u32 & 0x7f) << 18);
        self.w
    }
}
#[doc = "Field `PKE` reader - PKE"]
pub struct PKE_R(crate::FieldReader<bool, bool>);
impl PKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKE` writer - PKE"]
pub struct PKE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PAM` reader - PAM"]
pub struct PAM_R(crate::FieldReader<u8, u8>);
impl PAM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAM` writer - PAM"]
pub struct PAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `TRGM` reader - TRGM"]
pub struct TRGM_R(crate::FieldReader<u8, u8>);
impl TRGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGM` writer - TRGM"]
pub struct TRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `SWRM` reader - SWRM"]
pub struct SWRM_R(crate::FieldReader<bool, bool>);
impl SWRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRM` writer - SWRM"]
pub struct SWRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRM_W<'a> {
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
#[doc = "Field `BWM` reader - BWM"]
pub struct BWM_R(crate::FieldReader<bool, bool>);
impl BWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWM` writer - BWM"]
pub struct BWM_W<'a> {
    w: &'a mut W,
}
impl<'a> BWM_W<'a> {
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
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W {
        SINCOS_W { w: self }
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&mut self) -> DINCOS_W {
        DINCOS_W { w: self }
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&mut self) -> SBURST_W {
        SBURST_W { w: self }
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&mut self) -> DBURST_W {
        DBURST_W { w: self }
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W {
        PKE_W { w: self }
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W { w: self }
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&mut self) -> TRGM_W {
        TRGM_W { w: self }
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&mut self) -> SWRM_W {
        SWRM_W { w: self }
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&mut self) -> BWM_W {
        BWM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c4tcr](index.html) module"]
pub struct MDMA_C4TCR_SPEC;
impl crate::RegisterSpec for MDMA_C4TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c4tcr::R](R) reader structure"]
impl crate::Readable for MDMA_C4TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c4tcr::W](W) writer structure"]
impl crate::Writable for MDMA_C4TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C4TCR to value 0"]
impl crate::Resettable for MDMA_C4TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
