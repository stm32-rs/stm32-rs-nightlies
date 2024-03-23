#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "Field `CKPSC` reader - HRTIM Master Clock prescaler"]
pub type CKPSC_R = crate::FieldReader;
#[doc = "Field `CKPSC` writer - HRTIM Master Clock prescaler"]
pub type CKPSC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Master Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    #[doc = "0: The timer operates in single-shot mode and stops when it reaches the MPER value"]
    SingleShot = 0,
    #[doc = "1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Master Continuous mode"]
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::SingleShot,
            true => CONT::Continuous,
        }
    }
    #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT::SingleShot
    }
    #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
#[doc = "Field `CONT` writer - Master Continuous mode"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::SingleShot)
    }
    #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
#[doc = "Master Re-triggerable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETRIG {
    #[doc = "0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    Disabled = 0,
    #[doc = "1: The timer is retriggerable: a counter reset is done whatever the counter state"]
    Enabled = 1,
}
impl From<RETRIG> for bool {
    #[inline(always)]
    fn from(variant: RETRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETRIG` reader - Master Re-triggerable mode"]
pub type RETRIG_R = crate::BitReader<RETRIG>;
impl RETRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RETRIG {
        match self.bits {
            false => RETRIG::Disabled,
            true => RETRIG::Enabled,
        }
    }
    #[doc = "The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG::Disabled
    }
    #[doc = "The timer is retriggerable: a counter reset is done whatever the counter state"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG::Enabled
    }
}
#[doc = "Field `RETRIG` writer - Master Re-triggerable mode"]
pub type RETRIG_W<'a, REG> = crate::BitWriter<'a, REG, RETRIG>;
impl<'a, REG> RETRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETRIG::Disabled)
    }
    #[doc = "The timer is retriggerable: a counter reset is done whatever the counter state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RETRIG::Enabled)
    }
}
#[doc = "Half mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF {
    #[doc = "0: Half mode disabled"]
    Disabled = 0,
    #[doc = "1: Half mode enabled"]
    Enabled = 1,
}
impl From<HALF> for bool {
    #[inline(always)]
    fn from(variant: HALF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HALF_R = crate::BitReader<HALF>;
impl HALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HALF {
        match self.bits {
            false => HALF::Disabled,
            true => HALF::Enabled,
        }
    }
    #[doc = "Half mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF::Disabled
    }
    #[doc = "Half mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF::Enabled
    }
}
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HALF_W<'a, REG> = crate::BitWriter<'a, REG, HALF>;
impl<'a, REG> HALF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALF::Disabled)
    }
    #[doc = "Half mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALF::Enabled)
    }
}
#[doc = "ynchronization input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCIN {
    #[doc = "0: Disabled. HRTIM is not synchronized and runs in standalone mode"]
    Disabled = 0,
    #[doc = "2: Internal event: the HRTIM is synchronized with the on-chip timer"]
    Internal = 2,
    #[doc = "3: External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
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
#[doc = "Field `SYNCIN` reader - ynchronization input"]
pub type SYNCIN_R = crate::FieldReader<SYNCIN>;
impl SYNCIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCIN> {
        match self.bits {
            0 => Some(SYNCIN::Disabled),
            2 => Some(SYNCIN::Internal),
            3 => Some(SYNCIN::External),
            _ => None,
        }
    }
    #[doc = "Disabled. HRTIM is not synchronized and runs in standalone mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCIN::Disabled
    }
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCIN::Internal
    }
    #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCIN::External
    }
}
#[doc = "Field `SYNCIN` writer - ynchronization input"]
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCIN>;
impl<'a, REG> SYNCIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled. HRTIM is not synchronized and runs in standalone mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::Disabled)
    }
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::Internal)
    }
    #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCIN::External)
    }
}
#[doc = "Synchronization Resets Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRSTM {
    #[doc = "0: No effect on the master timer"]
    Disabled = 0,
    #[doc = "1: A synchroniation input event resets the master timer"]
    Enabled = 1,
}
impl From<SYNCRSTM> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCRSTM` reader - Synchronization Resets Master"]
pub type SYNCRSTM_R = crate::BitReader<SYNCRSTM>;
impl SYNCRSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCRSTM {
        match self.bits {
            false => SYNCRSTM::Disabled,
            true => SYNCRSTM::Enabled,
        }
    }
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTM::Disabled
    }
    #[doc = "A synchroniation input event resets the master timer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCRSTM::Enabled
    }
}
#[doc = "Field `SYNCRSTM` writer - Synchronization Resets Master"]
pub type SYNCRSTM_W<'a, REG> = crate::BitWriter<'a, REG, SYNCRSTM>;
impl<'a, REG> SYNCRSTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRSTM::Disabled)
    }
    #[doc = "A synchroniation input event resets the master timer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRSTM::Enabled)
    }
}
#[doc = "Synchronization Starts Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRTM {
    #[doc = "0: No effect on the master timer"]
    Disabled = 0,
    #[doc = "1: A synchroniation input event starts the master timer"]
    Enabled = 1,
}
impl From<SYNCSTRTM> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCSTRTM` reader - Synchronization Starts Master"]
pub type SYNCSTRTM_R = crate::BitReader<SYNCSTRTM>;
impl SYNCSTRTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCSTRTM {
        match self.bits {
            false => SYNCSTRTM::Disabled,
            true => SYNCSTRTM::Enabled,
        }
    }
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTM::Disabled
    }
    #[doc = "A synchroniation input event starts the master timer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCSTRTM::Enabled
    }
}
#[doc = "Field `SYNCSTRTM` writer - Synchronization Starts Master"]
pub type SYNCSTRTM_W<'a, REG> = crate::BitWriter<'a, REG, SYNCSTRTM>;
impl<'a, REG> SYNCSTRTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRTM::Disabled)
    }
    #[doc = "A synchroniation input event starts the master timer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRTM::Enabled)
    }
}
#[doc = "Synchronization output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "2: Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    PositivePulse = 2,
    #[doc = "3: Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
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
#[doc = "Field `SYNCOUT` reader - Synchronization output"]
pub type SYNCOUT_R = crate::FieldReader<SYNCOUT>;
impl SYNCOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCOUT> {
        match self.bits {
            0 => Some(SYNCOUT::Disabled),
            2 => Some(SYNCOUT::PositivePulse),
            3 => Some(SYNCOUT::NegativePulse),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT::Disabled
    }
    #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == SYNCOUT::PositivePulse
    }
    #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == SYNCOUT::NegativePulse
    }
}
#[doc = "Field `SYNCOUT` writer - Synchronization output"]
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCOUT>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::Disabled)
    }
    #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::PositivePulse)
    }
    #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT::NegativePulse)
    }
}
#[doc = "Synchronization source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC {
    #[doc = "0: Master timer Start"]
    MasterStart = 0,
    #[doc = "1: Master timer Compare 1 event"]
    MasterCompare1 = 1,
    #[doc = "2: Timer A start/reset"]
    TimerAstart = 2,
    #[doc = "3: Timer A Compare 1 event"]
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
#[doc = "Field `SYNCSRC` reader - Synchronization source"]
pub type SYNCSRC_R = crate::FieldReader<SYNCSRC>;
impl SYNCSRC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Master timer Start"]
    #[inline(always)]
    pub fn is_master_start(&self) -> bool {
        *self == SYNCSRC::MasterStart
    }
    #[doc = "Master timer Compare 1 event"]
    #[inline(always)]
    pub fn is_master_compare1(&self) -> bool {
        *self == SYNCSRC::MasterCompare1
    }
    #[doc = "Timer A start/reset"]
    #[inline(always)]
    pub fn is_timer_astart(&self) -> bool {
        *self == SYNCSRC::TimerAstart
    }
    #[doc = "Timer A Compare 1 event"]
    #[inline(always)]
    pub fn is_timer_acompare1(&self) -> bool {
        *self == SYNCSRC::TimerAcompare1
    }
}
#[doc = "Field `SYNCSRC` writer - Synchronization source"]
pub type SYNCSRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SYNCSRC>;
impl<'a, REG> SYNCSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master timer Start"]
    #[inline(always)]
    pub fn master_start(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::MasterStart)
    }
    #[doc = "Master timer Compare 1 event"]
    #[inline(always)]
    pub fn master_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::MasterCompare1)
    }
    #[doc = "Timer A start/reset"]
    #[inline(always)]
    pub fn timer_astart(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::TimerAstart)
    }
    #[doc = "Timer A Compare 1 event"]
    #[inline(always)]
    pub fn timer_acompare1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::TimerAcompare1)
    }
}
#[doc = "Master Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEN {
    #[doc = "0: Master timer counter disabled"]
    Disabled = 0,
    #[doc = "1: Master timer counter enabled"]
    Enabled = 1,
}
impl From<MCEN> for bool {
    #[inline(always)]
    fn from(variant: MCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCEN` reader - Master Counter enable"]
pub type MCEN_R = crate::BitReader<MCEN>;
impl MCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCEN {
        match self.bits {
            false => MCEN::Disabled,
            true => MCEN::Enabled,
        }
    }
    #[doc = "Master timer counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEN::Disabled
    }
    #[doc = "Master timer counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEN::Enabled
    }
}
#[doc = "Field `MCEN` writer - Master Counter enable"]
pub type MCEN_W<'a, REG> = crate::BitWriter<'a, REG, MCEN>;
impl<'a, REG> MCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEN::Disabled)
    }
    #[doc = "Master timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MCEN::Enabled)
    }
}
#[doc = "Timer A counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACEN {
    #[doc = "0: Timer counter disabled"]
    Disabled = 0,
    #[doc = "1: Timer counter enabled"]
    Enabled = 1,
}
impl From<TACEN> for bool {
    #[inline(always)]
    fn from(variant: TACEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACEN` reader - Timer A counter enable"]
pub type TACEN_R = crate::BitReader<TACEN>;
impl TACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TACEN {
        match self.bits {
            false => TACEN::Disabled,
            true => TACEN::Enabled,
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TACEN::Disabled
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TACEN::Enabled
    }
}
#[doc = "Field `TACEN` writer - Timer A counter enable"]
pub type TACEN_W<'a, REG> = crate::BitWriter<'a, REG, TACEN>;
impl<'a, REG> TACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TACEN::Disabled)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TACEN::Enabled)
    }
}
#[doc = "Field `TBCEN` reader - Timer B counter enable"]
pub use TACEN_R as TBCEN_R;
#[doc = "Field `TCCEN` reader - Timer C counter enable"]
pub use TACEN_R as TCCEN_R;
#[doc = "Field `TDCEN` reader - Timer D counter enable"]
pub use TACEN_R as TDCEN_R;
#[doc = "Field `TECEN` reader - Timer E counter enable"]
pub use TACEN_R as TECEN_R;
#[doc = "Field `TBCEN` writer - Timer B counter enable"]
pub use TACEN_W as TBCEN_W;
#[doc = "Field `TCCEN` writer - Timer C counter enable"]
pub use TACEN_W as TCCEN_W;
#[doc = "Field `TDCEN` writer - Timer D counter enable"]
pub use TACEN_W as TDCEN_W;
#[doc = "Field `TECEN` writer - Timer E counter enable"]
pub use TACEN_W as TECEN_W;
#[doc = "AC Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACSYNC {
    #[doc = "0: No DAC trigger generated"]
    Disabled = 0,
    #[doc = "1: Trigger generated on DACSync1"]
    Dacsync1 = 1,
    #[doc = "2: Trigger generated on DACSync2"]
    Dacsync2 = 2,
    #[doc = "3: Trigger generated on DACSync3"]
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
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DACSYNC_R = crate::FieldReader<DACSYNC>;
impl DACSYNC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No DAC trigger generated"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC::Disabled
    }
    #[doc = "Trigger generated on DACSync1"]
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC::Dacsync1
    }
    #[doc = "Trigger generated on DACSync2"]
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC::Dacsync2
    }
    #[doc = "Trigger generated on DACSync3"]
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC::Dacsync3
    }
}
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DACSYNC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DACSYNC>;
impl<'a, REG> DACSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DAC trigger generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Disabled)
    }
    #[doc = "Trigger generated on DACSync1"]
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync1)
    }
    #[doc = "Trigger generated on DACSync2"]
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync2)
    }
    #[doc = "Trigger generated on DACSync3"]
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut crate::W<REG> {
        self.variant(DACSYNC::Dacsync3)
    }
}
#[doc = "Preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREEN {
    #[doc = "0: Preload disabled: the write access is directly done into the active register"]
    Disabled = 0,
    #[doc = "1: Preload enabled: the write access is done into the preload register"]
    Enabled = 1,
}
impl From<PREEN> for bool {
    #[inline(always)]
    fn from(variant: PREEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PREEN_R = crate::BitReader<PREEN>;
impl PREEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PREEN {
        match self.bits {
            false => PREEN::Disabled,
            true => PREEN::Enabled,
        }
    }
    #[doc = "Preload disabled: the write access is directly done into the active register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN::Disabled
    }
    #[doc = "Preload enabled: the write access is done into the preload register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN::Enabled
    }
}
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PREEN_W<'a, REG> = crate::BitWriter<'a, REG, PREEN>;
impl<'a, REG> PREEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload disabled: the write access is directly done into the active register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PREEN::Disabled)
    }
    #[doc = "Preload enabled: the write access is done into the preload register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PREEN::Enabled)
    }
}
#[doc = "Master Timer Repetition update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MREPU {
    #[doc = "0: Update on repetition disabled"]
    Disabled = 0,
    #[doc = "1: Update on repetition enabled"]
    Enabled = 1,
}
impl From<MREPU> for bool {
    #[inline(always)]
    fn from(variant: MREPU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MREPU` reader - Master Timer Repetition update"]
pub type MREPU_R = crate::BitReader<MREPU>;
impl MREPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MREPU {
        match self.bits {
            false => MREPU::Disabled,
            true => MREPU::Enabled,
        }
    }
    #[doc = "Update on repetition disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MREPU::Disabled
    }
    #[doc = "Update on repetition enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MREPU::Enabled
    }
}
#[doc = "Field `MREPU` writer - Master Timer Repetition update"]
pub type MREPU_W<'a, REG> = crate::BitWriter<'a, REG, MREPU>;
impl<'a, REG> MREPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update on repetition disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MREPU::Disabled)
    }
    #[doc = "Update on repetition enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MREPU::Enabled)
    }
}
#[doc = "Burst DMA Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRSTDMA {
    #[doc = "0: Update done independently from the DMA burst transfer completion"]
    Independent = 0,
    #[doc = "1: Update done when the DMA burst transfer is completed"]
    Completion = 1,
    #[doc = "2: Update done on master timer roll-over following a DMA burst transfer completion"]
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
#[doc = "Field `BRSTDMA` reader - Burst DMA Update"]
pub type BRSTDMA_R = crate::FieldReader<BRSTDMA>;
impl BRSTDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BRSTDMA> {
        match self.bits {
            0 => Some(BRSTDMA::Independent),
            1 => Some(BRSTDMA::Completion),
            2 => Some(BRSTDMA::Rollover),
            _ => None,
        }
    }
    #[doc = "Update done independently from the DMA burst transfer completion"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == BRSTDMA::Independent
    }
    #[doc = "Update done when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn is_completion(&self) -> bool {
        *self == BRSTDMA::Completion
    }
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
    #[inline(always)]
    pub fn is_rollover(&self) -> bool {
        *self == BRSTDMA::Rollover
    }
}
#[doc = "Field `BRSTDMA` writer - Burst DMA Update"]
pub type BRSTDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BRSTDMA>;
impl<'a, REG> BRSTDMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Update done independently from the DMA burst transfer completion"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Independent)
    }
    #[doc = "Update done when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Completion)
    }
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
    #[inline(always)]
    pub fn rollover(self) -> &'a mut crate::W<REG> {
        self.variant(BRSTDMA::Rollover)
    }
}
impl R {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ckpsc(&mut self) -> CKPSC_W<MCRrs> {
        CKPSC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<MCRrs> {
        CONT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<MCRrs> {
        RETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<MCRrs> {
        HALF_W::new(self, 5)
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<MCRrs> {
        SYNCIN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W<MCRrs> {
        SYNCRSTM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W<MCRrs> {
        SYNCSTRTM_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<MCRrs> {
        SYNCOUT_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<MCRrs> {
        SYNCSRC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcen(&mut self) -> MCEN_W<MCRrs> {
        MCEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tacen(&mut self) -> TACEN_W<MCRrs> {
        TACEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbcen(&mut self) -> TBCEN_W<MCRrs> {
        TBCEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tccen(&mut self) -> TCCEN_W<MCRrs> {
        TCCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcen(&mut self) -> TDCEN_W<MCRrs> {
        TDCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tecen(&mut self) -> TECEN_W<MCRrs> {
        TECEN_W::new(self, 21)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<MCRrs> {
        DACSYNC_W::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<MCRrs> {
        PREEN_W::new(self, 27)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn mrepu(&mut self) -> MREPU_W<MCRrs> {
        MREPU_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    #[must_use]
    pub fn brstdma(&mut self) -> BRSTDMA_W<MCRrs> {
        BRSTDMA_W::new(self, 30)
    }
}
#[doc = "Master Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCRrs {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0;
}
