///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CKPSC` reader - HRTIM Timer x Clock prescaler
pub type CKPSC_R = crate::FieldReader;
///Field `CKPSC` writer - HRTIM Timer x Clock prescaler
pub type CKPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Continuous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: The timer operates in single-shot mode and stops when it reaches the TIMxPER value
    SingleShot = 0,
    ///1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the TIMxPER value
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Continuous mode
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
    ///The timer operates in single-shot mode and stops when it reaches the TIMxPER value
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT::SingleShot
    }
    ///The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the TIMxPER value
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - Continuous mode
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The timer operates in single-shot mode and stops when it reaches the TIMxPER value
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::SingleShot)
    }
    ///The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the TIMxPER value
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**Re-triggerable mode

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
///Field `RETRIG` reader - Re-triggerable mode
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
///Field `RETRIG` writer - Re-triggerable mode
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
/**Push-Pull mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSHPLL {
    ///0: Push-pull mode disabled
    Disabled = 0,
    ///1: Push-pull mode enabled
    Enabled = 1,
}
impl From<PSHPLL> for bool {
    #[inline(always)]
    fn from(variant: PSHPLL) -> Self {
        variant as u8 != 0
    }
}
///Field `PSHPLL` reader - Push-Pull mode enable
pub type PSHPLL_R = crate::BitReader<PSHPLL>;
impl PSHPLL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSHPLL {
        match self.bits {
            false => PSHPLL::Disabled,
            true => PSHPLL::Enabled,
        }
    }
    ///Push-pull mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSHPLL::Disabled
    }
    ///Push-pull mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSHPLL::Enabled
    }
}
///Field `PSHPLL` writer - Push-Pull mode enable
pub type PSHPLL_W<'a, REG> = crate::BitWriter<'a, REG, PSHPLL>;
impl<'a, REG> PSHPLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Push-pull mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Disabled)
    }
    ///Push-pull mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Enabled)
    }
}
/**Synchronization Resets Timer x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRST {
    ///0: Synchronization event has no effect on Timer x
    Disabled = 0,
    ///1: Synchronization event resets Timer x
    Reset = 1,
}
impl From<SYNCRST> for bool {
    #[inline(always)]
    fn from(variant: SYNCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCRST` reader - Synchronization Resets Timer x
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
    ///Synchronization event has no effect on Timer x
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRST::Disabled
    }
    ///Synchronization event resets Timer x
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYNCRST::Reset
    }
}
///Field `SYNCRST` writer - Synchronization Resets Timer x
pub type SYNCRST_W<'a, REG> = crate::BitWriter<'a, REG, SYNCRST>;
impl<'a, REG> SYNCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization event has no effect on Timer x
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRST::Disabled)
    }
    ///Synchronization event resets Timer x
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRST::Reset)
    }
}
/**Synchronization Starts Timer x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRT {
    ///0: Synchronization event has no effect on Timer x
    Disabled = 0,
    ///1: Synchronization event starts Timer x
    Start = 1,
}
impl From<SYNCSTRT> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRT) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCSTRT` reader - Synchronization Starts Timer x
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
    ///Synchronization event has no effect on Timer x
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRT::Disabled
    }
    ///Synchronization event starts Timer x
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYNCSTRT::Start
    }
}
///Field `SYNCSTRT` writer - Synchronization Starts Timer x
pub type SYNCSTRT_W<'a, REG> = crate::BitWriter<'a, REG, SYNCSTRT>;
impl<'a, REG> SYNCSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization event has no effect on Timer x
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRT::Disabled)
    }
    ///Synchronization event starts Timer x
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRT::Start)
    }
}
/**Delayed CMP2 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP2 {
    ///0: CMP2 register is always active (standard compare mode)
    Standard = 0,
    ///1: CMP2 is recomputed and is active following a capture 1 event
    Capture1 = 1,
    ///2: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    Capture1Compare1 = 2,
    ///3: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    Capture1Compare3 = 3,
}
impl From<DELCMP2> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELCMP2 {
    type Ux = u8;
}
impl crate::IsEnum for DELCMP2 {}
///Field `DELCMP2` reader - Delayed CMP2 mode
pub type DELCMP2_R = crate::FieldReader<DELCMP2>;
impl DELCMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DELCMP2 {
        match self.bits {
            0 => DELCMP2::Standard,
            1 => DELCMP2::Capture1,
            2 => DELCMP2::Capture1Compare1,
            3 => DELCMP2::Capture1Compare3,
            _ => unreachable!(),
        }
    }
    ///CMP2 register is always active (standard compare mode)
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP2::Standard
    }
    ///CMP2 is recomputed and is active following a capture 1 event
    #[inline(always)]
    pub fn is_capture1(&self) -> bool {
        *self == DELCMP2::Capture1
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    #[inline(always)]
    pub fn is_capture1_compare1(&self) -> bool {
        *self == DELCMP2::Capture1Compare1
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    #[inline(always)]
    pub fn is_capture1_compare3(&self) -> bool {
        *self == DELCMP2::Capture1Compare3
    }
}
///Field `DELCMP2` writer - Delayed CMP2 mode
pub type DELCMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELCMP2, crate::Safe>;
impl<'a, REG> DELCMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMP2 register is always active (standard compare mode)
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Standard)
    }
    ///CMP2 is recomputed and is active following a capture 1 event
    #[inline(always)]
    pub fn capture1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1)
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    #[inline(always)]
    pub fn capture1_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare1)
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    #[inline(always)]
    pub fn capture1_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare3)
    }
}
/**Delayed CMP4 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP4 {
    ///0: CMP4 register is always active (standard compare mode)
    Standard = 0,
    ///1: CMP4 is recomputed and is active following a capture 2 event
    Capture2 = 1,
    ///2: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    Capture2Compare1 = 2,
    ///3: CMP4 is recomputed and is active following a capture event or a Compare 3 match
    CaptureCompare3 = 3,
}
impl From<DELCMP4> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELCMP4 {
    type Ux = u8;
}
impl crate::IsEnum for DELCMP4 {}
///Field `DELCMP4` reader - Delayed CMP4 mode
pub type DELCMP4_R = crate::FieldReader<DELCMP4>;
impl DELCMP4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DELCMP4 {
        match self.bits {
            0 => DELCMP4::Standard,
            1 => DELCMP4::Capture2,
            2 => DELCMP4::Capture2Compare1,
            3 => DELCMP4::CaptureCompare3,
            _ => unreachable!(),
        }
    }
    ///CMP4 register is always active (standard compare mode)
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP4::Standard
    }
    ///CMP4 is recomputed and is active following a capture 2 event
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == DELCMP4::Capture2
    }
    ///CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    #[inline(always)]
    pub fn is_capture2_compare1(&self) -> bool {
        *self == DELCMP4::Capture2Compare1
    }
    ///CMP4 is recomputed and is active following a capture event or a Compare 3 match
    #[inline(always)]
    pub fn is_capture_compare3(&self) -> bool {
        *self == DELCMP4::CaptureCompare3
    }
}
///Field `DELCMP4` writer - Delayed CMP4 mode
pub type DELCMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELCMP4, crate::Safe>;
impl<'a, REG> DELCMP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMP4 register is always active (standard compare mode)
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Standard)
    }
    ///CMP4 is recomputed and is active following a capture 2 event
    #[inline(always)]
    pub fn capture2(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2)
    }
    ///CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    #[inline(always)]
    pub fn capture2_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2Compare1)
    }
    ///CMP4 is recomputed and is active following a capture event or a Compare 3 match
    #[inline(always)]
    pub fn capture_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::CaptureCompare3)
    }
}
/**Timer x Repetition update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TREPU {
    ///0: Update by timer x repetition disabled
    Disabled = 0,
    ///1: Update by timer x repetition enabled
    Enabled = 1,
}
impl From<TREPU> for bool {
    #[inline(always)]
    fn from(variant: TREPU) -> Self {
        variant as u8 != 0
    }
}
///Field `TREPU` reader - Timer x Repetition update
pub type TREPU_R = crate::BitReader<TREPU>;
impl TREPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TREPU {
        match self.bits {
            false => TREPU::Disabled,
            true => TREPU::Enabled,
        }
    }
    ///Update by timer x repetition disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TREPU::Disabled
    }
    ///Update by timer x repetition enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TREPU::Enabled
    }
}
///Field `TREPU` writer - Timer x Repetition update
pub type TREPU_W<'a, REG> = crate::BitWriter<'a, REG, TREPU>;
impl<'a, REG> TREPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x repetition disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TREPU::Disabled)
    }
    ///Update by timer x repetition enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TREPU::Enabled)
    }
}
/**Timerx reset update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRSTU {
    ///0: Update by timer x reset/roll-over disabled
    Disabled = 0,
    ///1: Update by timer x reset/roll-over enabled
    Enabled = 1,
}
impl From<TRSTU> for bool {
    #[inline(always)]
    fn from(variant: TRSTU) -> Self {
        variant as u8 != 0
    }
}
///Field `TRSTU` reader - Timerx reset update
pub type TRSTU_R = crate::BitReader<TRSTU>;
impl TRSTU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRSTU {
        match self.bits {
            false => TRSTU::Disabled,
            true => TRSTU::Enabled,
        }
    }
    ///Update by timer x reset/roll-over disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRSTU::Disabled
    }
    ///Update by timer x reset/roll-over enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRSTU::Enabled
    }
}
///Field `TRSTU` writer - Timerx reset update
pub type TRSTU_W<'a, REG> = crate::BitWriter<'a, REG, TRSTU>;
impl<'a, REG> TRSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x reset/roll-over disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTU::Disabled)
    }
    ///Update by timer x reset/roll-over enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTU::Enabled)
    }
}
/**TBU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBU {
    ///0: Update by timer x disabled
    Disabled = 0,
    ///1: Update by timer x enabled
    Enabled = 1,
}
impl From<TBU> for bool {
    #[inline(always)]
    fn from(variant: TBU) -> Self {
        variant as u8 != 0
    }
}
///Field `TBU` reader - TBU
pub type TBU_R = crate::BitReader<TBU>;
impl TBU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TBU {
        match self.bits {
            false => TBU::Disabled,
            true => TBU::Enabled,
        }
    }
    ///Update by timer x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBU::Disabled
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBU::Enabled
    }
}
///Field `TBU` writer - TBU
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG, TBU>;
impl<'a, REG> TBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Disabled)
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Enabled)
    }
}
///Field `TCU` reader - TCU
pub use TBU_R as TCU_R;
///Field `TDU` reader - TDU
pub use TBU_R as TDU_R;
///Field `TEU` reader - TEU
pub use TBU_R as TEU_R;
///Field `TCU` writer - TCU
pub use TBU_W as TCU_W;
///Field `TDU` writer - TDU
pub use TBU_W as TDU_W;
///Field `TEU` writer - TEU
pub use TBU_W as TEU_W;
/**Master Timer update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTU {
    ///0: Update by master timer disabled
    Disabled = 0,
    ///1: Update by master timer enabled
    Enabled = 1,
}
impl From<MSTU> for bool {
    #[inline(always)]
    fn from(variant: MSTU) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTU` reader - Master Timer update
pub type MSTU_R = crate::BitReader<MSTU>;
impl MSTU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTU {
        match self.bits {
            false => MSTU::Disabled,
            true => MSTU::Enabled,
        }
    }
    ///Update by master timer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTU::Disabled
    }
    ///Update by master timer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTU::Enabled
    }
}
///Field `MSTU` writer - Master Timer update
pub type MSTU_W<'a, REG> = crate::BitWriter<'a, REG, MSTU>;
impl<'a, REG> MSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by master timer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Disabled)
    }
    ///Update by master timer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Enabled)
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
/**Update Gating

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDGAT {
    ///0: Update occurs independently from the DMA burst transfer
    Independent = 0,
    ///1: Update occurs when the DMA burst transfer is completed
    Dmaburst = 1,
    ///2: Update occurs on the update event following DMA burst transfer completion
    DmaburstUpdate = 2,
    ///3: Update occurs on a rising edge of HRTIM update enable input 1
    Input1 = 3,
    ///4: Update occurs on a rising edge of HRTIM update enable input 2
    Input2 = 4,
    ///5: Update occurs on a rising edge of HRTIM update enable input 3
    Input3 = 5,
    ///6: Update occurs on the update event following a rising edge of HRTIM update enable input 1
    Input1Update = 6,
    ///7: Update occurs on the update event following a rising edge of HRTIM update enable input 2
    Input2Update = 7,
    ///8: Update occurs on the update event following a rising edge of HRTIM update enable input 3
    Input3Update = 8,
}
impl From<UPDGAT> for u8 {
    #[inline(always)]
    fn from(variant: UPDGAT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPDGAT {
    type Ux = u8;
}
impl crate::IsEnum for UPDGAT {}
///Field `UPDGAT` reader - Update Gating
pub type UPDGAT_R = crate::FieldReader<UPDGAT>;
impl UPDGAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPDGAT> {
        match self.bits {
            0 => Some(UPDGAT::Independent),
            1 => Some(UPDGAT::Dmaburst),
            2 => Some(UPDGAT::DmaburstUpdate),
            3 => Some(UPDGAT::Input1),
            4 => Some(UPDGAT::Input2),
            5 => Some(UPDGAT::Input3),
            6 => Some(UPDGAT::Input1Update),
            7 => Some(UPDGAT::Input2Update),
            8 => Some(UPDGAT::Input3Update),
            _ => None,
        }
    }
    ///Update occurs independently from the DMA burst transfer
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == UPDGAT::Independent
    }
    ///Update occurs when the DMA burst transfer is completed
    #[inline(always)]
    pub fn is_dmaburst(&self) -> bool {
        *self == UPDGAT::Dmaburst
    }
    ///Update occurs on the update event following DMA burst transfer completion
    #[inline(always)]
    pub fn is_dmaburst_update(&self) -> bool {
        *self == UPDGAT::DmaburstUpdate
    }
    ///Update occurs on a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == UPDGAT::Input1
    }
    ///Update occurs on a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == UPDGAT::Input2
    }
    ///Update occurs on a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == UPDGAT::Input3
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn is_input1_update(&self) -> bool {
        *self == UPDGAT::Input1Update
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn is_input2_update(&self) -> bool {
        *self == UPDGAT::Input2Update
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn is_input3_update(&self) -> bool {
        *self == UPDGAT::Input3Update
    }
}
///Field `UPDGAT` writer - Update Gating
pub type UPDGAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UPDGAT>;
impl<'a, REG> UPDGAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Update occurs independently from the DMA burst transfer
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Independent)
    }
    ///Update occurs when the DMA burst transfer is completed
    #[inline(always)]
    pub fn dmaburst(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Dmaburst)
    }
    ///Update occurs on the update event following DMA burst transfer completion
    #[inline(always)]
    pub fn dmaburst_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::DmaburstUpdate)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn input1_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1Update)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn input2_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2Update)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn input3_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3Update)
    }
}
impl R {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrst(&self) -> SYNCRST_R {
        SYNCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&self) -> SYNCSTRT_R {
        SYNCSTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn trepu(&self) -> TREPU_R {
        TREPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&self) -> TRSTU_R {
        TRSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
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
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("updgat", &self.updgat())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("mstu", &self.mstu())
            .field("tbu", &self.tbu())
            .field("teu", &self.teu())
            .field("tdu", &self.tdu())
            .field("tcu", &self.tcu())
            .field("trstu", &self.trstu())
            .field("trepu", &self.trepu())
            .field("delcmp4", &self.delcmp4())
            .field("delcmp2", &self.delcmp2())
            .field("syncstrt", &self.syncstrt())
            .field("syncrst", &self.syncrst())
            .field("pshpll", &self.pshpll())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ckpsc", &self.ckpsc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&mut self) -> CKPSC_W<CRrs> {
        CKPSC_W::new(self, 0)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<CRrs> {
        CONT_W::new(self, 3)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W<CRrs> {
        RETRIG_W::new(self, 4)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<CRrs> {
        HALF_W::new(self, 5)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&mut self) -> PSHPLL_W<CRrs> {
        PSHPLL_W::new(self, 6)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrst(&mut self) -> SYNCRST_W<CRrs> {
        SYNCRST_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&mut self) -> SYNCSTRT_W<CRrs> {
        SYNCSTRT_W::new(self, 11)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&mut self) -> DELCMP2_W<CRrs> {
        DELCMP2_W::new(self, 12)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&mut self) -> DELCMP4_W<CRrs> {
        DELCMP4_W::new(self, 14)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn trepu(&mut self) -> TREPU_W<CRrs> {
        TREPU_W::new(self, 17)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&mut self) -> TRSTU_W<CRrs> {
        TRSTU_W::new(self, 18)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<CRrs> {
        TBU_W::new(self, 20)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&mut self) -> TCU_W<CRrs> {
        TCU_W::new(self, 21)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&mut self) -> TDU_W<CRrs> {
        TDU_W::new(self, 22)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&mut self) -> TEU_W<CRrs> {
        TEU_W::new(self, 23)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&mut self) -> MSTU_W<CRrs> {
        MSTU_W::new(self, 24)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W<CRrs> {
        DACSYNC_W::new(self, 25)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W<CRrs> {
        PREEN_W::new(self, 27)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&mut self) -> UPDGAT_W<CRrs> {
        UPDGAT_W::new(self, 28)
    }
}
/**Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#HRTIM_TIMA:CR)*/
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
