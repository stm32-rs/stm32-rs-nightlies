#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDS` reader - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
pub struct LPDS_R(crate::FieldReader<bool, bool>);
impl LPDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDS` writer - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
pub struct LPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDS_W<'a> {
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
#[doc = "Field `PVDE` reader - Programmable voltage detector enable"]
pub struct PVDE_R(crate::FieldReader<bool, bool>);
impl PVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDE` writer - Programmable voltage detector enable"]
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
#[doc = "Field `PLS` reader - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
pub struct PLS_R(crate::FieldReader<u8, u8>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub struct DBP_R(crate::FieldReader<bool, bool>);
impl DBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub struct DBP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBP_W<'a> {
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
#[doc = "Field `FLPS` reader - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
pub struct FLPS_R(crate::FieldReader<bool, bool>);
impl FLPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLPS` writer - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
pub struct FLPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BOOSTE` reader - BOOSTE"]
pub struct BOOSTE_R(crate::FieldReader<bool, bool>);
impl BOOSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOSTE` writer - BOOSTE"]
pub struct BOOSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOSTE_W<'a> {
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
#[doc = "Field `AVD_READY` reader - AVD_READY"]
pub struct AVD_READY_R(crate::FieldReader<bool, bool>);
impl AVD_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVD_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVD_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVD_READY` writer - AVD_READY"]
pub struct AVD_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> AVD_READY_W<'a> {
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
#[doc = "Field `SVOS` reader - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub struct SVOS_R(crate::FieldReader<u8, u8>);
impl SVOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SVOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVOS` writer - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub struct SVOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `AVDEN` reader - Peripheral voltage monitor on VDDA enable"]
pub struct AVDEN_R(crate::FieldReader<bool, bool>);
impl AVDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVDEN` writer - Peripheral voltage monitor on VDDA enable"]
pub struct AVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDEN_W<'a> {
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
#[doc = "Field `ALS` reader - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
pub struct ALS_R(crate::FieldReader<u8, u8>);
impl ALS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALS` writer - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
pub struct ALS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `AXIRAM1SO` reader - AXIRAM1SO"]
pub struct AXIRAM1SO_R(crate::FieldReader<bool, bool>);
impl AXIRAM1SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM1SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIRAM1SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM1SO` writer - AXIRAM1SO"]
pub struct AXIRAM1SO_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM1SO_W<'a> {
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
#[doc = "Field `AXIRAM2SO` reader - AXIRAM2SO"]
pub struct AXIRAM2SO_R(crate::FieldReader<bool, bool>);
impl AXIRAM2SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM2SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIRAM2SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM2SO` writer - AXIRAM2SO"]
pub struct AXIRAM2SO_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM2SO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `AXIRAM3SO` reader - AXIRAM3SO"]
pub struct AXIRAM3SO_R(crate::FieldReader<bool, bool>);
impl AXIRAM3SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM3SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIRAM3SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM3SO` writer - AXIRAM3SO"]
pub struct AXIRAM3SO_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM3SO_W<'a> {
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
#[doc = "Field `AHBRAM1SO` reader - AHBRAM1SO"]
pub struct AHBRAM1SO_R(crate::FieldReader<bool, bool>);
impl AHBRAM1SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBRAM1SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBRAM1SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBRAM1SO` writer - AHBRAM1SO"]
pub struct AHBRAM1SO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBRAM1SO_W<'a> {
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
#[doc = "Field `AHBRAM2SO` reader - AHBRAM2SO"]
pub struct AHBRAM2SO_R(crate::FieldReader<bool, bool>);
impl AHBRAM2SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBRAM2SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBRAM2SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBRAM2SO` writer - AHBRAM2SO"]
pub struct AHBRAM2SO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBRAM2SO_W<'a> {
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
#[doc = "Field `ITCMSO` reader - ITCMSO"]
pub struct ITCMSO_R(crate::FieldReader<bool, bool>);
impl ITCMSO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITCMSO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITCMSO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITCMSO` writer - ITCMSO"]
pub struct ITCMSO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCMSO_W<'a> {
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
#[doc = "Field `GFXSO` reader - GFXSO"]
pub struct GFXSO_R(crate::FieldReader<bool, bool>);
impl GFXSO_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFXSO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFXSO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFXSO` writer - GFXSO"]
pub struct GFXSO_W<'a> {
    w: &'a mut W,
}
impl<'a> GFXSO_W<'a> {
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
#[doc = "Field `HSITFSO` reader - HSITFSO"]
pub struct HSITFSO_R(crate::FieldReader<bool, bool>);
impl HSITFSO_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSITFSO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSITFSO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSITFSO` writer - HSITFSO"]
pub struct HSITFSO_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITFSO_W<'a> {
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
#[doc = "Field `SRDRAMSO` reader - SRDRAMSO"]
pub struct SRDRAMSO_R(crate::FieldReader<bool, bool>);
impl SRDRAMSO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRDRAMSO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRDRAMSO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRDRAMSO` writer - SRDRAMSO"]
pub struct SRDRAMSO_W<'a> {
    w: &'a mut W,
}
impl<'a> SRDRAMSO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Programmable voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BOOSTE"]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AVD_READY"]
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Peripheral voltage monitor on VDDA enable"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - AXIRAM1SO"]
    #[inline(always)]
    pub fn axiram1so(&self) -> AXIRAM1SO_R {
        AXIRAM1SO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AXIRAM2SO"]
    #[inline(always)]
    pub fn axiram2so(&self) -> AXIRAM2SO_R {
        AXIRAM2SO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - AXIRAM3SO"]
    #[inline(always)]
    pub fn axiram3so(&self) -> AXIRAM3SO_R {
        AXIRAM3SO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - AHBRAM1SO"]
    #[inline(always)]
    pub fn ahbram1so(&self) -> AHBRAM1SO_R {
        AHBRAM1SO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AHBRAM2SO"]
    #[inline(always)]
    pub fn ahbram2so(&self) -> AHBRAM2SO_R {
        AHBRAM2SO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ITCMSO"]
    #[inline(always)]
    pub fn itcmso(&self) -> ITCMSO_R {
        ITCMSO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GFXSO"]
    #[inline(always)]
    pub fn gfxso(&self) -> GFXSO_R {
        GFXSO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HSITFSO"]
    #[inline(always)]
    pub fn hsitfso(&self) -> HSITFSO_R {
        HSITFSO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SRDRAMSO"]
    #[inline(always)]
    pub fn srdramso(&self) -> SRDRAMSO_R {
        SRDRAMSO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W { w: self }
    }
    #[doc = "Bit 4 - Programmable voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    #[doc = "Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    #[doc = "Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
    #[inline(always)]
    pub fn flps(&mut self) -> FLPS_W {
        FLPS_W { w: self }
    }
    #[doc = "Bit 12 - BOOSTE"]
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W {
        BOOSTE_W { w: self }
    }
    #[doc = "Bit 13 - AVD_READY"]
    #[inline(always)]
    pub fn avd_ready(&mut self) -> AVD_READY_W {
        AVD_READY_W { w: self }
    }
    #[doc = "Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn svos(&mut self) -> SVOS_W {
        SVOS_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral voltage monitor on VDDA enable"]
    #[inline(always)]
    pub fn avden(&mut self) -> AVDEN_W {
        AVDEN_W { w: self }
    }
    #[doc = "Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W {
        ALS_W { w: self }
    }
    #[doc = "Bit 19 - AXIRAM1SO"]
    #[inline(always)]
    pub fn axiram1so(&mut self) -> AXIRAM1SO_W {
        AXIRAM1SO_W { w: self }
    }
    #[doc = "Bit 20 - AXIRAM2SO"]
    #[inline(always)]
    pub fn axiram2so(&mut self) -> AXIRAM2SO_W {
        AXIRAM2SO_W { w: self }
    }
    #[doc = "Bit 21 - AXIRAM3SO"]
    #[inline(always)]
    pub fn axiram3so(&mut self) -> AXIRAM3SO_W {
        AXIRAM3SO_W { w: self }
    }
    #[doc = "Bit 22 - AHBRAM1SO"]
    #[inline(always)]
    pub fn ahbram1so(&mut self) -> AHBRAM1SO_W {
        AHBRAM1SO_W { w: self }
    }
    #[doc = "Bit 23 - AHBRAM2SO"]
    #[inline(always)]
    pub fn ahbram2so(&mut self) -> AHBRAM2SO_W {
        AHBRAM2SO_W { w: self }
    }
    #[doc = "Bit 24 - ITCMSO"]
    #[inline(always)]
    pub fn itcmso(&mut self) -> ITCMSO_W {
        ITCMSO_W { w: self }
    }
    #[doc = "Bit 25 - GFXSO"]
    #[inline(always)]
    pub fn gfxso(&mut self) -> GFXSO_W {
        GFXSO_W { w: self }
    }
    #[doc = "Bit 26 - HSITFSO"]
    #[inline(always)]
    pub fn hsitfso(&mut self) -> HSITFSO_W {
        HSITFSO_W { w: self }
    }
    #[doc = "Bit 27 - SRDRAMSO"]
    #[inline(always)]
    pub fn srdramso(&mut self) -> SRDRAMSO_W {
        SRDRAMSO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0xf000_c000"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_c000
    }
}
