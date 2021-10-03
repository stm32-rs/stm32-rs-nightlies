#[doc = "Register `OPTSR_PRG_` reader"]
pub struct R(crate::R<OPTSR_PRG__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR_PRG__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR_PRG__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR_PRG__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTSR_PRG_` writer"]
pub struct W(crate::W<OPTSR_PRG__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTSR_PRG__SPEC>;
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
impl From<crate::W<OPTSR_PRG__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTSR_PRG__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOR_LEV` reader - BOR reset level option configuration bits"]
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
#[doc = "Field `BOR_LEV` writer - BOR reset level option configuration bits"]
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
#[doc = "Field `IWDG1_HW` reader - IWDG1 option configuration bit"]
pub struct IWDG1_HW_R(crate::FieldReader<bool, bool>);
impl IWDG1_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG1_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG1_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG1_HW` writer - IWDG1 option configuration bit"]
pub struct IWDG1_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1_HW_W<'a> {
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
#[doc = "Field `nRST_STOP_D1` reader - Option byte erase after D1 DStop option configuration bit"]
pub struct NRST_STOP_D1_R(crate::FieldReader<bool, bool>);
impl NRST_STOP_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STOP_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STOP_D1` writer - Option byte erase after D1 DStop option configuration bit"]
pub struct NRST_STOP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_D1_W<'a> {
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
#[doc = "Field `nRST_STBY_D1` reader - Option byte erase after D1 DStandby option configuration bit"]
pub struct NRST_STBY_D1_R(crate::FieldReader<bool, bool>);
impl NRST_STBY_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STBY_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STBY_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STBY_D1` writer - Option byte erase after D1 DStandby option configuration bit"]
pub struct NRST_STBY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STBY_D1_W<'a> {
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
#[doc = "Field `RDP` reader - Readout protection level option configuration byte"]
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
#[doc = "Field `RDP` writer - Readout protection level option configuration byte"]
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
#[doc = "Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option configuration bit"]
pub struct FZ_IWDG_STOP_R(crate::FieldReader<bool, bool>);
impl FZ_IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZ_IWDG_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZ_IWDG_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option configuration bit"]
pub struct FZ_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG_STOP_W<'a> {
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
#[doc = "Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option configuration bit"]
pub struct FZ_IWDG_SDBY_R(crate::FieldReader<bool, bool>);
impl FZ_IWDG_SDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZ_IWDG_SDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZ_IWDG_SDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option configuration bit"]
pub struct FZ_IWDG_SDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG_SDBY_W<'a> {
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
#[doc = "Field `ST_RAM_SIZE` reader - DTCM size select option configuration bits"]
pub struct ST_RAM_SIZE_R(crate::FieldReader<u8, u8>);
impl ST_RAM_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ST_RAM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_RAM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST_RAM_SIZE` writer - DTCM size select option configuration bits"]
pub struct ST_RAM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_RAM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `SECURITY` reader - Security option configuration bit"]
pub struct SECURITY_R(crate::FieldReader<bool, bool>);
impl SECURITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURITY` writer - Security option configuration bit"]
pub struct SECURITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RSS1` reader - User option configuration bit 1"]
pub struct RSS1_R(crate::FieldReader<bool, bool>);
impl RSS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSS1` writer - User option configuration bit 1"]
pub struct RSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSS1_W<'a> {
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
#[doc = "Field `RSS2` reader - User option configuration bit 2"]
pub struct RSS2_R(crate::FieldReader<bool, bool>);
impl RSS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSS2` writer - User option configuration bit 2"]
pub struct RSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `IO_HSLV` reader - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
pub struct IO_HSLV_R(crate::FieldReader<bool, bool>);
impl IO_HSLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IO_HSLV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_HSLV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_HSLV` writer - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
pub struct IO_HSLV_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_HSLV_W<'a> {
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
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option configuration bit"]
pub struct SWAP_BANK_OPT_R(crate::FieldReader<bool, bool>);
impl SWAP_BANK_OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWAP_BANK_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP_BANK_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option configuration bit"]
pub struct SWAP_BANK_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_BANK_OPT_W<'a> {
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
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&self) -> IWDG1_HW_R {
        IWDG1_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> NRST_STBY_D1_R {
        NRST_STBY_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> RSS1_R {
        RSS1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    pub fn rss2(&self) -> RSS2_R {
        RSS2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&mut self) -> IWDG1_HW_W {
        IWDG1_HW_W { w: self }
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&mut self) -> NRST_STOP_D1_W {
        NRST_STOP_D1_W { w: self }
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&mut self) -> NRST_STBY_D1_W {
        NRST_STBY_D1_W { w: self }
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W {
        FZ_IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W {
        FZ_IWDG_SDBY_W { w: self }
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W {
        ST_RAM_SIZE_W { w: self }
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W {
        SECURITY_W { w: self }
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    pub fn rss1(&mut self) -> RSS1_W {
        RSS1_W { w: self }
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    pub fn rss2(&mut self) -> RSS2_W {
        RSS2_W { w: self }
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W {
        IO_HSLV_W { w: self }
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W {
        SWAP_BANK_OPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_prg_](index.html) module"]
pub struct OPTSR_PRG__SPEC;
impl crate::RegisterSpec for OPTSR_PRG__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optsr_prg_::R](R) reader structure"]
impl crate::Readable for OPTSR_PRG__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optsr_prg_::W](W) writer structure"]
impl crate::Writable for OPTSR_PRG__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTSR_PRG_ to value 0"]
impl crate::Resettable for OPTSR_PRG__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
