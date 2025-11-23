///Register `CNTR` reader
pub type R = crate::R<CNTRrs>;
///Register `CNTR` writer
pub type W = crate::W<CNTRrs>;
/**Force USB Reset

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRES {
    ///0: Clear USB reset
    NoReset = 0,
    ///1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
    Reset = 1,
}
impl From<FRES> for bool {
    #[inline(always)]
    fn from(variant: FRES) -> Self {
        variant as u8 != 0
    }
}
///Field `FRES` reader - Force USB Reset
pub type FRES_R = crate::BitReader<FRES>;
impl FRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRES {
        match self.bits {
            false => FRES::NoReset,
            true => FRES::Reset,
        }
    }
    ///Clear USB reset
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FRES::NoReset
    }
    ///Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FRES::Reset
    }
}
///Field `FRES` writer - Force USB Reset
pub type FRES_W<'a, REG> = crate::BitWriter<'a, REG, FRES>;
impl<'a, REG> FRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear USB reset
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FRES::NoReset)
    }
    ///Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FRES::Reset)
    }
}
/**Power down

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDWN {
    ///0: No power down
    Disabled = 0,
    ///1: Enter power down mode
    Enabled = 1,
}
impl From<PDWN> for bool {
    #[inline(always)]
    fn from(variant: PDWN) -> Self {
        variant as u8 != 0
    }
}
///Field `PDWN` reader - Power down
pub type PDWN_R = crate::BitReader<PDWN>;
impl PDWN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDWN {
        match self.bits {
            false => PDWN::Disabled,
            true => PDWN::Enabled,
        }
    }
    ///No power down
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDWN::Disabled
    }
    ///Enter power down mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDWN::Enabled
    }
}
///Field `PDWN` writer - Power down
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG, PDWN>;
impl<'a, REG> PDWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No power down
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN::Disabled)
    }
    ///Enter power down mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN::Enabled)
    }
}
/**Low-power mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMODE {
    ///0: No low-power mode
    Disabled = 0,
    ///1: Enter low-power mode
    Enabled = 1,
}
impl From<LPMODE> for bool {
    #[inline(always)]
    fn from(variant: LPMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMODE` reader - Low-power mode
pub type LPMODE_R = crate::BitReader<LPMODE>;
impl LPMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMODE {
        match self.bits {
            false => LPMODE::Disabled,
            true => LPMODE::Enabled,
        }
    }
    ///No low-power mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMODE::Disabled
    }
    ///Enter low-power mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMODE::Enabled
    }
}
///Field `LPMODE` writer - Low-power mode
pub type LPMODE_W<'a, REG> = crate::BitWriter<'a, REG, LPMODE>;
impl<'a, REG> LPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No low-power mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMODE::Disabled)
    }
    ///Enter low-power mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMODE::Enabled)
    }
}
/**Force suspend

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSUSP {
    ///0: No effect
    NoEffect = 0,
    ///1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
    Suspend = 1,
}
impl From<FSUSP> for bool {
    #[inline(always)]
    fn from(variant: FSUSP) -> Self {
        variant as u8 != 0
    }
}
///Field `FSUSP` reader - Force suspend
pub type FSUSP_R = crate::BitReader<FSUSP>;
impl FSUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSUSP {
        match self.bits {
            false => FSUSP::NoEffect,
            true => FSUSP::Suspend,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FSUSP::NoEffect
    }
    ///Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSUSP::Suspend
    }
}
///Field `FSUSP` writer - Force suspend
pub type FSUSP_W<'a, REG> = crate::BitWriter<'a, REG, FSUSP>;
impl<'a, REG> FSUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FSUSP::NoEffect)
    }
    ///Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(FSUSP::Suspend)
    }
}
/**Resume request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME {
    ///1: Resume requested
    Requested = 1,
}
impl From<RESUME> for bool {
    #[inline(always)]
    fn from(variant: RESUME) -> Self {
        variant as u8 != 0
    }
}
///Field `RESUME` reader - Resume request
pub type RESUME_R = crate::BitReader<RESUME>;
impl RESUME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESUME> {
        match self.bits {
            true => Some(RESUME::Requested),
            _ => None,
        }
    }
    ///Resume requested
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == RESUME::Requested
    }
}
///Field `RESUME` writer - Resume request
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG, RESUME>;
impl<'a, REG> RESUME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resume requested
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME::Requested)
    }
}
/**Expected start of frame interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFM {
    ///0: ESOF Interrupt disabled
    Disabled = 0,
    ///1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<ESOFM> for bool {
    #[inline(always)]
    fn from(variant: ESOFM) -> Self {
        variant as u8 != 0
    }
}
///Field `ESOFM` reader - Expected start of frame interrupt mask
pub type ESOFM_R = crate::BitReader<ESOFM>;
impl ESOFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESOFM {
        match self.bits {
            false => ESOFM::Disabled,
            true => ESOFM::Enabled,
        }
    }
    ///ESOF Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESOFM::Disabled
    }
    ///ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESOFM::Enabled
    }
}
///Field `ESOFM` writer - Expected start of frame interrupt mask
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG, ESOFM>;
impl<'a, REG> ESOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ESOF Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM::Disabled)
    }
    ///ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM::Enabled)
    }
}
/**Start of frame interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFM {
    ///0: SOF Interrupt disabled
    Disabled = 0,
    ///1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<SOFM> for bool {
    #[inline(always)]
    fn from(variant: SOFM) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFM` reader - Start of frame interrupt mask
pub type SOFM_R = crate::BitReader<SOFM>;
impl SOFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFM {
        match self.bits {
            false => SOFM::Disabled,
            true => SOFM::Enabled,
        }
    }
    ///SOF Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOFM::Disabled
    }
    ///SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOFM::Enabled
    }
}
///Field `SOFM` writer - Start of frame interrupt mask
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG, SOFM>;
impl<'a, REG> SOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SOF Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM::Disabled)
    }
    ///SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM::Enabled)
    }
}
/**USB reset interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETM {
    ///0: RESET Interrupt disabled
    Disabled = 0,
    ///1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<RESETM> for bool {
    #[inline(always)]
    fn from(variant: RESETM) -> Self {
        variant as u8 != 0
    }
}
///Field `RESETM` reader - USB reset interrupt mask
pub type RESETM_R = crate::BitReader<RESETM>;
impl RESETM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESETM {
        match self.bits {
            false => RESETM::Disabled,
            true => RESETM::Enabled,
        }
    }
    ///RESET Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETM::Disabled
    }
    ///RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETM::Enabled
    }
}
///Field `RESETM` writer - USB reset interrupt mask
pub type RESETM_W<'a, REG> = crate::BitWriter<'a, REG, RESETM>;
impl<'a, REG> RESETM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RESET Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RESETM::Disabled)
    }
    ///RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RESETM::Enabled)
    }
}
/**Suspend mode interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPM {
    ///0: Suspend Mode Request SUSP Interrupt disabled
    Disabled = 0,
    ///1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<SUSPM> for bool {
    #[inline(always)]
    fn from(variant: SUSPM) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSPM` reader - Suspend mode interrupt mask
pub type SUSPM_R = crate::BitReader<SUSPM>;
impl SUSPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSPM {
        match self.bits {
            false => SUSPM::Disabled,
            true => SUSPM::Enabled,
        }
    }
    ///Suspend Mode Request SUSP Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPM::Disabled
    }
    ///SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPM::Enabled
    }
}
///Field `SUSPM` writer - Suspend mode interrupt mask
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG, SUSPM>;
impl<'a, REG> SUSPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Suspend Mode Request SUSP Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM::Disabled)
    }
    ///SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM::Enabled)
    }
}
/**Wakeup interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPM {
    ///0: WKUP Interrupt disabled
    Disabled = 0,
    ///1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<WKUPM> for bool {
    #[inline(always)]
    fn from(variant: WKUPM) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUPM` reader - Wakeup interrupt mask
pub type WKUPM_R = crate::BitReader<WKUPM>;
impl WKUPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WKUPM {
        match self.bits {
            false => WKUPM::Disabled,
            true => WKUPM::Enabled,
        }
    }
    ///WKUP Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUPM::Disabled
    }
    ///WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUPM::Enabled
    }
}
///Field `WKUPM` writer - Wakeup interrupt mask
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG, WKUPM>;
impl<'a, REG> WKUPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM::Disabled)
    }
    ///WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM::Enabled)
    }
}
/**Error interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRM {
    ///0: ERR Interrupt disabled
    Disabled = 0,
    ///1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<ERRM> for bool {
    #[inline(always)]
    fn from(variant: ERRM) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRM` reader - Error interrupt mask
pub type ERRM_R = crate::BitReader<ERRM>;
impl ERRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRM {
        match self.bits {
            false => ERRM::Disabled,
            true => ERRM::Enabled,
        }
    }
    ///ERR Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRM::Disabled
    }
    ///ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRM::Enabled
    }
}
///Field `ERRM` writer - Error interrupt mask
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG, ERRM>;
impl<'a, REG> ERRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ERR Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM::Disabled)
    }
    ///ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM::Enabled)
    }
}
/**Packet memory area over / underrun interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRM {
    ///0: PMAOVR Interrupt disabled
    Disabled = 0,
    ///1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<PMAOVRM> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM) -> Self {
        variant as u8 != 0
    }
}
///Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_R = crate::BitReader<PMAOVRM>;
impl PMAOVRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRM {
        match self.bits {
            false => PMAOVRM::Disabled,
            true => PMAOVRM::Enabled,
        }
    }
    ///PMAOVR Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PMAOVRM::Disabled
    }
    ///PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PMAOVRM::Enabled
    }
}
///Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG, PMAOVRM>;
impl<'a, REG> PMAOVRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PMAOVR Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM::Disabled)
    }
    ///PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM::Enabled)
    }
}
/**Correct transfer interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRM {
    ///0: Correct Transfer (CTR) Interrupt disabled
    Disabled = 0,
    ///1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    Enabled = 1,
}
impl From<CTRM> for bool {
    #[inline(always)]
    fn from(variant: CTRM) -> Self {
        variant as u8 != 0
    }
}
///Field `CTRM` reader - Correct transfer interrupt mask
pub type CTRM_R = crate::BitReader<CTRM>;
impl CTRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTRM {
        match self.bits {
            false => CTRM::Disabled,
            true => CTRM::Enabled,
        }
    }
    ///Correct Transfer (CTR) Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTRM::Disabled
    }
    ///CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTRM::Enabled
    }
}
///Field `CTRM` writer - Correct transfer interrupt mask
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG, CTRM>;
impl<'a, REG> CTRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Correct Transfer (CTR) Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM::Disabled)
    }
    ///CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM::Enabled)
    }
}
impl R {
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTR")
            .field("fres", &self.fres())
            .field("pdwn", &self.pdwn())
            .field("lpmode", &self.lpmode())
            .field("fsusp", &self.fsusp())
            .field("resume", &self.resume())
            .field("esofm", &self.esofm())
            .field("sofm", &self.sofm())
            .field("resetm", &self.resetm())
            .field("suspm", &self.suspm())
            .field("wkupm", &self.wkupm())
            .field("errm", &self.errm())
            .field("pmaovrm", &self.pmaovrm())
            .field("ctrm", &self.ctrm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W<'_, CNTRrs> {
        FRES_W::new(self, 0)
    }
    ///Bit 1 - Power down
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W<'_, CNTRrs> {
        PDWN_W::new(self, 1)
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W<'_, CNTRrs> {
        LPMODE_W::new(self, 2)
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W<'_, CNTRrs> {
        FSUSP_W::new(self, 3)
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<'_, CNTRrs> {
        RESUME_W::new(self, 4)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W<'_, CNTRrs> {
        ESOFM_W::new(self, 8)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<'_, CNTRrs> {
        SOFM_W::new(self, 9)
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W<'_, CNTRrs> {
        RESETM_W::new(self, 10)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W<'_, CNTRrs> {
        SUSPM_W::new(self, 11)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W<'_, CNTRrs> {
        WKUPM_W::new(self, 12)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W<'_, CNTRrs> {
        ERRM_W::new(self, 13)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<'_, CNTRrs> {
        PMAOVRM_W::new(self, 14)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W<'_, CNTRrs> {
        CTRM_W::new(self, 15)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#USB:CNTR)*/
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`cntr::R`](R) reader structure
impl crate::Readable for CNTRrs {}
///`write(|w| ..)` method takes [`cntr::W`](W) writer structure
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTR to value 0x03
impl crate::Resettable for CNTRrs {
    const RESET_VALUE: u32 = 0x03;
}
