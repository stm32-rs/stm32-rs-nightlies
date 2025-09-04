///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**Enable Wakeup pin WKUP1 for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    ///0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU1
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Disabled,
            true => EWUP1::Enabled,
        }
    }
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1::Disabled
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1::Enabled
    }
}
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU1
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Disabled)
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Enabled)
    }
}
/**Enable Wakeup pin WKUP2 for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2 {
    ///0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP2> for bool {
    #[inline(always)]
    fn from(variant: EWUP2) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU1
pub type EWUP2_R = crate::BitReader<EWUP2>;
impl EWUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP2 {
        match self.bits {
            false => EWUP2::Disabled,
            true => EWUP2::Enabled,
        }
    }
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2::Disabled
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2::Enabled
    }
}
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU1
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG, EWUP2>;
impl<'a, REG> EWUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Disabled)
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP2::Enabled)
    }
}
/**Enable Wakeup pin WKUP3 for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3 {
    ///0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP3> for bool {
    #[inline(always)]
    fn from(variant: EWUP3) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU1
pub type EWUP3_R = crate::BitReader<EWUP3>;
impl EWUP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP3 {
        match self.bits {
            false => EWUP3::Disabled,
            true => EWUP3::Enabled,
        }
    }
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3::Disabled
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3::Enabled
    }
}
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU1
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG, EWUP3>;
impl<'a, REG> EWUP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Disabled)
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP3::Enabled)
    }
}
/**Ultra-low-power enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EULPEN {
    ///0: Disable (the supply voltage is monitored continuously)
    Disabled = 0,
    ///1: Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically
    Enabled = 1,
}
impl From<EULPEN> for bool {
    #[inline(always)]
    fn from(variant: EULPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `EULPEN` reader - Ultra-low-power enable
pub type EULPEN_R = crate::BitReader<EULPEN>;
impl EULPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EULPEN {
        match self.bits {
            false => EULPEN::Disabled,
            true => EULPEN::Enabled,
        }
    }
    ///Disable (the supply voltage is monitored continuously)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EULPEN::Disabled
    }
    ///Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EULPEN::Enabled
    }
}
///Field `EULPEN` writer - Ultra-low-power enable
pub type EULPEN_W<'a, REG> = crate::BitWriter<'a, REG, EULPEN>;
impl<'a, REG> EULPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable (the supply voltage is monitored continuously)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EULPEN::Disabled)
    }
    ///Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EULPEN::Enabled)
    }
}
/**Enable wakeup PVD for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWPVD {
    ///0: PVD not enabled by the sub-GHz radio active state
    Disabled = 0,
    ///1: PVD enabled while the sub-GHz radio is active
    Enabled = 1,
}
impl From<EWPVD> for bool {
    #[inline(always)]
    fn from(variant: EWPVD) -> Self {
        variant as u8 != 0
    }
}
///Field `EWPVD` reader - Enable wakeup PVD for CPU1
pub type EWPVD_R = crate::BitReader<EWPVD>;
impl EWPVD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWPVD {
        match self.bits {
            false => EWPVD::Disabled,
            true => EWPVD::Enabled,
        }
    }
    ///PVD not enabled by the sub-GHz radio active state
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWPVD::Disabled
    }
    ///PVD enabled while the sub-GHz radio is active
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWPVD::Enabled
    }
}
///Field `EWPVD` writer - Enable wakeup PVD for CPU1
pub type EWPVD_W<'a, REG> = crate::BitWriter<'a, REG, EWPVD>;
impl<'a, REG> EWPVD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD not enabled by the sub-GHz radio active state
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWPVD::Disabled)
    }
    ///PVD enabled while the sub-GHz radio is active
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWPVD::Enabled)
    }
}
/**SRAM2 retention in Standby mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS {
    ///0: SRAM2 powered off in Standby mode (SRAM2 content lost)
    PowerOff = 0,
    ///1: SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)
    OnLpr = 1,
}
impl From<RRS> for bool {
    #[inline(always)]
    fn from(variant: RRS) -> Self {
        variant as u8 != 0
    }
}
///Field `RRS` reader - SRAM2 retention in Standby mode
pub type RRS_R = crate::BitReader<RRS>;
impl RRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRS {
        match self.bits {
            false => RRS::PowerOff,
            true => RRS::OnLpr,
        }
    }
    ///SRAM2 powered off in Standby mode (SRAM2 content lost)
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == RRS::PowerOff
    }
    ///SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)
    #[inline(always)]
    pub fn is_on_lpr(&self) -> bool {
        *self == RRS::OnLpr
    }
}
///Field `RRS` writer - SRAM2 retention in Standby mode
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG, RRS>;
impl<'a, REG> RRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 powered off in Standby mode (SRAM2 content lost)
    #[inline(always)]
    pub fn power_off(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PowerOff)
    }
    ///SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)
    #[inline(always)]
    pub fn on_lpr(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::OnLpr)
    }
}
/**Apply pull-up and pull-down configuration from CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC {
    ///0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    Disabled = 0,
    ///1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    Enabled = 1,
}
impl From<APC> for bool {
    #[inline(always)]
    fn from(variant: APC) -> Self {
        variant as u8 != 0
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration from CPU1
pub type APC_R = crate::BitReader<APC>;
impl APC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APC {
        match self.bits {
            false => APC::Disabled,
            true => APC::Enabled,
        }
    }
    ///I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC::Disabled
    }
    ///PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC::Enabled
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration from CPU1
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG, APC>;
impl<'a, REG> APC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Disabled)
    }
    ///PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Enabled)
    }
}
/**Enable Radio BUSY Wakeup from Standby for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRFBUSY {
    ///0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs
    Disabled = 0,
    ///1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    Enabled = 1,
}
impl From<EWRFBUSY> for bool {
    #[inline(always)]
    fn from(variant: EWRFBUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `EWRFBUSY` reader - Enable Radio BUSY Wakeup from Standby for CPU1
pub type EWRFBUSY_R = crate::BitReader<EWRFBUSY>;
impl EWRFBUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWRFBUSY {
        match self.bits {
            false => EWRFBUSY::Disabled,
            true => EWRFBUSY::Enabled,
        }
    }
    ///Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFBUSY::Disabled
    }
    ///Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFBUSY::Enabled
    }
}
///Field `EWRFBUSY` writer - Enable Radio BUSY Wakeup from Standby for CPU1
pub type EWRFBUSY_W<'a, REG> = crate::BitWriter<'a, REG, EWRFBUSY>;
impl<'a, REG> EWRFBUSY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWRFBUSY::Disabled)
    }
    ///Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWRFBUSY::Enabled)
    }
}
/**akeup for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRFIRQ {
    ///0: Radio IRQ\[2:0\] is disabled and does not trigger a wakeup from Standby event to CPU1.
    Disabled = 0,
    ///1: Radio IRQ\[2:0\] is enabled and triggers a wakeup from Standby event to CPU1.
    Enabled = 1,
}
impl From<EWRFIRQ> for bool {
    #[inline(always)]
    fn from(variant: EWRFIRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `EWRFIRQ` reader - akeup for CPU1
pub type EWRFIRQ_R = crate::BitReader<EWRFIRQ>;
impl EWRFIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWRFIRQ {
        match self.bits {
            false => EWRFIRQ::Disabled,
            true => EWRFIRQ::Enabled,
        }
    }
    ///Radio IRQ\[2:0\] is disabled and does not trigger a wakeup from Standby event to CPU1.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFIRQ::Disabled
    }
    ///Radio IRQ\[2:0\] is enabled and triggers a wakeup from Standby event to CPU1.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFIRQ::Enabled
    }
}
///Field `EWRFIRQ` writer - akeup for CPU1
pub type EWRFIRQ_W<'a, REG> = crate::BitWriter<'a, REG, EWRFIRQ>;
impl<'a, REG> EWRFIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Radio IRQ\[2:0\] is disabled and does not trigger a wakeup from Standby event to CPU1.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWRFIRQ::Disabled)
    }
    ///Radio IRQ\[2:0\] is enabled and triggers a wakeup from Standby event to CPU1.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWRFIRQ::Enabled)
    }
}
///Field `EC2H` reader - nable CPU2 Hold interrupt for CPU1
pub type EC2H_R = crate::BitReader;
///Field `EC2H` writer - nable CPU2 Hold interrupt for CPU1
pub type EC2H_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Enable internal wakeup line for CPU1

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL {
    ///0: Internal wakeup line interrupt to CPU1 disabled
    Disabled = 0,
    ///1: Internal wakeup line interrupt to CPU1 enabled
    Enabled = 1,
}
impl From<EIWUL> for bool {
    #[inline(always)]
    fn from(variant: EIWUL) -> Self {
        variant as u8 != 0
    }
}
///Field `EIWUL` reader - Enable internal wakeup line for CPU1
pub type EIWUL_R = crate::BitReader<EIWUL>;
impl EIWUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIWUL {
        match self.bits {
            false => EIWUL::Disabled,
            true => EIWUL::Enabled,
        }
    }
    ///Internal wakeup line interrupt to CPU1 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL::Disabled
    }
    ///Internal wakeup line interrupt to CPU1 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL::Enabled
    }
}
///Field `EIWUL` writer - Enable internal wakeup line for CPU1
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG, EIWUL>;
impl<'a, REG> EIWUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal wakeup line interrupt to CPU1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Disabled)
    }
    ///Internal wakeup line interrupt to CPU1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Enabled)
    }
}
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU1
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU1
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - Ultra-low-power enable
    #[inline(always)]
    pub fn eulpen(&self) -> EULPEN_R {
        EULPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable wakeup PVD for CPU1
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 retention in Standby mode
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration from CPU1
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - akeup for CPU1
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - nable CPU2 Hold interrupt for CPU1
    #[inline(always)]
    pub fn ec2h(&self) -> EC2H_R {
        EC2H_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line for CPU1
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("eiwul", &self.eiwul())
            .field("ec2h", &self.ec2h())
            .field("ewrfirq", &self.ewrfirq())
            .field("ewrfbusy", &self.ewrfbusy())
            .field("apc", &self.apc())
            .field("rrs", &self.rrs())
            .field("ewpvd", &self.ewpvd())
            .field("eulpen", &self.eulpen())
            .field("ewup3", &self.ewup3())
            .field("ewup2", &self.ewup2())
            .field("ewup1", &self.ewup1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU1
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU1
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU1
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 7 - Ultra-low-power enable
    #[inline(always)]
    pub fn eulpen(&mut self) -> EULPEN_W<CR3rs> {
        EULPEN_W::new(self, 7)
    }
    ///Bit 8 - Enable wakeup PVD for CPU1
    #[inline(always)]
    pub fn ewpvd(&mut self) -> EWPVD_W<CR3rs> {
        EWPVD_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 retention in Standby mode
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<CR3rs> {
        RRS_W::new(self, 9)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration from CPU1
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<CR3rs> {
        APC_W::new(self, 10)
    }
    ///Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1
    #[inline(always)]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W<CR3rs> {
        EWRFBUSY_W::new(self, 11)
    }
    ///Bit 13 - akeup for CPU1
    #[inline(always)]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W<CR3rs> {
        EWRFIRQ_W::new(self, 13)
    }
    ///Bit 14 - nable CPU2 Hold interrupt for CPU1
    #[inline(always)]
    pub fn ec2h(&mut self) -> EC2H_W<CR3rs> {
        EC2H_W::new(self, 14)
    }
    ///Bit 15 - Enable internal wakeup line for CPU1
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**Power control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x8000
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
