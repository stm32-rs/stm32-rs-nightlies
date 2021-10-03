#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable internal wakeup line for CPU1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIWUL_A {
    #[doc = "0: Internal wakeup line interrupt to CPU1 disabled"]
    DISABLED = 0,
    #[doc = "1: Internal wakeup line interrupt to CPU1 enabled"]
    ENABLED = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line for CPU1"]
pub struct EIWUL_R(crate::FieldReader<bool, EIWUL_A>);
impl EIWUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIWUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::DISABLED,
            true => EIWUL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EIWUL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EIWUL_A::ENABLED
    }
}
impl core::ops::Deref for EIWUL_R {
    type Target = crate::FieldReader<bool, EIWUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line for CPU1"]
pub struct EIWUL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIWUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIWUL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal wakeup line interrupt to CPU1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIWUL_A::DISABLED)
    }
    #[doc = "Internal wakeup line interrupt to CPU1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIWUL_A::ENABLED)
    }
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
#[doc = "Field `EC2H` reader - nable CPU2 Hold interrupt for CPU1"]
pub struct EC2H_R(crate::FieldReader<bool, bool>);
impl EC2H_R {
    pub(crate) fn new(bits: bool) -> Self {
        EC2H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC2H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC2H` writer - nable CPU2 Hold interrupt for CPU1"]
pub struct EC2H_W<'a> {
    w: &'a mut W,
}
impl<'a> EC2H_W<'a> {
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
#[doc = "akeup for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFIRQ_A {
    #[doc = "0: Radio IRQ\\[2:0\\]
is disabled and does not trigger a wakeup from Standby event to CPU1."]
    DISABLED = 0,
    #[doc = "1: Radio IRQ\\[2:0\\]
is enabled and triggers a wakeup from Standby event to CPU1."]
    ENABLED = 1,
}
impl From<EWRFIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWRFIRQ` reader - akeup for CPU1"]
pub struct EWRFIRQ_R(crate::FieldReader<bool, EWRFIRQ_A>);
impl EWRFIRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWRFIRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRFIRQ_A {
        match self.bits {
            false => EWRFIRQ_A::DISABLED,
            true => EWRFIRQ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWRFIRQ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWRFIRQ_A::ENABLED
    }
}
impl core::ops::Deref for EWRFIRQ_R {
    type Target = crate::FieldReader<bool, EWRFIRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWRFIRQ` writer - akeup for CPU1"]
pub struct EWRFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWRFIRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Radio IRQ\\[2:0\\]
is disabled and does not trigger a wakeup from Standby event to CPU1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::DISABLED)
    }
    #[doc = "Radio IRQ\\[2:0\\]
is enabled and triggers a wakeup from Standby event to CPU1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::ENABLED)
    }
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
#[doc = "Enable Radio BUSY Wakeup from Standby for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFBUSY_A {
    #[doc = "0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs"]
    DISABLED = 0,
    #[doc = "1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4"]
    ENABLED = 1,
}
impl From<EWRFBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWRFBUSY` reader - Enable Radio BUSY Wakeup from Standby for CPU1"]
pub struct EWRFBUSY_R(crate::FieldReader<bool, EWRFBUSY_A>);
impl EWRFBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWRFBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRFBUSY_A {
        match self.bits {
            false => EWRFBUSY_A::DISABLED,
            true => EWRFBUSY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWRFBUSY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWRFBUSY_A::ENABLED
    }
}
impl core::ops::Deref for EWRFBUSY_R {
    type Target = crate::FieldReader<bool, EWRFBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWRFBUSY` writer - Enable Radio BUSY Wakeup from Standby for CPU1"]
pub struct EWRFBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWRFBUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::DISABLED)
    }
    #[doc = "Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Apply pull-up and pull-down configuration from CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APC_A {
    #[doc = "0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied"]
    DISABLED = 0,
    #[doc = "1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os"]
    ENABLED = 1,
}
impl From<APC_A> for bool {
    #[inline(always)]
    fn from(variant: APC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration from CPU1"]
pub struct APC_R(crate::FieldReader<bool, APC_A>);
impl APC_R {
    pub(crate) fn new(bits: bool) -> Self {
        APC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APC_A {
        match self.bits {
            false => APC_A::DISABLED,
            true => APC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == APC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == APC_A::ENABLED
    }
}
impl core::ops::Deref for APC_R {
    type Target = crate::FieldReader<bool, APC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration from CPU1"]
pub struct APC_W<'a> {
    w: &'a mut W,
}
impl<'a> APC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APC_A::DISABLED)
    }
    #[doc = "PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(APC_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: SRAM2 powered off in Standby mode (SRAM2 content lost)"]
    POWEROFF = 0,
    #[doc = "1: SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)"]
    ONLPR = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub struct RRS_R(crate::FieldReader<bool, RRS_A>);
impl RRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::POWEROFF,
            true => RRS_A::ONLPR,
        }
    }
    #[doc = "Checks if the value of the field is `POWEROFF`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        **self == RRS_A::POWEROFF
    }
    #[doc = "Checks if the value of the field is `ONLPR`"]
    #[inline(always)]
    pub fn is_on_lpr(&self) -> bool {
        **self == RRS_A::ONLPR
    }
}
impl core::ops::Deref for RRS_R {
    type Target = crate::FieldReader<bool, RRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM2 powered off in Standby mode (SRAM2 content lost)"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(RRS_A::POWEROFF)
    }
    #[doc = "SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)"]
    #[inline(always)]
    pub fn on_lpr(self) -> &'a mut W {
        self.variant(RRS_A::ONLPR)
    }
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
#[doc = "Enable wakeup PVD for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWPVD_A {
    #[doc = "0: PVD not enabled by the sub-GHz radio active state"]
    DISABLED = 0,
    #[doc = "1: PVD enabled while the sub-GHz radio is active"]
    ENABLED = 1,
}
impl From<EWPVD_A> for bool {
    #[inline(always)]
    fn from(variant: EWPVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWPVD` reader - Enable wakeup PVD for CPU1"]
pub struct EWPVD_R(crate::FieldReader<bool, EWPVD_A>);
impl EWPVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWPVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWPVD_A {
        match self.bits {
            false => EWPVD_A::DISABLED,
            true => EWPVD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWPVD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWPVD_A::ENABLED
    }
}
impl core::ops::Deref for EWPVD_R {
    type Target = crate::FieldReader<bool, EWPVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWPVD` writer - Enable wakeup PVD for CPU1"]
pub struct EWPVD_W<'a> {
    w: &'a mut W,
}
impl<'a> EWPVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWPVD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PVD not enabled by the sub-GHz radio active state"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWPVD_A::DISABLED)
    }
    #[doc = "PVD enabled while the sub-GHz radio is active"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWPVD_A::ENABLED)
    }
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
#[doc = "Ultra-low-power enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EULPEN_A {
    #[doc = "0: Disable (the supply voltage is monitored continuously)"]
    DISABLED = 0,
    #[doc = "1: Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically"]
    ENABLED = 1,
}
impl From<EULPEN_A> for bool {
    #[inline(always)]
    fn from(variant: EULPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EULPEN` reader - Ultra-low-power enable"]
pub struct EULPEN_R(crate::FieldReader<bool, EULPEN_A>);
impl EULPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EULPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EULPEN_A {
        match self.bits {
            false => EULPEN_A::DISABLED,
            true => EULPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EULPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EULPEN_A::ENABLED
    }
}
impl core::ops::Deref for EULPEN_R {
    type Target = crate::FieldReader<bool, EULPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EULPEN` writer - Ultra-low-power enable"]
pub struct EULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EULPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EULPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable (the supply voltage is monitored continuously)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EULPEN_A::DISABLED)
    }
    #[doc = "Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EULPEN_A::ENABLED)
    }
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
#[doc = "Enable Wakeup pin WKUP3 for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP3_A {
    #[doc = "0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    DISABLED = 0,
    #[doc = "1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    ENABLED = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU1"]
pub struct EWUP3_R(crate::FieldReader<bool, EWUP3_A>);
impl EWUP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::DISABLED,
            true => EWUP3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP3_A::ENABLED
    }
}
impl core::ops::Deref for EWUP3_R {
    type Target = crate::FieldReader<bool, EWUP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU1"]
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWUP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::DISABLED)
    }
    #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::ENABLED)
    }
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
#[doc = "Enable Wakeup pin WKUP2 for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP2_A {
    #[doc = "0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    DISABLED = 0,
    #[doc = "1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    ENABLED = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU1"]
pub struct EWUP2_R(crate::FieldReader<bool, EWUP2_A>);
impl EWUP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::DISABLED,
            true => EWUP2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP2_A::ENABLED
    }
}
impl core::ops::Deref for EWUP2_R {
    type Target = crate::FieldReader<bool, EWUP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU1"]
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWUP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::DISABLED)
    }
    #[doc = "WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::ENABLED)
    }
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
#[doc = "Enable Wakeup pin WKUP1 for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP1_A {
    #[doc = "0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    DISABLED = 0,
    #[doc = "1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    ENABLED = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU1"]
pub struct EWUP1_R(crate::FieldReader<bool, EWUP1_A>);
impl EWUP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::DISABLED,
            true => EWUP1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP1_A::ENABLED
    }
}
impl core::ops::Deref for EWUP1_R {
    type Target = crate::FieldReader<bool, EWUP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU1"]
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWUP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::DISABLED)
    }
    #[doc = "WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::ENABLED)
    }
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
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - nable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    pub fn ec2h(&self) -> EC2H_R {
        EC2H_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - akeup for CPU1"]
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1"]
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration from CPU1"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU1"]
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ultra-low-power enable"]
    #[inline(always)]
    pub fn eulpen(&self) -> EULPEN_R {
        EULPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU1"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU1"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W {
        EIWUL_W { w: self }
    }
    #[doc = "Bit 14 - nable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    pub fn ec2h(&mut self) -> EC2H_W {
        EC2H_W { w: self }
    }
    #[doc = "Bit 13 - akeup for CPU1"]
    #[inline(always)]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W {
        EWRFIRQ_W { w: self }
    }
    #[doc = "Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1"]
    #[inline(always)]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W {
        EWRFBUSY_W { w: self }
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration from CPU1"]
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W {
        APC_W { w: self }
    }
    #[doc = "Bit 9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU1"]
    #[inline(always)]
    pub fn ewpvd(&mut self) -> EWPVD_W {
        EWPVD_W { w: self }
    }
    #[doc = "Bit 7 - Ultra-low-power enable"]
    #[inline(always)]
    pub fn eulpen(&mut self) -> EULPEN_W {
        EULPEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU1"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU1"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU1"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
