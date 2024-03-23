#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CNTRrs>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CNTRrs>;
#[doc = "Force USB Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRES {
    #[doc = "0: Clear USB reset"]
    NoReset = 0,
    #[doc = "1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    Reset = 1,
}
impl From<FRES> for bool {
    #[inline(always)]
    fn from(variant: FRES) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRES` reader - Force USB Reset"]
pub type FRES_R = crate::BitReader<FRES>;
impl FRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRES {
        match self.bits {
            false => FRES::NoReset,
            true => FRES::Reset,
        }
    }
    #[doc = "Clear USB reset"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FRES::NoReset
    }
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FRES::Reset
    }
}
#[doc = "Field `FRES` writer - Force USB Reset"]
pub type FRES_W<'a, REG> = crate::BitWriter<'a, REG, FRES>;
impl<'a, REG> FRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear USB reset"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FRES::NoReset)
    }
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FRES::Reset)
    }
}
#[doc = "Power down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDWN {
    #[doc = "0: No power down"]
    Disabled = 0,
    #[doc = "1: Enter power down mode"]
    Enabled = 1,
}
impl From<PDWN> for bool {
    #[inline(always)]
    fn from(variant: PDWN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWN` reader - Power down"]
pub type PDWN_R = crate::BitReader<PDWN>;
impl PDWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDWN {
        match self.bits {
            false => PDWN::Disabled,
            true => PDWN::Enabled,
        }
    }
    #[doc = "No power down"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDWN::Disabled
    }
    #[doc = "Enter power down mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDWN::Enabled
    }
}
#[doc = "Field `PDWN` writer - Power down"]
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG, PDWN>;
impl<'a, REG> PDWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No power down"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN::Disabled)
    }
    #[doc = "Enter power down mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN::Enabled)
    }
}
#[doc = "Low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMODE {
    #[doc = "0: No low-power mode"]
    Disabled = 0,
    #[doc = "1: Enter low-power mode"]
    Enabled = 1,
}
impl From<LPMODE> for bool {
    #[inline(always)]
    fn from(variant: LPMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMODE` reader - Low-power mode"]
pub type LPMODE_R = crate::BitReader<LPMODE>;
impl LPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMODE {
        match self.bits {
            false => LPMODE::Disabled,
            true => LPMODE::Enabled,
        }
    }
    #[doc = "No low-power mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMODE::Disabled
    }
    #[doc = "Enter low-power mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMODE::Enabled
    }
}
#[doc = "Field `LPMODE` writer - Low-power mode"]
pub type LPMODE_W<'a, REG> = crate::BitWriter<'a, REG, LPMODE>;
impl<'a, REG> LPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMODE::Disabled)
    }
    #[doc = "Enter low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMODE::Enabled)
    }
}
#[doc = "Force suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSUSP {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    Suspend = 1,
}
impl From<FSUSP> for bool {
    #[inline(always)]
    fn from(variant: FSUSP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSUSP` reader - Force suspend"]
pub type FSUSP_R = crate::BitReader<FSUSP>;
impl FSUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSUSP {
        match self.bits {
            false => FSUSP::NoEffect,
            true => FSUSP::Suspend,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FSUSP::NoEffect
    }
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSUSP::Suspend
    }
}
#[doc = "Field `FSUSP` writer - Force suspend"]
pub type FSUSP_W<'a, REG> = crate::BitWriter<'a, REG, FSUSP>;
impl<'a, REG> FSUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FSUSP::NoEffect)
    }
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(FSUSP::Suspend)
    }
}
#[doc = "Resume request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME {
    #[doc = "1: Resume requested"]
    Requested = 1,
}
impl From<RESUME> for bool {
    #[inline(always)]
    fn from(variant: RESUME) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Resume request"]
pub type RESUME_R = crate::BitReader<RESUME>;
impl RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESUME> {
        match self.bits {
            true => Some(RESUME::Requested),
            _ => None,
        }
    }
    #[doc = "Resume requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == RESUME::Requested
    }
}
#[doc = "Field `RESUME` writer - Resume request"]
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG, RESUME>;
impl<'a, REG> RESUME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME::Requested)
    }
}
#[doc = "LPM L1 Resume request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RESUME {
    #[doc = "1: LPM L1 request requested"]
    Requested = 1,
}
impl From<L1RESUME> for bool {
    #[inline(always)]
    fn from(variant: L1RESUME) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1RESUME` reader - LPM L1 Resume request"]
pub type L1RESUME_R = crate::BitReader<L1RESUME>;
impl L1RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<L1RESUME> {
        match self.bits {
            true => Some(L1RESUME::Requested),
            _ => None,
        }
    }
    #[doc = "LPM L1 request requested"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == L1RESUME::Requested
    }
}
#[doc = "Field `L1RESUME` writer - LPM L1 Resume request"]
pub type L1RESUME_W<'a, REG> = crate::BitWriter<'a, REG, L1RESUME>;
impl<'a, REG> L1RESUME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM L1 request requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESUME::Requested)
    }
}
#[doc = "LPM L1 state request interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQM {
    #[doc = "0: L1REQ Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<L1REQM> for bool {
    #[inline(always)]
    fn from(variant: L1REQM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1REQM_R = crate::BitReader<L1REQM>;
impl L1REQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1REQM {
        match self.bits {
            false => L1REQM::Disabled,
            true => L1REQM::Enabled,
        }
    }
    #[doc = "L1REQ Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == L1REQM::Disabled
    }
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == L1REQM::Enabled
    }
}
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG, L1REQM>;
impl<'a, REG> L1REQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "L1REQ Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM::Disabled)
    }
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM::Enabled)
    }
}
#[doc = "Expected start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFM {
    #[doc = "0: ESOF Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<ESOFM> for bool {
    #[inline(always)]
    fn from(variant: ESOFM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader<ESOFM>;
impl ESOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESOFM {
        match self.bits {
            false => ESOFM::Disabled,
            true => ESOFM::Enabled,
        }
    }
    #[doc = "ESOF Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESOFM::Disabled
    }
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESOFM::Enabled
    }
}
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG, ESOFM>;
impl<'a, REG> ESOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ESOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM::Disabled)
    }
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM::Enabled)
    }
}
#[doc = "Start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFM {
    #[doc = "0: SOF Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<SOFM> for bool {
    #[inline(always)]
    fn from(variant: SOFM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader<SOFM>;
impl SOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFM {
        match self.bits {
            false => SOFM::Disabled,
            true => SOFM::Enabled,
        }
    }
    #[doc = "SOF Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOFM::Disabled
    }
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOFM::Enabled
    }
}
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG, SOFM>;
impl<'a, REG> SOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM::Disabled)
    }
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM::Enabled)
    }
}
#[doc = "USB reset interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETM {
    #[doc = "0: RESET Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<RESETM> for bool {
    #[inline(always)]
    fn from(variant: RESETM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETM` reader - USB reset interrupt mask"]
pub type RESETM_R = crate::BitReader<RESETM>;
impl RESETM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESETM {
        match self.bits {
            false => RESETM::Disabled,
            true => RESETM::Enabled,
        }
    }
    #[doc = "RESET Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETM::Disabled
    }
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETM::Enabled
    }
}
#[doc = "Field `RESETM` writer - USB reset interrupt mask"]
pub type RESETM_W<'a, REG> = crate::BitWriter<'a, REG, RESETM>;
impl<'a, REG> RESETM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESET Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RESETM::Disabled)
    }
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RESETM::Enabled)
    }
}
#[doc = "Suspend mode interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPM {
    #[doc = "0: Suspend Mode Request SUSP Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<SUSPM> for bool {
    #[inline(always)]
    fn from(variant: SUSPM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader<SUSPM>;
impl SUSPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPM {
        match self.bits {
            false => SUSPM::Disabled,
            true => SUSPM::Enabled,
        }
    }
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPM::Disabled
    }
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPM::Enabled
    }
}
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG, SUSPM>;
impl<'a, REG> SUSPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM::Disabled)
    }
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM::Enabled)
    }
}
#[doc = "Wakeup interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPM {
    #[doc = "0: WKUP Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<WKUPM> for bool {
    #[inline(always)]
    fn from(variant: WKUPM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader<WKUPM>;
impl WKUPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUPM {
        match self.bits {
            false => WKUPM::Disabled,
            true => WKUPM::Enabled,
        }
    }
    #[doc = "WKUP Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUPM::Disabled
    }
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUPM::Enabled
    }
}
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG, WKUPM>;
impl<'a, REG> WKUPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM::Disabled)
    }
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM::Enabled)
    }
}
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRM {
    #[doc = "0: ERR Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<ERRM> for bool {
    #[inline(always)]
    fn from(variant: ERRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader<ERRM>;
impl ERRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRM {
        match self.bits {
            false => ERRM::Disabled,
            true => ERRM::Enabled,
        }
    }
    #[doc = "ERR Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRM::Disabled
    }
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRM::Enabled
    }
}
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG, ERRM>;
impl<'a, REG> ERRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM::Disabled)
    }
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM::Enabled)
    }
}
#[doc = "Packet memory area over / underrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRM {
    #[doc = "0: PMAOVR Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<PMAOVRM> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader<PMAOVRM>;
impl PMAOVRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRM {
        match self.bits {
            false => PMAOVRM::Disabled,
            true => PMAOVRM::Enabled,
        }
    }
    #[doc = "PMAOVR Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PMAOVRM::Disabled
    }
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PMAOVRM::Enabled
    }
}
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG, PMAOVRM>;
impl<'a, REG> PMAOVRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMAOVR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM::Disabled)
    }
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM::Enabled)
    }
}
#[doc = "Correct transfer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRM {
    #[doc = "0: Correct Transfer (CTR) Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    Enabled = 1,
}
impl From<CTRM> for bool {
    #[inline(always)]
    fn from(variant: CTRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader<CTRM>;
impl CTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTRM {
        match self.bits {
            false => CTRM::Disabled,
            true => CTRM::Enabled,
        }
    }
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTRM::Disabled
    }
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTRM::Enabled
    }
}
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG, CTRM>;
impl<'a, REG> CTRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM::Disabled)
    }
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<CNTRrs> {
        FRES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<CNTRrs> {
        PDWN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LPMODE_W<CNTRrs> {
        LPMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    #[must_use]
    pub fn fsusp(&mut self) -> FSUSP_W<CNTRrs> {
        FSUSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<CNTRrs> {
        RESUME_W::new(self, 4)
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<CNTRrs> {
        L1RESUME_W::new(self, 5)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<CNTRrs> {
        L1REQM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<CNTRrs> {
        ESOFM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<CNTRrs> {
        SOFM_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn resetm(&mut self) -> RESETM_W<CNTRrs> {
        RESETM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<CNTRrs> {
        SUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<CNTRrs> {
        WKUPM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<CNTRrs> {
        ERRM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<CNTRrs> {
        PMAOVRM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<CNTRrs> {
        CTRM_W::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CNTRrs {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTR to value 0x03"]
impl crate::Resettable for CNTRrs {
    const RESET_VALUE: u32 = 0x03;
}
