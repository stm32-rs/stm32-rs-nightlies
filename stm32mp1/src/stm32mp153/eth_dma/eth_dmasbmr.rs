#[doc = "Register `ETH_DMASBMR` reader"]
pub struct R(crate::R<ETH_DMASBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMASBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMASBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMASBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMASBMR` writer"]
pub struct W(crate::W<ETH_DMASBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMASBMR_SPEC>;
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
impl From<crate::W<ETH_DMASBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMASBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub struct FB_R(crate::FieldReader<bool, bool>);
impl FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
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
#[doc = "Field `BLEN4` reader - BLEN4"]
pub struct BLEN4_R(crate::FieldReader<bool, bool>);
impl BLEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN4` writer - BLEN4"]
pub struct BLEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN4_W<'a> {
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
#[doc = "Field `BLEN8` reader - BLEN8"]
pub struct BLEN8_R(crate::FieldReader<bool, bool>);
impl BLEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN8` writer - BLEN8"]
pub struct BLEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN8_W<'a> {
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
#[doc = "Field `BLEN16` reader - BLEN16"]
pub struct BLEN16_R(crate::FieldReader<bool, bool>);
impl BLEN16_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN16` writer - BLEN16"]
pub struct BLEN16_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN16_W<'a> {
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
#[doc = "Field `BLEN32` reader - BLEN32"]
pub struct BLEN32_R(crate::FieldReader<bool, bool>);
impl BLEN32_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN32` writer - BLEN32"]
pub struct BLEN32_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN32_W<'a> {
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
#[doc = "Field `BLEN64` reader - BLEN64"]
pub struct BLEN64_R(crate::FieldReader<bool, bool>);
impl BLEN64_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN64_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN64` writer - BLEN64"]
pub struct BLEN64_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN64_W<'a> {
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
#[doc = "Field `BLEN128` reader - BLEN128"]
pub struct BLEN128_R(crate::FieldReader<bool, bool>);
impl BLEN128_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN128` writer - BLEN128"]
pub struct BLEN128_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN128_W<'a> {
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
#[doc = "Field `BLEN256` reader - BLEN256"]
pub struct BLEN256_R(crate::FieldReader<bool, bool>);
impl BLEN256_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEN256_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN256_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN256` writer - BLEN256"]
pub struct BLEN256_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN256_W<'a> {
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
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub struct AAL_R(crate::FieldReader<bool, bool>);
impl AAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub struct AAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AAL_W<'a> {
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
#[doc = "Field `ONEKBBE` reader - ONEKBBE"]
pub struct ONEKBBE_R(crate::FieldReader<bool, bool>);
impl ONEKBBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEKBBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONEKBBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEKBBE` writer - ONEKBBE"]
pub struct ONEKBBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEKBBE_W<'a> {
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
#[doc = "Field `RD_OSR_LMT` reader - RD_OSR_LMT"]
pub struct RD_OSR_LMT_R(crate::FieldReader<u8, u8>);
impl RD_OSR_LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_OSR_LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_OSR_LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_OSR_LMT` writer - RD_OSR_LMT"]
pub struct RD_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `WR_OSR_LMT` reader - WR_OSR_LMT"]
pub struct WR_OSR_LMT_R(crate::FieldReader<u8, u8>);
impl WR_OSR_LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WR_OSR_LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_OSR_LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_OSR_LMT` writer - WR_OSR_LMT"]
pub struct WR_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `LPI_XIT_PKT` reader - LPI_XIT_PKT"]
pub struct LPI_XIT_PKT_R(crate::FieldReader<bool, bool>);
impl LPI_XIT_PKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPI_XIT_PKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPI_XIT_PKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPI_XIT_PKT` writer - LPI_XIT_PKT"]
pub struct LPI_XIT_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI_XIT_PKT_W<'a> {
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
#[doc = "Field `EN_LPI` reader - EN_LPI"]
pub struct EN_LPI_R(crate::FieldReader<bool, bool>);
impl EN_LPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_LPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_LPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_LPI` writer - EN_LPI"]
pub struct EN_LPI_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_LPI_W<'a> {
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
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    pub fn lpi_xit_pkt(&self) -> LPI_XIT_PKT_R {
        LPI_XIT_PKT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
    #[doc = "Bit 1 - BLEN4"]
    #[inline(always)]
    pub fn blen4(&mut self) -> BLEN4_W {
        BLEN4_W { w: self }
    }
    #[doc = "Bit 2 - BLEN8"]
    #[inline(always)]
    pub fn blen8(&mut self) -> BLEN8_W {
        BLEN8_W { w: self }
    }
    #[doc = "Bit 3 - BLEN16"]
    #[inline(always)]
    pub fn blen16(&mut self) -> BLEN16_W {
        BLEN16_W { w: self }
    }
    #[doc = "Bit 4 - BLEN32"]
    #[inline(always)]
    pub fn blen32(&mut self) -> BLEN32_W {
        BLEN32_W { w: self }
    }
    #[doc = "Bit 5 - BLEN64"]
    #[inline(always)]
    pub fn blen64(&mut self) -> BLEN64_W {
        BLEN64_W { w: self }
    }
    #[doc = "Bit 6 - BLEN128"]
    #[inline(always)]
    pub fn blen128(&mut self) -> BLEN128_W {
        BLEN128_W { w: self }
    }
    #[doc = "Bit 7 - BLEN256"]
    #[inline(always)]
    pub fn blen256(&mut self) -> BLEN256_W {
        BLEN256_W { w: self }
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W {
        AAL_W { w: self }
    }
    #[doc = "Bit 13 - ONEKBBE"]
    #[inline(always)]
    pub fn onekbbe(&mut self) -> ONEKBBE_W {
        ONEKBBE_W { w: self }
    }
    #[doc = "Bits 16:17 - RD_OSR_LMT"]
    #[inline(always)]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W {
        RD_OSR_LMT_W { w: self }
    }
    #[doc = "Bits 24:25 - WR_OSR_LMT"]
    #[inline(always)]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W {
        WR_OSR_LMT_W { w: self }
    }
    #[doc = "Bit 30 - LPI_XIT_PKT"]
    #[inline(always)]
    pub fn lpi_xit_pkt(&mut self) -> LPI_XIT_PKT_W {
        LPI_XIT_PKT_W { w: self }
    }
    #[doc = "Bit 31 - EN_LPI"]
    #[inline(always)]
    pub fn en_lpi(&mut self) -> EN_LPI_W {
        EN_LPI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmasbmr](index.html) module"]
pub struct ETH_DMASBMR_SPEC;
impl crate::RegisterSpec for ETH_DMASBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmasbmr::R](R) reader structure"]
impl crate::Readable for ETH_DMASBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmasbmr::W](W) writer structure"]
impl crate::Writable for ETH_DMASBMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMASBMR to value 0x8000"]
impl crate::Resettable for ETH_DMASBMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
