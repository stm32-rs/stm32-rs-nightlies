#[doc = "Register `TIMBCR` reader"]
pub type R = crate::R<TIMBCRrs>;
#[doc = "Register `TIMBCR` writer"]
pub type W = crate::W<TIMBCRrs>;
#[doc = "Field `CKPSCx` reader - HRTIM Timer x Clock prescaler"]
pub type CKPSCX_R = crate::FieldReader;
#[doc = "Field `CKPSCx` writer - HRTIM Timer x Clock prescaler"]
pub type CKPSCX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Continuous mode\n\nValue on reset: 0"]
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
#[doc = "Field `CONT` reader - Continuous mode"]
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
#[doc = "Field `CONT` writer - Continuous mode"]
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
#[doc = "Re-triggerable mode\n\nValue on reset: 0"]
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
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
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
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
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
#[doc = "Push-Pull mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSHPLL {
    #[doc = "0: Push-pull mode disabled"]
    Disabled = 0,
    #[doc = "1: Push-pull mode enabled"]
    Enabled = 1,
}
impl From<PSHPLL> for bool {
    #[inline(always)]
    fn from(variant: PSHPLL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub type PSHPLL_R = crate::BitReader<PSHPLL>;
impl PSHPLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSHPLL {
        match self.bits {
            false => PSHPLL::Disabled,
            true => PSHPLL::Enabled,
        }
    }
    #[doc = "Push-pull mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSHPLL::Disabled
    }
    #[doc = "Push-pull mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSHPLL::Enabled
    }
}
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub type PSHPLL_W<'a, REG> = crate::BitWriter<'a, REG, PSHPLL>;
impl<'a, REG> PSHPLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push-pull mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Disabled)
    }
    #[doc = "Push-pull mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Enabled)
    }
}
#[doc = "Synchronization Resets Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRSTX {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    Disabled = 0,
    #[doc = "1: Synchronization event resets Timer x"]
    Reset = 1,
}
impl From<SYNCRSTX> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub type SYNCRSTX_R = crate::BitReader<SYNCRSTX>;
impl SYNCRSTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCRSTX {
        match self.bits {
            false => SYNCRSTX::Disabled,
            true => SYNCRSTX::Reset,
        }
    }
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTX::Disabled
    }
    #[doc = "Synchronization event resets Timer x"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYNCRSTX::Reset
    }
}
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub type SYNCRSTX_W<'a, REG> = crate::BitWriter<'a, REG, SYNCRSTX>;
impl<'a, REG> SYNCRSTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRSTX::Disabled)
    }
    #[doc = "Synchronization event resets Timer x"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCRSTX::Reset)
    }
}
#[doc = "Synchronization Starts Timer x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRTX {
    #[doc = "0: Synchronization event has no effect on Timer x"]
    Disabled = 0,
    #[doc = "1: Synchronization event starts Timer x"]
    Start = 1,
}
impl From<SYNCSTRTX> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub type SYNCSTRTX_R = crate::BitReader<SYNCSTRTX>;
impl SYNCSTRTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCSTRTX {
        match self.bits {
            false => SYNCSTRTX::Disabled,
            true => SYNCSTRTX::Start,
        }
    }
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTX::Disabled
    }
    #[doc = "Synchronization event starts Timer x"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYNCSTRTX::Start
    }
}
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub type SYNCSTRTX_W<'a, REG> = crate::BitWriter<'a, REG, SYNCSTRTX>;
impl<'a, REG> SYNCSTRTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization event has no effect on Timer x"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRTX::Disabled)
    }
    #[doc = "Synchronization event starts Timer x"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSTRTX::Start)
    }
}
#[doc = "Delayed CMP2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP2 {
    #[doc = "0: CMP2 register is always active (standard compare mode)"]
    Standard = 0,
    #[doc = "1: CMP2 is recomputed and is active following a capture 1 event"]
    Capture1 = 1,
    #[doc = "2: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    Capture1Compare1 = 2,
    #[doc = "3: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
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
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub type DELCMP2_R = crate::FieldReader<DELCMP2>;
impl DELCMP2_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "CMP2 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP2::Standard
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event"]
    #[inline(always)]
    pub fn is_capture1(&self) -> bool {
        *self == DELCMP2::Capture1
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    #[inline(always)]
    pub fn is_capture1_compare1(&self) -> bool {
        *self == DELCMP2::Capture1Compare1
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    #[inline(always)]
    pub fn is_capture1_compare3(&self) -> bool {
        *self == DELCMP2::Capture1Compare3
    }
}
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub type DELCMP2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DELCMP2>;
impl<'a, REG> DELCMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP2 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Standard)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event"]
    #[inline(always)]
    pub fn capture1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture1_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare1)
    }
    #[doc = "CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture1_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare3)
    }
}
#[doc = "Delayed CMP4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP4 {
    #[doc = "0: CMP4 register is always active (standard compare mode)"]
    Standard = 0,
    #[doc = "1: CMP4 is recomputed and is active following a capture 2 event"]
    Capture2 = 1,
    #[doc = "2: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    Capture2Compare1 = 2,
    #[doc = "3: CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
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
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub type DELCMP4_R = crate::FieldReader<DELCMP4>;
impl DELCMP4_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP4::Standard
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event"]
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == DELCMP4::Capture2
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    #[inline(always)]
    pub fn is_capture2_compare1(&self) -> bool {
        *self == DELCMP4::Capture2Compare1
    }
    #[doc = "CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    #[inline(always)]
    pub fn is_capture_compare3(&self) -> bool {
        *self == DELCMP4::CaptureCompare3
    }
}
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub type DELCMP4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DELCMP4>;
impl<'a, REG> DELCMP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP4 register is always active (standard compare mode)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Standard)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event"]
    #[inline(always)]
    pub fn capture2(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2)
    }
    #[doc = "CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match"]
    #[inline(always)]
    pub fn capture2_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2Compare1)
    }
    #[doc = "CMP4 is recomputed and is active following a capture event or a Compare 3 match"]
    #[inline(always)]
    pub fn capture_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::CaptureCompare3)
    }
}
#[doc = "Timer x Repetition update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_REPU {
    #[doc = "0: Update by timer x repetition disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x repetition enabled"]
    Enabled = 1,
}
impl From<TX_REPU> for bool {
    #[inline(always)]
    fn from(variant: TX_REPU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub type TX_REPU_R = crate::BitReader<TX_REPU>;
impl TX_REPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_REPU {
        match self.bits {
            false => TX_REPU::Disabled,
            true => TX_REPU::Enabled,
        }
    }
    #[doc = "Update by timer x repetition disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_REPU::Disabled
    }
    #[doc = "Update by timer x repetition enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_REPU::Enabled
    }
}
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub type TX_REPU_W<'a, REG> = crate::BitWriter<'a, REG, TX_REPU>;
impl<'a, REG> TX_REPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update by timer x repetition disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_REPU::Disabled)
    }
    #[doc = "Update by timer x repetition enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_REPU::Enabled)
    }
}
#[doc = "Timerx reset update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_RSTU {
    #[doc = "0: Update by timer x reset/roll-over disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x reset/roll-over enabled"]
    Enabled = 1,
}
impl From<TX_RSTU> for bool {
    #[inline(always)]
    fn from(variant: TX_RSTU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub type TX_RSTU_R = crate::BitReader<TX_RSTU>;
impl TX_RSTU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_RSTU {
        match self.bits {
            false => TX_RSTU::Disabled,
            true => TX_RSTU::Enabled,
        }
    }
    #[doc = "Update by timer x reset/roll-over disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_RSTU::Disabled
    }
    #[doc = "Update by timer x reset/roll-over enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_RSTU::Enabled
    }
}
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub type TX_RSTU_W<'a, REG> = crate::BitWriter<'a, REG, TX_RSTU>;
impl<'a, REG> TX_RSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update by timer x reset/roll-over disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_RSTU::Disabled)
    }
    #[doc = "Update by timer x reset/roll-over enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_RSTU::Enabled)
    }
}
#[doc = "TBU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBU {
    #[doc = "0: Update by timer x disabled"]
    Disabled = 0,
    #[doc = "1: Update by timer x enabled"]
    Enabled = 1,
}
impl From<TBU> for bool {
    #[inline(always)]
    fn from(variant: TBU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBU` reader - TBU"]
pub type TBU_R = crate::BitReader<TBU>;
impl TBU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBU {
        match self.bits {
            false => TBU::Disabled,
            true => TBU::Enabled,
        }
    }
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBU::Disabled
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBU::Enabled
    }
}
#[doc = "Field `TBU` writer - TBU"]
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG, TBU>;
impl<'a, REG> TBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update by timer x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Disabled)
    }
    #[doc = "Update by timer x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Enabled)
    }
}
#[doc = "Field `TCU` reader - TCU"]
pub use TBU_R as TCU_R;
#[doc = "Field `TDU` reader - TDU"]
pub use TBU_R as TDU_R;
#[doc = "Field `TEU` reader - TEU"]
pub use TBU_R as TEU_R;
#[doc = "Field `TCU` writer - TCU"]
pub use TBU_W as TCU_W;
#[doc = "Field `TDU` writer - TDU"]
pub use TBU_W as TDU_W;
#[doc = "Field `TEU` writer - TEU"]
pub use TBU_W as TEU_W;
#[doc = "Master Timer update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTU {
    #[doc = "0: Update by master timer disabled"]
    Disabled = 0,
    #[doc = "1: Update by master timer enabled"]
    Enabled = 1,
}
impl From<MSTU> for bool {
    #[inline(always)]
    fn from(variant: MSTU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTU` reader - Master Timer update"]
pub type MSTU_R = crate::BitReader<MSTU>;
impl MSTU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTU {
        match self.bits {
            false => MSTU::Disabled,
            true => MSTU::Enabled,
        }
    }
    #[doc = "Update by master timer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTU::Disabled
    }
    #[doc = "Update by master timer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTU::Enabled
    }
}
#[doc = "Field `MSTU` writer - Master Timer update"]
pub type MSTU_W<'a, REG> = crate::BitWriter<'a, REG, MSTU>;
impl<'a, REG> MSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update by master timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Disabled)
    }
    #[doc = "Update by master timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Enabled)
    }
}
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
#[doc = "Update Gating\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDGAT {
    #[doc = "0: Update occurs independently from the DMA burst transfer"]
    Independent = 0,
    #[doc = "1: Update occurs when the DMA burst transfer is completed"]
    Dmaburst = 1,
    #[doc = "2: Update occurs on the update event following DMA burst transfer completion"]
    DmaburstUpdate = 2,
    #[doc = "3: Update occurs on a rising edge of HRTIM update enable input 1"]
    Input1 = 3,
    #[doc = "4: Update occurs on a rising edge of HRTIM update enable input 2"]
    Input2 = 4,
    #[doc = "5: Update occurs on a rising edge of HRTIM update enable input 3"]
    Input3 = 5,
    #[doc = "6: Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    Input1Update = 6,
    #[doc = "7: Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    Input2Update = 7,
    #[doc = "8: Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
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
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub type UPDGAT_R = crate::FieldReader<UPDGAT>;
impl UPDGAT_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Update occurs independently from the DMA burst transfer"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == UPDGAT::Independent
    }
    #[doc = "Update occurs when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn is_dmaburst(&self) -> bool {
        *self == UPDGAT::Dmaburst
    }
    #[doc = "Update occurs on the update event following DMA burst transfer completion"]
    #[inline(always)]
    pub fn is_dmaburst_update(&self) -> bool {
        *self == UPDGAT::DmaburstUpdate
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == UPDGAT::Input1
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == UPDGAT::Input2
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == UPDGAT::Input3
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn is_input1_update(&self) -> bool {
        *self == UPDGAT::Input1Update
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn is_input2_update(&self) -> bool {
        *self == UPDGAT::Input2Update
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn is_input3_update(&self) -> bool {
        *self == UPDGAT::Input3Update
    }
}
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub type UPDGAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UPDGAT>;
impl<'a, REG> UPDGAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Update occurs independently from the DMA burst transfer"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Independent)
    }
    #[doc = "Update occurs when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn dmaburst(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Dmaburst)
    }
    #[doc = "Update occurs on the update event following DMA burst transfer completion"]
    #[inline(always)]
    pub fn dmaburst_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::DmaburstUpdate)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2)
    }
    #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
    #[inline(always)]
    pub fn input1_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1Update)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
    #[inline(always)]
    pub fn input2_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2Update)
    }
    #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
    #[inline(always)]
    pub fn input3_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3Update)
    }
}
impl R {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ckpscx(&self) -> CKPSCX_R {
        CKPSCX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TX_REPU_R {
        TX_REPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TX_RSTU_R {
        TX_RSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ckpscx(&mut self) -> CKPSCX_W<TIMBCRrs> {
        CKPSCX_W::new(self, 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<TIMBCRrs> {
        CONT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<TIMBCRrs> {
        RETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<TIMBCRrs> {
        HALF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PSHPLL_W<TIMBCRrs> {
        PSHPLL_W::new(self, 6)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W<TIMBCRrs> {
        SYNCRSTX_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W<TIMBCRrs> {
        SYNCSTRTX_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> DELCMP2_W<TIMBCRrs> {
        DELCMP2_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> DELCMP4_W<TIMBCRrs> {
        DELCMP4_W::new(self, 14)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TX_REPU_W<TIMBCRrs> {
        TX_REPU_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TX_RSTU_W<TIMBCRrs> {
        TX_RSTU_W::new(self, 18)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<TIMBCRrs> {
        TBU_W::new(self, 20)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TCU_W<TIMBCRrs> {
        TCU_W::new(self, 21)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TDU_W<TIMBCRrs> {
        TDU_W::new(self, 22)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    #[must_use]
    pub fn teu(&mut self) -> TEU_W<TIMBCRrs> {
        TEU_W::new(self, 23)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MSTU_W<TIMBCRrs> {
        MSTU_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<TIMBCRrs> {
        DACSYNC_W::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<TIMBCRrs> {
        PREEN_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UPDGAT_W<TIMBCRrs> {
        UPDGAT_W::new(self, 28)
    }
}
#[doc = "Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMBCRrs;
impl crate::RegisterSpec for TIMBCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timbcr::R`](R) reader structure"]
impl crate::Readable for TIMBCRrs {}
#[doc = "`write(|w| ..)` method takes [`timbcr::W`](W) writer structure"]
impl crate::Writable for TIMBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMBCR to value 0"]
impl crate::Resettable for TIMBCRrs {
    const RESET_VALUE: u32 = 0;
}
