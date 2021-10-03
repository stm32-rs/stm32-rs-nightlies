#[doc = "Register `OPTCR` reader"]
pub struct R(crate::R<OPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR` writer"]
pub struct W(crate::W<OPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR_SPEC>;
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
impl From<crate::W<OPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTLOCK` reader - Option lock"]
pub struct OPTLOCK_R(crate::FieldReader<bool, bool>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTLOCK` writer - Option lock"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
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
#[doc = "Field `OPTSTRT` reader - Option start"]
pub struct OPTSTRT_R(crate::FieldReader<bool, bool>);
impl OPTSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTSTRT` writer - Option start"]
pub struct OPTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTRT_W<'a> {
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
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub struct BOR_LEV_R(crate::FieldReader<u8, u8>);
impl BOR_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOR_LEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOR_LEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub struct BOR_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_LEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `WWDG_SW` reader - User option bytes"]
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
#[doc = "Field `WWDG_SW` writer - User option bytes"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IWDG_SW` reader - User option bytes"]
pub struct IWDG_SW_R(crate::FieldReader<bool, bool>);
impl IWDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_SW` writer - User option bytes"]
pub struct IWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_SW_W<'a> {
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
#[doc = "Field `nRST_STOP` reader - User option bytes"]
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
#[doc = "Field `nRST_STOP` writer - User option bytes"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `nRST_STDBY` reader - User option bytes"]
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
#[doc = "Field `nRST_STDBY` writer - User option bytes"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RDP` reader - Read protect"]
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
#[doc = "Field `RDP` writer - Read protect"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `nWRP` reader - Not write protect"]
pub struct NWRP_R(crate::FieldReader<u16, u16>);
impl NWRP_R {
    pub(crate) fn new(bits: u16) -> Self {
        NWRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NWRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nWRP` writer - Not write protect"]
pub struct NWRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NWRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `nDBOOT` reader - Dual Boot mode (valid only when nDBANK=0)"]
pub struct NDBOOT_R(crate::FieldReader<bool, bool>);
impl NDBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NDBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nDBOOT` writer - Dual Boot mode (valid only when nDBANK=0)"]
pub struct NDBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDBOOT_W<'a> {
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
#[doc = "Field `nDBANK` reader - Not dual bank mode"]
pub struct NDBANK_R(crate::FieldReader<bool, bool>);
impl NDBANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NDBANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDBANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nDBANK` writer - Not dual bank mode"]
pub struct NDBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> NDBANK_W<'a> {
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
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in standby mode"]
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
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in standby mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - User option bytes"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - User option bytes"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> NWRP_R {
        NWRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Dual Boot mode (valid only when nDBANK=0)"]
    #[inline(always)]
    pub fn n_dboot(&self) -> NDBOOT_R {
        NDBOOT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Not dual bank mode"]
    #[inline(always)]
    pub fn n_dbank(&self) -> NDBANK_R {
        NDBANK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Independent watchdog counter freeze in standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W {
        OPTSTRT_W { w: self }
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    #[doc = "Bit 4 - User option bytes"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W {
        WWDG_SW_W { w: self }
    }
    #[doc = "Bit 5 - User option bytes"]
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W {
        IWDG_SW_W { w: self }
    }
    #[doc = "Bit 6 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRST_STOP_W {
        NRST_STOP_W { w: self }
    }
    #[doc = "Bit 7 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W {
        NRST_STDBY_W { w: self }
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&mut self) -> NWRP_W {
        NWRP_W { w: self }
    }
    #[doc = "Bit 28 - Dual Boot mode (valid only when nDBANK=0)"]
    #[inline(always)]
    pub fn n_dboot(&mut self) -> NDBOOT_W {
        NDBOOT_W { w: self }
    }
    #[doc = "Bit 29 - Not dual bank mode"]
    #[inline(always)]
    pub fn n_dbank(&mut self) -> NDBANK_W {
        NDBANK_W { w: self }
    }
    #[doc = "Bit 30 - Independent watchdog counter freeze in standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W {
        IWDG_STDBY_W { w: self }
    }
    #[doc = "Bit 31 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W {
        IWDG_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr](index.html) module"]
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr::R](R) reader structure"]
impl crate::Readable for OPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr::W](W) writer structure"]
impl crate::Writable for OPTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR to value 0x0fff_aaed"]
impl crate::Resettable for OPTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_aaed
    }
}
