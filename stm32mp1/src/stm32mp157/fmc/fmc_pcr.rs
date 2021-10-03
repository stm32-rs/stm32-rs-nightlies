#[doc = "Register `FMC_PCR` reader"]
pub struct R(crate::R<FMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PCR` writer"]
pub struct W(crate::W<FMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PCR_SPEC>;
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
impl From<crate::W<FMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub struct PWAITEN_R(crate::FieldReader<bool, bool>);
impl PWAITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWAITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWAITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub struct PWAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAITEN_W<'a> {
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
#[doc = "Field `PBKEN` reader - PBKEN"]
pub struct PBKEN_R(crate::FieldReader<bool, bool>);
impl PBKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBKEN` writer - PBKEN"]
pub struct PBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBKEN_W<'a> {
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
#[doc = "Field `PWID` reader - PWID"]
pub struct PWID_R(crate::FieldReader<u8, u8>);
impl PWID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWID` writer - PWID"]
pub struct PWID_W<'a> {
    w: &'a mut W,
}
impl<'a> PWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ECCEN` reader - ECCEN"]
pub struct ECCEN_R(crate::FieldReader<bool, bool>);
impl ECCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCEN` writer - ECCEN"]
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
#[doc = "Field `ECCALG` reader - ECCALG"]
pub struct ECCALG_R(crate::FieldReader<bool, bool>);
impl ECCALG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCALG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCALG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCALG` writer - ECCALG"]
pub struct ECCALG_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCALG_W<'a> {
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
#[doc = "Field `TCLR` reader - TCLR"]
pub struct TCLR_R(crate::FieldReader<u8, u8>);
impl TCLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLR` writer - TCLR"]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `TAR` reader - TAR"]
pub struct TAR_R(crate::FieldReader<u8, u8>);
impl TAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAR` writer - TAR"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `ECCSS` reader - ECCSS"]
pub struct ECCSS_R(crate::FieldReader<u8, u8>);
impl ECCSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSS` writer - ECCSS"]
pub struct ECCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `TCEH` reader - TCEH"]
pub struct TCEH_R(crate::FieldReader<u8, u8>);
impl TCEH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCEH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCEH` writer - TCEH"]
pub struct TCEH_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `BCHECC` reader - BCHECC"]
pub struct BCHECC_R(crate::FieldReader<bool, bool>);
impl BCHECC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCHECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCHECC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCHECC` writer - BCHECC"]
pub struct BCHECC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCHECC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `WEN` reader - WEN"]
pub struct WEN_R(crate::FieldReader<bool, bool>);
impl WEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEN` writer - WEN"]
pub struct WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    pub fn eccalg(&self) -> ECCALG_R {
        ECCALG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    pub fn eccss(&self) -> ECCSS_R {
        ECCSS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    pub fn tceh(&self) -> TCEH_R {
        TCEH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    pub fn bchecc(&self) -> BCHECC_R {
        BCHECC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W { w: self }
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W { w: self }
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W { w: self }
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    pub fn eccalg(&mut self) -> ECCALG_W {
        ECCALG_W { w: self }
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    pub fn eccss(&mut self) -> ECCSS_W {
        ECCSS_W { w: self }
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    pub fn tceh(&mut self) -> TCEH_W {
        TCEH_W { w: self }
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    pub fn bchecc(&mut self) -> BCHECC_W {
        BCHECC_W { w: self }
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W {
        WEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Flash Programmable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](index.html) module"]
pub struct FMC_PCR_SPEC;
impl crate::RegisterSpec for FMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pcr::R](R) reader structure"]
impl crate::Readable for FMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](W) writer structure"]
impl crate::Writable for FMC_PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_PCR to value 0x0007_fe08"]
impl crate::Resettable for FMC_PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_fe08
    }
}
