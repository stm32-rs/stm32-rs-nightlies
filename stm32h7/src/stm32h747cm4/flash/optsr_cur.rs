#[doc = "Register `OPTSR_CUR` reader"]
pub struct R(crate::R<OPTSR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTSR_CUR` writer"]
pub struct W(crate::W<OPTSR_CUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTSR_CUR_SPEC>;
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
impl From<crate::W<OPTSR_CUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTSR_CUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option status bit"]
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
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option status bit"]
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
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag"]
pub struct OPTCHANGEERR_R(crate::FieldReader<bool, bool>);
impl OPTCHANGEERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTCHANGEERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTCHANGEERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTCHANGEERR` writer - Option byte change error flag"]
pub struct OPTCHANGEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTCHANGEERR_W<'a> {
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
#[doc = "Field `IO_HSLV` reader - I"]
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
#[doc = "Field `IO_HSLV` writer - I"]
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
#[doc = "Field `NRST_STBY_D2` reader - D2 domain DStandby entry reset option status bit"]
pub struct NRST_STBY_D2_R(crate::FieldReader<bool, bool>);
impl NRST_STBY_D2_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STBY_D2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STBY_D2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRST_STBY_D2` writer - D2 domain DStandby entry reset option status bit"]
pub struct NRST_STBY_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STBY_D2_W<'a> {
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
#[doc = "Field `NRST_STOP_D2` reader - D2 domain DStop entry reset option status bit"]
pub struct NRST_STOP_D2_R(crate::FieldReader<bool, bool>);
impl NRST_STOP_D2_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_D2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STOP_D2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRST_STOP_D2` writer - D2 domain DStop entry reset option status bit"]
pub struct NRST_STOP_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_D2_W<'a> {
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
#[doc = "Field `BOOT_CM7` reader - Arm Cortex"]
pub struct BOOT_CM7_R(crate::FieldReader<bool, bool>);
impl BOOT_CM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_CM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_CM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_CM7` writer - Arm Cortex"]
pub struct BOOT_CM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `BOOT_CM4` reader - Arm Cortex"]
pub struct BOOT_CM4_R(crate::FieldReader<bool, bool>);
impl BOOT_CM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_CM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_CM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_CM4` writer - Arm Cortex"]
pub struct BOOT_CM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM4_W<'a> {
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
#[doc = "Field `SECURITY` reader - Security enable option status bit"]
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
#[doc = "Field `SECURITY` writer - Security enable option status bit"]
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
#[doc = "Field `ST_RAM_SIZE` reader - ST RAM size option status"]
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
#[doc = "Field `ST_RAM_SIZE` writer - ST RAM size option status"]
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
#[doc = "Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option status bit"]
pub struct IWDG_FZ_SDBY_R(crate::FieldReader<bool, bool>);
impl IWDG_FZ_SDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_FZ_SDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_FZ_SDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option status bit"]
pub struct IWDG_FZ_SDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_FZ_SDBY_W<'a> {
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
#[doc = "Field `IWDG_FZ_STOP` reader - IWDG Stop mode freeze option status bit"]
pub struct IWDG_FZ_STOP_R(crate::FieldReader<bool, bool>);
impl IWDG_FZ_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_FZ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_FZ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_FZ_STOP` writer - IWDG Stop mode freeze option status bit"]
pub struct IWDG_FZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_FZ_STOP_W<'a> {
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
#[doc = "Field `RDP` reader - Readout protection level option status byte"]
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
#[doc = "Field `RDP` writer - Readout protection level option status byte"]
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
#[doc = "Field `RST_STDY_D1` reader - D1 domain DStandby entry reset option status bit"]
pub struct RST_STDY_D1_R(crate::FieldReader<bool, bool>);
impl RST_STDY_D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_STDY_D1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_STDY_D1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_STDY_D1` writer - D1 domain DStandby entry reset option status bit"]
pub struct RST_STDY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_STDY_D1_W<'a> {
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
#[doc = "Field `NRST_STOP_D1` reader - D1 domain DStop entry reset option status bit"]
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
#[doc = "Field `NRST_STOP_D1` writer - D1 domain DStop entry reset option status bit"]
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
#[doc = "Field `IWDG2_SW` reader - IWDG2 control mode option status bit"]
pub struct IWDG2_SW_R(crate::FieldReader<bool, bool>);
impl IWDG2_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG2_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG2_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG2_SW` writer - IWDG2 control mode option status bit"]
pub struct IWDG2_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2_SW_W<'a> {
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
#[doc = "Field `IWDG_SW` reader - IWDG control mode option status bit"]
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
#[doc = "Field `IWDG_SW` writer - IWDG control mode option status bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit"]
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
#[doc = "Field `BOR_LEV` writer - Brownout level option status bit"]
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
#[doc = "Field `OPT_BUSY` reader - Option byte change ongoing flag"]
pub struct OPT_BUSY_R(crate::FieldReader<bool, bool>);
impl OPT_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPT_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPT_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPT_BUSY` writer - Option byte change ongoing flag"]
pub struct OPT_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> OPT_BUSY_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&self) -> NRST_STBY_D2_R {
        NRST_STBY_D2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&self) -> NRST_STOP_D2_R {
        NRST_STOP_D2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&self) -> BOOT_CM7_R {
        BOOT_CM7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&self) -> BOOT_CM4_R {
        BOOT_CM4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn rst_stdy_d1(&self) -> RST_STDY_D1_R {
        RST_STDY_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&self) -> IWDG2_SW_R {
        IWDG2_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&self) -> OPT_BUSY_R {
        OPT_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W {
        SWAP_BANK_OPT_W { w: self }
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W {
        OPTCHANGEERR_W { w: self }
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W {
        IO_HSLV_W { w: self }
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&mut self) -> NRST_STBY_D2_W {
        NRST_STBY_D2_W { w: self }
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&mut self) -> NRST_STOP_D2_W {
        NRST_STOP_D2_W { w: self }
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&mut self) -> BOOT_CM7_W {
        BOOT_CM7_W { w: self }
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&mut self) -> BOOT_CM4_W {
        BOOT_CM4_W { w: self }
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W {
        SECURITY_W { w: self }
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W {
        ST_RAM_SIZE_W { w: self }
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W {
        IWDG_FZ_SDBY_W { w: self }
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W {
        IWDG_FZ_STOP_W { w: self }
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn rst_stdy_d1(&mut self) -> RST_STDY_D1_W {
        RST_STDY_D1_W { w: self }
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&mut self) -> NRST_STOP_D1_W {
        NRST_STOP_D1_W { w: self }
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&mut self) -> IWDG2_SW_W {
        IWDG2_SW_W { w: self }
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W {
        IWDG_SW_W { w: self }
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W {
        OPT_BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_cur](index.html) module"]
pub struct OPTSR_CUR_SPEC;
impl crate::RegisterSpec for OPTSR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optsr_cur::R](R) reader structure"]
impl crate::Readable for OPTSR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optsr_cur::W](W) writer structure"]
impl crate::Writable for OPTSR_CUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTSR_CUR to value 0"]
impl crate::Resettable for OPTSR_CUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
