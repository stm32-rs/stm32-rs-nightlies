///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CKPSC` reader - HRTIM Master Clock prescaler
pub type CKPSC_R = crate::FieldReader;
///Field `CKPSC` writer - HRTIM Master Clock prescaler
pub type CKPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Master Continuous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: The timer operates in single-shot mode and stops when it reaches the MPER value
    SingleShot = 0,
    ///1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Master Continuous mode
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::SingleShot,
            true => CONT::Continuous,
        }
    }
    ///The timer operates in single-shot mode and stops when it reaches the MPER value
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT::SingleShot
    }
    ///The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - Master Continuous mode
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The timer operates in single-shot mode and stops when it reaches the MPER value
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::SingleShot)
    }
    ///The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**Master Re-triggerable mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETRIG {
    ///0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
    Disabled = 0,
    ///1: The timer is retriggerable: a counter reset is done whatever the counter state
    Enabled = 1,
}
impl From<RETRIG> for bool {
    #[inline(always)]
    fn from(variant: RETRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `RETRIG` reader - Master Re-triggerable mode
pub type RETRIG_R = crate::BitReader<RETRIG>;
impl RETRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RETRIG {
        match self.bits {
            false => RETRIG::Disabled,
            true => RETRIG::Enabled,
        }
    }
    ///The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG::Disabled
    }
    ///The timer is retriggerable: a counter reset is done whatever the counter state
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG::Enabled
    }
}
///Field `RETRIG` writer - Master Re-triggerable mode
pub type RETRIG_W<'a, REG> = crate::BitWriter<'a, REG, RETRIG>;
impl<'a, REG> RETRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETRIG::Disabled)
    }
    ///The timer is retriggerable: a counter reset is done whatever the counter state
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETRIG::Enabled)
    }
}
/**Half mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF {
    ///0: Half mode disabled
    Disabled = 0,
    ///1: Half mode enabled
    Enabled = 1,
}
impl From<HALF> for bool {
    #[inline(always)]
    fn from(variant: HALF) -> Self {
        variant as u8 != 0
    }
}
///Field `HALF` reader - Half mode enable
pub type HALF_R = crate::BitReader<HALF>;
impl HALF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HALF {
        match self.bits {
            false => HALF::Disabled,
            true => HALF::Enabled,
        }
    }
    ///Half mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF::Disabled
    }
    ///Half mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF::Enabled
    }
}
///Field `HALF` writer - Half mode enable
pub type HALF_W<'a, REG> = crate::BitWriter<'a, REG, HALF>;
impl<'a, REG> HALF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALF::Disabled)
    }
    ///Half mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALF::Enabled)
    }
}
/**ynchronization input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCIN {
    ///0: Disabled. HRTIM is not synchronized and runs in standalone mode
    Disabled = 0,
    ///2: Internal event: the HRTIM is synchronized with the on-chip timer
    Internal = 2,
    ///3: External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
    External = 3,
}
impl From<SYNCIN> for u8 {
    #[inline(always)]
    fn from(variant: SYNCIN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCIN {
    type Ux = u8;
}
impl crate::IsEnum for SYNCIN {}
///Field `SYNCIN` reader - ynchronization input
pub type SYNCIN_R = crate::FieldReader<SYNCIN>;
impl SYNCIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCIN> {
        match self.bits {
            0 => Some(SYNCIN::Disabled),
            2 => Some(SYNCIN::Internal),
            3 => Some(SYNCIN::External),
            _ => None,
        }
    }
    ///Disabled. HRTIM is not synchronized and runs in standalone mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCIN::Disabled
    }
    ///Internal event: the HRTIM is synchronized with the on-chip timer
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCIN::Internal
    }
    ///External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCIN::External
    }
}
///Field `SYNCIN` writer - ynchronization input
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCIN>;
impl<'a, REG> SYNCIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disabled. HRTIM is not synchronized and runs in standalone mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::Disabled)
    }
    ///Internal event: the HRTIM is synchronized with the on-chip timer
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::Internal)
    }
    ///External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::External)
    }
}
/**Synchronization Resets Master

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRST {
    ///0: No effect on the master timer
    Disabled = 0,
    ///1: A synchroniation input event resets the master timer
    Reset = 1,
}
impl From<SYNCRST> for bool {
    #[inline(always)]
    fn from(variant: SYNCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCRST` reader - Synchronization Resets Master
pub type SYNCRST_R = crate::BitReader<SYNCRST>;
impl SYNCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCRST {
        match self.bits {
            false => SYNCRST::Disabled,
            true => SYNCRST::Reset,
        }
    }
    ///No effect on the master timer
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRST::Disabled
    }
    ///A synchroniation input event resets the master timer
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYNCRST::Reset
    }
}
///Field `SYNCRST` writer - Synchronization Resets Master
pub type SYNCRST_W<'a, REG> = crate::BitWriter<'a, REG, SYNCRST>;
impl<'a, REG> SYNCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on the master timer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRST::Disabled)
    }
    ///A synchroniation input event resets the master timer
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRST::Reset)
    }
}
/**Synchronization Starts Master

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRT {
    ///0: No effect on the master timer
    Disabled = 0,
    ///1: A synchroniation input event starts the master timer
    Start = 1,
}
impl From<SYNCSTRT> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRT) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCSTRT` reader - Synchronization Starts Master
pub type SYNCSTRT_R = crate::BitReader<SYNCSTRT>;
impl SYNCSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCSTRT {
        match self.bits {
            false => SYNCSTRT::Disabled,
            true => SYNCSTRT::Start,
        }
    }
    ///No effect on the master timer
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRT::Disabled
    }
    ///A synchroniation input event starts the master timer
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYNCSTRT::Start
    }
}
///Field `SYNCSTRT` writer - Synchronization Starts Master
pub type SYNCSTRT_W<'a, REG> = crate::BitWriter<'a, REG, SYNCSTRT>;
impl<'a, REG> SYNCSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on the master timer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRT::Disabled)
    }
    ///A synchroniation input event starts the master timer
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRT::Start)
    }
}
/**Synchronization output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT {
    ///0: Disabled
    Disabled = 0,
    ///2: Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
    PositivePulse = 2,
    ///3: Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
    NegativePulse = 3,
}
impl From<SYNCOUT> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCOUT {
    type Ux = u8;
}
impl crate::IsEnum for SYNCOUT {}
///Field `SYNCOUT` reader - Synchronization output
pub type SYNCOUT_R = crate::FieldReader<SYNCOUT>;
impl SYNCOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCOUT> {
        match self.bits {
            0 => Some(SYNCOUT::Disabled),
            2 => Some(SYNCOUT::PositivePulse),
            3 => Some(SYNCOUT::NegativePulse),
            _ => None,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT::Disabled
    }
    ///Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == SYNCOUT::PositivePulse
    }
    ///Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == SYNCOUT::NegativePulse
    }
}
///Field `SYNCOUT` writer - Synchronization output
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCOUT>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::Disabled)
    }
    ///Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::PositivePulse)
    }
    ///Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::NegativePulse)
    }
}
/**Synchronization source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC {
    ///0: Master timer Start
    MasterStart = 0,
    ///1: Master timer Compare 1 event
    MasterCompare1 = 1,
    ///2: Timer A start/reset
    TimerAstart = 2,
    ///3: Timer A Compare 1 event
    TimerAcompare1 = 3,
}
impl From<SYNCSRC> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCSRC {
    type Ux = u8;
}
impl crate::IsEnum for SYNCSRC {}
///Field `SYNCSRC` reader - Synchronization source
pub type SYNCSRC_R = crate::FieldReader<SYNCSRC>;
impl SYNCSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCSRC {
        match self.bits {
            0 => SYNCSRC::MasterStart,
            1 => SYNCSRC::MasterCompare1,
            2 => SYNCSRC::TimerAstart,
            3 => SYNCSRC::TimerAcompare1,
            _ => unreachable!(),
        }
    }
    ///Master timer Start
    #[inline(always)]
    pub fn is_master_start(&self) -> bool {
        *self == SYNCSRC::MasterStart
    }
    ///Master timer Compare 1 event
    #[inline(always)]
    pub fn is_master_compare1(&self) -> bool {
        *self == SYNCSRC::MasterCompare1
    }
    ///Timer A start/reset
    #[inline(always)]
    pub fn is_timer_astart(&self) -> bool {
        *self == SYNCSRC::TimerAstart
    }
    ///Timer A Compare 1 event
    #[inline(always)]
    pub fn is_timer_acompare1(&self) -> bool {
        *self == SYNCSRC::TimerAcompare1
    }
}
///Field `SYNCSRC` writer - Synchronization source
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCSRC, crate::Safe>;
impl<'a, REG> SYNCSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Master timer Start
    #[inline(always)]
    pub fn master_start(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::MasterStart)
    }
    ///Master timer Compare 1 event
    #[inline(always)]
    pub fn master_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::MasterCompare1)
    }
    ///Timer A start/reset
    #[inline(always)]
    pub fn timer_astart(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::TimerAstart)
    }
    ///Timer A Compare 1 event
    #[inline(always)]
    pub fn timer_acompare1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::TimerAcompare1)
    }
}
/**Master Counter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEN {
    ///0: Master timer counter disabled
    Disabled = 0,
    ///1: Master timer counter enabled
    Enabled = 1,
}
impl From<MCEN> for bool {
    #[inline(always)]
    fn from(variant: MCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MCEN` reader - Master Counter enable
pub type MCEN_R = crate::BitReader<MCEN>;
impl MCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCEN {
        match self.bits {
            false => MCEN::Disabled,
            true => MCEN::Enabled,
        }
    }
    ///Master timer counter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEN::Disabled
    }
    ///Master timer counter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEN::Enabled
    }
}
///Field `MCEN` writer - Master Counter enable
pub type MCEN_W<'a, REG> = crate::BitWriter<'a, REG, MCEN>;
impl<'a, REG> MCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEN::Disabled)
    }
    ///Master timer counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEN::Enabled)
    }
}
/**Timer %s counter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACEN {
    ///0: Timer counter disabled
    Disabled = 0,
    ///1: Timer counter enabled
    Enabled = 1,
}
impl From<TACEN> for bool {
    #[inline(always)]
    fn from(variant: TACEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCEN(A,B,C,D,E)` reader - Timer %s counter enable
pub type TCEN_R = crate::BitReader<TACEN>;
impl TCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TACEN {
        match self.bits {
            false => TACEN::Disabled,
            true => TACEN::Enabled,
        }
    }
    ///Timer counter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TACEN::Disabled
    }
    ///Timer counter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TACEN::Enabled
    }
}
///Field `TCEN(A,B,C,D,E)` writer - Timer %s counter enable
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG, TACEN>;
impl<'a, REG> TCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TACEN::Disabled)
    }
    ///Timer counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TACEN::Enabled)
    }
}
/**AC Synchronization

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACSYNC {
    ///0: No DAC trigger generated
    Disabled = 0,
    ///1: Trigger generated on DACSync1
    Dacsync1 = 1,
    ///2: Trigger generated on DACSync2
    Dacsync2 = 2,
    ///3: Trigger generated on DACSync3
    Dacsync3 = 3,
}
impl From<DACSYNC> for u8 {
    #[inline(always)]
    fn from(variant: DACSYNC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DACSYNC {
    type Ux = u8;
}
impl crate::IsEnum for DACSYNC {}
///Field `DACSYNC` reader - AC Synchronization
pub type DACSYNC_R = crate::FieldReader<DACSYNC>;
impl DACSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DACSYNC {
        match self.bits {
            0 => DACSYNC::Disabled,
            1 => DACSYNC::Dacsync1,
            2 => DACSYNC::Dacsync2,
            3 => DACSYNC::Dacsync3,
            _ => unreachable!(),
        }
    }
    ///No DAC trigger generated
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC::Disabled
    }
    ///Trigger generated on DACSync1
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC::Dacsync1
    }
    ///Trigger generated on DACSync2
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC::Dacsync2
    }
    ///Trigger generated on DACSync3
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC::Dacsync3
    }
}
///Field `DACSYNC` writer - AC Synchronization
pub type DACSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DACSYNC, crate::Safe>;
impl<'a, REG> DACSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No DAC trigger generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Disabled)
    }
    ///Trigger generated on DACSync1
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync1)
    }
    ///Trigger generated on DACSync2
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync2)
    }
    ///Trigger generated on DACSync3
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync3)
    }
}
/**Preload enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREEN {
    ///0: Preload disabled: the write access is directly done into the active register
    Disabled = 0,
    ///1: Preload enabled: the write access is done into the preload register
    Enabled = 1,
}
impl From<PREEN> for bool {
    #[inline(always)]
    fn from(variant: PREEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PREEN` reader - Preload enable
pub type PREEN_R = crate::BitReader<PREEN>;
impl PREEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PREEN {
        match self.bits {
            false => PREEN::Disabled,
            true => PREEN::Enabled,
        }
    }
    ///Preload disabled: the write access is directly done into the active register
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN::Disabled
    }
    ///Preload enabled: the write access is done into the preload register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN::Enabled
    }
}
///Field `PREEN` writer - Preload enable
pub type PREEN_W<'a, REG> = crate::BitWriter<'a, REG, PREEN>;
impl<'a, REG> PREEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload disabled: the write access is directly done into the active register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PREEN::Disabled)
    }
    ///Preload enabled: the write access is done into the preload register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PREEN::Enabled)
    }
}
/**Master Timer Repetition update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MREPU {
    ///0: Update on repetition disabled
    Disabled = 0,
    ///1: Update on repetition enabled
    Enabled = 1,
}
impl From<MREPU> for bool {
    #[inline(always)]
    fn from(variant: MREPU) -> Self {
        variant as u8 != 0
    }
}
///Field `MREPU` reader - Master Timer Repetition update
pub type MREPU_R = crate::BitReader<MREPU>;
impl MREPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MREPU {
        match self.bits {
            false => MREPU::Disabled,
            true => MREPU::Enabled,
        }
    }
    ///Update on repetition disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MREPU::Disabled
    }
    ///Update on repetition enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MREPU::Enabled
    }
}
///Field `MREPU` writer - Master Timer Repetition update
pub type MREPU_W<'a, REG> = crate::BitWriter<'a, REG, MREPU>;
impl<'a, REG> MREPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update on repetition disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MREPU::Disabled)
    }
    ///Update on repetition enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MREPU::Enabled)
    }
}
/**Burst DMA Update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRSTDMA {
    ///0: Update done independently from the DMA burst transfer completion
    Independent = 0,
    ///1: Update done when the DMA burst transfer is completed
    Completion = 1,
    ///2: Update done on master timer roll-over following a DMA burst transfer completion
    Rollover = 2,
}
impl From<BRSTDMA> for u8 {
    #[inline(always)]
    fn from(variant: BRSTDMA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BRSTDMA {
    type Ux = u8;
}
impl crate::IsEnum for BRSTDMA {}
///Field `BRSTDMA` reader - Burst DMA Update
pub type BRSTDMA_R = crate::FieldReader<BRSTDMA>;
impl BRSTDMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BRSTDMA> {
        match self.bits {
            0 => Some(BRSTDMA::Independent),
            1 => Some(BRSTDMA::Completion),
            2 => Some(BRSTDMA::Rollover),
            _ => None,
        }
    }
    ///Update done independently from the DMA burst transfer completion
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == BRSTDMA::Independent
    }
    ///Update done when the DMA burst transfer is completed
    #[inline(always)]
    pub fn is_completion(&self) -> bool {
        *self == BRSTDMA::Completion
    }
    ///Update done on master timer roll-over following a DMA burst transfer completion
    #[inline(always)]
    pub fn is_rollover(&self) -> bool {
        *self == BRSTDMA::Rollover
    }
}
///Field `BRSTDMA` writer - Burst DMA Update
pub type BRSTDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BRSTDMA>;
impl<'a, REG> BRSTDMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Update done independently from the DMA burst transfer completion
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Independent)
    }
    ///Update done when the DMA burst transfer is completed
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Completion)
    }
    ///Update done on master timer roll-over following a DMA burst transfer completion
    #[inline(always)]
    pub fn rollover(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Rollover)
    }
}
impl R {
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - ynchronization input
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    pub fn syncrst(&self) -> SYNCRST_R {
        SYNCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    pub fn syncstrt(&self) -> SYNCSTRT_R {
        SYNCSTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Timer (A,B,C,D,E) counter enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TACEN` field.</div>
    #[inline(always)]
    pub fn tcen(&self, n: u8) -> TCEN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TCEN_R::new(((self.bits >> (n + 17)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) counter enable
    #[inline(always)]
    pub fn tcen_iter(&self) -> impl Iterator<Item = TCEN_R> + '_ {
        (0..5).map(move |n| TCEN_R::new(((self.bits >> (n + 17)) & 1) != 0))
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    pub fn tacen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    pub fn tbcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    pub fn tccen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    pub fn tdcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    pub fn tecen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("brstdma", &self.brstdma())
            .field("mrepu", &self.mrepu())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("tacen", &self.tacen())
            .field("tbcen", &self.tbcen())
            .field("tccen", &self.tccen())
            .field("tdcen", &self.tdcen())
            .field("tecen", &self.tecen())
            .field("mcen", &self.mcen())
            .field("syncsrc", &self.syncsrc())
            .field("syncout", &self.syncout())
            .field("syncstrt", &self.syncstrt())
            .field("syncrst", &self.syncrst())
            .field("syncin", &self.syncin())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ckpsc", &self.ckpsc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&mut self) -> CKPSC_W<'_, CRrs> {
        CKPSC_W::new(self, 0)
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CRrs> {
        CONT_W::new(self, 3)
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W<'_, CRrs> {
        RETRIG_W::new(self, 4)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<'_, CRrs> {
        HALF_W::new(self, 5)
    }
    ///Bits 8:9 - ynchronization input
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W<'_, CRrs> {
        SYNCIN_W::new(self, 8)
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    pub fn syncrst(&mut self) -> SYNCRST_W<'_, CRrs> {
        SYNCRST_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    pub fn syncstrt(&mut self) -> SYNCSTRT_W<'_, CRrs> {
        SYNCSTRT_W::new(self, 11)
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W<'_, CRrs> {
        SYNCOUT_W::new(self, 12)
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<'_, CRrs> {
        SYNCSRC_W::new(self, 14)
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W<'_, CRrs> {
        MCEN_W::new(self, 16)
    }
    ///Timer (A,B,C,D,E) counter enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TACEN` field.</div>
    #[inline(always)]
    pub fn tcen(&mut self, n: u8) -> TCEN_W<'_, CRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TCEN_W::new(self, n + 17)
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    pub fn tacen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 17)
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    pub fn tbcen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 18)
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    pub fn tccen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 19)
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    pub fn tdcen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 20)
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    pub fn tecen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 21)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W<'_, CRrs> {
        DACSYNC_W::new(self, 25)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W<'_, CRrs> {
        PREEN_W::new(self, 27)
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    pub fn mrepu(&mut self) -> MREPU_W<'_, CRrs> {
        MREPU_W::new(self, 29)
    }
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    pub fn brstdma(&mut self) -> BRSTDMA_W<'_, CRrs> {
        BRSTDMA_W::new(self, 30)
    }
}
/**Master Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_Master:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
