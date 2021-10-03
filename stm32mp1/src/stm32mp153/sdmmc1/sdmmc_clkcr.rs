#[doc = "Register `SDMMC_CLKCR` reader"]
pub struct R(crate::R<SDMMC_CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_CLKCR` writer"]
pub struct W(crate::W<SDMMC_CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_CLKCR_SPEC>;
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
impl From<crate::W<SDMMC_CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub struct CLKDIV_R(crate::FieldReader<u16, u16>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PWRSAV` reader - PWRSAV"]
pub struct PWRSAV_R(crate::FieldReader<bool, bool>);
impl PWRSAV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRSAV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSAV` writer - PWRSAV"]
pub struct PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSAV_W<'a> {
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
#[doc = "Field `WIDBUS` reader - WIDBUS"]
pub struct WIDBUS_R(crate::FieldReader<u8, u8>);
impl WIDBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIDBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIDBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDBUS` writer - WIDBUS"]
pub struct WIDBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `NEGEDGE` reader - NEGEDGE"]
pub struct NEGEDGE_R(crate::FieldReader<bool, bool>);
impl NEGEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEGEDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEGEDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEGEDGE` writer - NEGEDGE"]
pub struct NEGEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGEDGE_W<'a> {
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
#[doc = "Field `HWFC_EN` reader - HWFC_EN"]
pub struct HWFC_EN_R(crate::FieldReader<bool, bool>);
impl HWFC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWFC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWFC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWFC_EN` writer - HWFC_EN"]
pub struct HWFC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWFC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DDR` reader - DDR"]
pub struct DDR_R(crate::FieldReader<bool, bool>);
impl DDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR` writer - DDR"]
pub struct DDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_W<'a> {
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
#[doc = "Field `BUSSPEED` reader - BUSSPEED"]
pub struct BUSSPEED_R(crate::FieldReader<bool, bool>);
impl BUSSPEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSSPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSSPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSSPEED` writer - BUSSPEED"]
pub struct BUSSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSSPEED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SELCLKRX` reader - SELCLKRX"]
pub struct SELCLKRX_R(crate::FieldReader<u8, u8>);
impl SELCLKRX_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELCLKRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELCLKRX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELCLKRX` writer - SELCLKRX"]
pub struct SELCLKRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCLKRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - WIDBUS"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - NEGEDGE"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HWFC_EN"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DDR"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BUSSPEED"]
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - SELCLKRX"]
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 12 - PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W {
        PWRSAV_W { w: self }
    }
    #[doc = "Bits 14:15 - WIDBUS"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W {
        WIDBUS_W { w: self }
    }
    #[doc = "Bit 16 - NEGEDGE"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W {
        NEGEDGE_W { w: self }
    }
    #[doc = "Bit 17 - HWFC_EN"]
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W {
        HWFC_EN_W { w: self }
    }
    #[doc = "Bit 18 - DDR"]
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W {
        DDR_W { w: self }
    }
    #[doc = "Bit 19 - BUSSPEED"]
    #[inline(always)]
    pub fn busspeed(&mut self) -> BUSSPEED_W {
        BUSSPEED_W { w: self }
    }
    #[doc = "Bits 20:21 - SELCLKRX"]
    #[inline(always)]
    pub fn selclkrx(&mut self) -> SELCLKRX_W {
        SELCLKRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and the bus width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_clkcr](index.html) module"]
pub struct SDMMC_CLKCR_SPEC;
impl crate::RegisterSpec for SDMMC_CLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_clkcr::R](R) reader structure"]
impl crate::Readable for SDMMC_CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_clkcr::W](W) writer structure"]
impl crate::Writable for SDMMC_CLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_CLKCR to value 0"]
impl crate::Resettable for SDMMC_CLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
