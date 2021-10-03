#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTR` writer"]
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDP` reader - Read protection level"]
pub struct RDP_R(crate::FieldReader<u8, u8>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDP` writer - Read protection level"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BOREN` reader - BOR reset Level"]
pub struct BOREN_R(crate::FieldReader<bool, bool>);
impl BOREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOREN` writer - BOR reset Level"]
pub struct BOREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOREN_W<'a> {
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
#[doc = "Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset"]
pub struct BORF_LEV_R(crate::FieldReader<u8, u8>);
impl BORF_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BORF_LEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORF_LEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset"]
pub struct BORF_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BORF_LEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `BORR_LEV` reader - These bits contain the VDD supply level threshold that releases the reset."]
pub struct BORR_LEV_R(crate::FieldReader<u8, u8>);
impl BORR_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BORR_LEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORR_LEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORR_LEV` writer - These bits contain the VDD supply level threshold that releases the reset."]
pub struct BORR_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BORR_LEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub struct NRST_STOP_R(crate::FieldReader<bool, bool>);
impl NRST_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub struct NRST_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_W<'a> {
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
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub struct NRST_STDBY_R(crate::FieldReader<bool, bool>);
impl NRST_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub struct NRST_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STDBY_W<'a> {
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
#[doc = "Field `nRSTS_HDW` reader - nRSTS_HDW"]
pub struct NRSTS_HDW_R(crate::FieldReader<bool, bool>);
impl NRSTS_HDW_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRSTS_HDW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRSTS_HDW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRSTS_HDW` writer - nRSTS_HDW"]
pub struct NRSTS_HDW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRSTS_HDW_W<'a> {
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
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub struct IDWG_SW_R(crate::FieldReader<bool, bool>);
impl IDWG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDWG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDWG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub struct IDWG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IDWG_SW_W<'a> {
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
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub struct IWDG_STOP_R(crate::FieldReader<bool, bool>);
impl IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub struct IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STOP_W<'a> {
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
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub struct IWDG_STDBY_R(crate::FieldReader<bool, bool>);
impl IWDG_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_STDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_STDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub struct IWDG_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STDBY_W<'a> {
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
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub struct WWDG_SW_R(crate::FieldReader<bool, bool>);
impl WWDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub struct WWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG_SW_W<'a> {
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
#[doc = "Field `RAM_PARITY_CHECK` reader - SRAM parity check control"]
pub struct RAM_PARITY_CHECK_R(crate::FieldReader<bool, bool>);
impl RAM_PARITY_CHECK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_PARITY_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_PARITY_CHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_PARITY_CHECK` writer - SRAM parity check control"]
pub struct RAM_PARITY_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_PARITY_CHECK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `nBOOT_SEL` reader - nBOOT_SEL"]
pub struct NBOOT_SEL_R(crate::FieldReader<bool, bool>);
impl NBOOT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBOOT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nBOOT_SEL` writer - nBOOT_SEL"]
pub struct NBOOT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT_SEL_W<'a> {
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
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub struct NBOOT1_R(crate::FieldReader<bool, bool>);
impl NBOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBOOT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub struct NBOOT1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT1_W<'a> {
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
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub struct NBOOT0_R(crate::FieldReader<bool, bool>);
impl NBOOT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBOOT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub struct NBOOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `NRST_MODE` reader - NRST_MODE"]
pub struct NRST_MODE_R(crate::FieldReader<u8, u8>);
impl NRST_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        NRST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRST_MODE` writer - NRST_MODE"]
pub struct NRST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `IRHEN` reader - Internal reset holder enable bit"]
pub struct IRHEN_R(crate::FieldReader<bool, bool>);
impl IRHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRHEN` writer - Internal reset holder enable bit"]
pub struct IRHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRHEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&self) -> BOREN_R {
        BOREN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset."]
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - nRSTS_HDW"]
    #[inline(always)]
    pub fn n_rsts_hdw(&self) -> NRSTS_HDW_R {
        NRSTS_HDW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn n_boot_sel(&self) -> NBOOT_SEL_R {
        NBOOT_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&mut self) -> BOREN_W {
        BOREN_W { w: self }
    }
    #[doc = "Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&mut self) -> BORF_LEV_W {
        BORF_LEV_W { w: self }
    }
    #[doc = "Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset."]
    #[inline(always)]
    pub fn borr_lev(&mut self) -> BORR_LEV_W {
        BORR_LEV_W { w: self }
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRST_STOP_W {
        NRST_STOP_W { w: self }
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W {
        NRST_STDBY_W { w: self }
    }
    #[doc = "Bit 15 - nRSTS_HDW"]
    #[inline(always)]
    pub fn n_rsts_hdw(&mut self) -> NRSTS_HDW_W {
        NRSTS_HDW_W { w: self }
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W {
        IDWG_SW_W { w: self }
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W {
        IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W {
        IWDG_STDBY_W { w: self }
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W {
        WWDG_SW_W { w: self }
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W {
        RAM_PARITY_CHECK_W { w: self }
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn n_boot_sel(&mut self) -> NBOOT_SEL_W {
        NBOOT_SEL_W { w: self }
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBOOT1_W {
        NBOOT1_W { w: self }
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBOOT0_W {
        NBOOT0_W { w: self }
    }
    #[doc = "Bits 27:28 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W {
        NRST_MODE_W { w: self }
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W {
        IRHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optr::W](W) writer structure"]
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
