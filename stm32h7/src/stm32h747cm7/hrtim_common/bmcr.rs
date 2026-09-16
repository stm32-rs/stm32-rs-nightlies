///Register `BMCR` reader
pub type R = crate::R<BMCRrs>;
///Register `BMCR` writer
pub type W = crate::W<BMCRrs>;
/**Burst Mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BME {
    ///0: Burst mode disabled
    Disabled = 0,
    ///1: Burst mode enabled
    Enabled = 1,
}
impl From<BME> for bool {
    #[inline(always)]
    fn from(variant: BME) -> Self {
        variant as u8 != 0
    }
}
///Field `BME` reader - Burst Mode enable
pub type BME_R = crate::BitReader<BME>;
impl BME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BME {
        match self.bits {
            false => BME::Disabled,
            true => BME::Enabled,
        }
    }
    ///Burst mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BME::Disabled
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BME::Enabled
    }
}
///Field `BME` writer - Burst Mode enable
pub type BME_W<'a, REG> = crate::BitWriter<'a, REG, BME>;
impl<'a, REG> BME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Burst mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BME::Disabled)
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BME::Enabled)
    }
}
/**Burst Mode operating mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMOM {
    ///0: Single-shot mode
    SingleShot = 0,
    ///1: Continuous operation
    Continuous = 1,
}
impl From<BMOM> for bool {
    #[inline(always)]
    fn from(variant: BMOM) -> Self {
        variant as u8 != 0
    }
}
///Field `BMOM` reader - Burst Mode operating mode
pub type BMOM_R = crate::BitReader<BMOM>;
impl BMOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMOM {
        match self.bits {
            false => BMOM::SingleShot,
            true => BMOM::Continuous,
        }
    }
    ///Single-shot mode
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == BMOM::SingleShot
    }
    ///Continuous operation
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == BMOM::Continuous
    }
}
///Field `BMOM` writer - Burst Mode operating mode
pub type BMOM_W<'a, REG> = crate::BitWriter<'a, REG, BMOM>;
impl<'a, REG> BMOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single-shot mode
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut crate::W<REG> {
        self.variant(BMOM::SingleShot)
    }
    ///Continuous operation
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(BMOM::Continuous)
    }
}
/**Burst Mode Clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCLK {
    ///0: Master timer reset/roll-over
    Master = 0,
    ///1: Timer A counter reset/roll-over
    TimerA = 1,
    ///2: Timer B counter reset/roll-over
    TimerB = 2,
    ///3: Timer C counter reset/roll-over
    TimerC = 3,
    ///4: Timer D counter reset/roll-over
    TimerD = 4,
    ///5: Timer E counter reset/roll-over
    TimerE = 5,
    ///6: On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
    Event1 = 6,
    ///7: On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
    Event2 = 7,
    ///8: On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
    Event3 = 8,
    ///9: On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
    Event4 = 9,
    ///10: Prescaled f_HRTIM clock (as per BMPRSC\[3:0\] setting
    Clock = 10,
}
impl From<BMCLK> for u8 {
    #[inline(always)]
    fn from(variant: BMCLK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BMCLK {
    type Ux = u8;
}
impl crate::IsEnum for BMCLK {}
///Field `BMCLK` reader - Burst Mode Clock source
pub type BMCLK_R = crate::FieldReader<BMCLK>;
impl BMCLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BMCLK> {
        match self.bits {
            0 => Some(BMCLK::Master),
            1 => Some(BMCLK::TimerA),
            2 => Some(BMCLK::TimerB),
            3 => Some(BMCLK::TimerC),
            4 => Some(BMCLK::TimerD),
            5 => Some(BMCLK::TimerE),
            6 => Some(BMCLK::Event1),
            7 => Some(BMCLK::Event2),
            8 => Some(BMCLK::Event3),
            9 => Some(BMCLK::Event4),
            10 => Some(BMCLK::Clock),
            _ => None,
        }
    }
    ///Master timer reset/roll-over
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == BMCLK::Master
    }
    ///Timer A counter reset/roll-over
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == BMCLK::TimerA
    }
    ///Timer B counter reset/roll-over
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == BMCLK::TimerB
    }
    ///Timer C counter reset/roll-over
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == BMCLK::TimerC
    }
    ///Timer D counter reset/roll-over
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == BMCLK::TimerD
    }
    ///Timer E counter reset/roll-over
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == BMCLK::TimerE
    }
    ///On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == BMCLK::Event1
    }
    ///On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn is_event2(&self) -> bool {
        *self == BMCLK::Event2
    }
    ///On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn is_event3(&self) -> bool {
        *self == BMCLK::Event3
    }
    ///On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn is_event4(&self) -> bool {
        *self == BMCLK::Event4
    }
    ///Prescaled f_HRTIM clock (as per BMPRSC\[3:0\] setting
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == BMCLK::Clock
    }
}
///Field `BMCLK` writer - Burst Mode Clock source
pub type BMCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BMCLK>;
impl<'a, REG> BMCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Master timer reset/roll-over
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Master)
    }
    ///Timer A counter reset/roll-over
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerA)
    }
    ///Timer B counter reset/roll-over
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerB)
    }
    ///Timer C counter reset/roll-over
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerC)
    }
    ///Timer D counter reset/roll-over
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerD)
    }
    ///Timer E counter reset/roll-over
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerE)
    }
    ///On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event1(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event1)
    }
    ///On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event2(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event2)
    }
    ///On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event3(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event3)
    }
    ///On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event4(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event4)
    }
    ///Prescaled f_HRTIM clock (as per BMPRSC\[3:0\] setting
    #[inline(always)]
    pub fn clock(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Clock)
    }
}
/**Burst Mode Prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMPRSC {
    ///0: Clock not divided
    Div1 = 0,
    ///1: Division by 2
    Div2 = 1,
    ///2: Division by 4
    Div4 = 2,
    ///3: Division by 8
    Div8 = 3,
    ///4: Division by 16
    Div16 = 4,
    ///5: Division by 32
    Div32 = 5,
    ///6: Division by 64
    Div64 = 6,
    ///7: Division by 128
    Div128 = 7,
    ///8: Division by 256
    Div256 = 8,
    ///9: Division by 512
    Div512 = 9,
    ///10: Division by 1024
    Div1024 = 10,
    ///11: Division by 2048
    Div2048 = 11,
    ///12: Division by 4096
    Div4096 = 12,
    ///13: Division by 8192
    Div8192 = 13,
    ///14: Division by 16384
    Div16384 = 14,
    ///15: Division by 32768
    Div32768 = 15,
}
impl From<BMPRSC> for u8 {
    #[inline(always)]
    fn from(variant: BMPRSC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BMPRSC {
    type Ux = u8;
}
impl crate::IsEnum for BMPRSC {}
///Field `BMPRSC` reader - Burst Mode Prescaler
pub type BMPRSC_R = crate::FieldReader<BMPRSC>;
impl BMPRSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMPRSC {
        match self.bits {
            0 => BMPRSC::Div1,
            1 => BMPRSC::Div2,
            2 => BMPRSC::Div4,
            3 => BMPRSC::Div8,
            4 => BMPRSC::Div16,
            5 => BMPRSC::Div32,
            6 => BMPRSC::Div64,
            7 => BMPRSC::Div128,
            8 => BMPRSC::Div256,
            9 => BMPRSC::Div512,
            10 => BMPRSC::Div1024,
            11 => BMPRSC::Div2048,
            12 => BMPRSC::Div4096,
            13 => BMPRSC::Div8192,
            14 => BMPRSC::Div16384,
            15 => BMPRSC::Div32768,
            _ => unreachable!(),
        }
    }
    ///Clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == BMPRSC::Div1
    }
    ///Division by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BMPRSC::Div2
    }
    ///Division by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BMPRSC::Div4
    }
    ///Division by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BMPRSC::Div8
    }
    ///Division by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BMPRSC::Div16
    }
    ///Division by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BMPRSC::Div32
    }
    ///Division by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BMPRSC::Div64
    }
    ///Division by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BMPRSC::Div128
    }
    ///Division by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BMPRSC::Div256
    }
    ///Division by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == BMPRSC::Div512
    }
    ///Division by 1024
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == BMPRSC::Div1024
    }
    ///Division by 2048
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == BMPRSC::Div2048
    }
    ///Division by 4096
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == BMPRSC::Div4096
    }
    ///Division by 8192
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == BMPRSC::Div8192
    }
    ///Division by 16384
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == BMPRSC::Div16384
    }
    ///Division by 32768
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == BMPRSC::Div32768
    }
}
///Field `BMPRSC` writer - Burst Mode Prescaler
pub type BMPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BMPRSC, crate::Safe>;
impl<'a, REG> BMPRSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div8)
    }
    ///Division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div16)
    }
    ///Division by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div32)
    }
    ///Division by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div64)
    }
    ///Division by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div128)
    }
    ///Division by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div256)
    }
    ///Division by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div512)
    }
    ///Division by 1024
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div1024)
    }
    ///Division by 2048
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div2048)
    }
    ///Division by 4096
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div4096)
    }
    ///Division by 8192
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div8192)
    }
    ///Division by 16384
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div16384)
    }
    ///Division by 32768
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div32768)
    }
}
/**Burst Mode Preload Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPREN {
    ///0: Preload disabled: the write access is directly done into active registers
    Disabled = 0,
    ///1: Preload enabled: the write access is done into preload registers
    Enabled = 1,
}
impl From<BMPREN> for bool {
    #[inline(always)]
    fn from(variant: BMPREN) -> Self {
        variant as u8 != 0
    }
}
///Field `BMPREN` reader - Burst Mode Preload Enable
pub type BMPREN_R = crate::BitReader<BMPREN>;
impl BMPREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMPREN {
        match self.bits {
            false => BMPREN::Disabled,
            true => BMPREN::Enabled,
        }
    }
    ///Preload disabled: the write access is directly done into active registers
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPREN::Disabled
    }
    ///Preload enabled: the write access is done into preload registers
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPREN::Enabled
    }
}
///Field `BMPREN` writer - Burst Mode Preload Enable
pub type BMPREN_W<'a, REG> = crate::BitWriter<'a, REG, BMPREN>;
impl<'a, REG> BMPREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload disabled: the write access is directly done into active registers
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPREN::Disabled)
    }
    ///Preload enabled: the write access is done into preload registers
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPREN::Enabled)
    }
}
/**Master Timer Burst Mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTBM {
    ///0: Counter clock is maintained and timer operates normally
    Normal = 0,
    ///1: Counter clock is stopped and counter is reset
    Stopped = 1,
}
impl From<MTBM> for bool {
    #[inline(always)]
    fn from(variant: MTBM) -> Self {
        variant as u8 != 0
    }
}
///Field `MTBM` reader - Master Timer Burst Mode
pub type MTBM_R = crate::BitReader<MTBM>;
impl MTBM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MTBM {
        match self.bits {
            false => MTBM::Normal,
            true => MTBM::Stopped,
        }
    }
    ///Counter clock is maintained and timer operates normally
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MTBM::Normal
    }
    ///Counter clock is stopped and counter is reset
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == MTBM::Stopped
    }
}
///Field `MTBM` writer - Master Timer Burst Mode
pub type MTBM_W<'a, REG> = crate::BitWriter<'a, REG, MTBM>;
impl<'a, REG> MTBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter clock is maintained and timer operates normally
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MTBM::Normal)
    }
    ///Counter clock is stopped and counter is reset
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(MTBM::Stopped)
    }
}
///Field `TBM(A,B,C,D,E)` reader - Timer %s Burst Mode
pub use MTBM_R as TBM_R;
///Field `TBM(A,B,C,D,E)` writer - Timer %s Burst Mode
pub use MTBM_W as TBM_W;
/**Burst Mode Status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATR {
    ///0: Normal operation
    Normal = 0,
    ///1: Burst operation ongoing
    Burst = 1,
}
impl From<BMSTATR> for bool {
    #[inline(always)]
    fn from(variant: BMSTATR) -> Self {
        variant as u8 != 0
    }
}
///Field `BMSTAT` reader - Burst Mode Status
pub type BMSTAT_R = crate::BitReader<BMSTATR>;
impl BMSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMSTATR {
        match self.bits {
            false => BMSTATR::Normal,
            true => BMSTATR::Burst,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BMSTATR::Normal
    }
    ///Burst operation ongoing
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BMSTATR::Burst
    }
}
/**Burst Mode Status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATW {
    ///0: Terminate burst mode
    Cancel = 0,
}
impl From<BMSTATW> for bool {
    #[inline(always)]
    fn from(variant: BMSTATW) -> Self {
        variant as u8 != 0
    }
}
///Field `BMSTAT` writer - Burst Mode Status
pub type BMSTAT_W<'a, REG> = crate::BitWriter0C<'a, REG, BMSTATW>;
impl<'a, REG> BMSTAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Terminate burst mode
    #[inline(always)]
    pub fn cancel(self) -> &'a mut crate::W<REG> {
        self.variant(BMSTATW::Cancel)
    }
}
impl R {
    ///Bit 0 - Burst Mode enable
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Burst Mode operating mode
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Burst Mode Clock source
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:9 - Burst Mode Prescaler
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 10 - Burst Mode Preload Enable
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Master Timer Burst Mode
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Timer (A,B,C,D,E) Burst Mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TABM` field.</div>
    #[inline(always)]
    pub fn tbm(&self, n: u8) -> TBM_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TBM_R::new(((self.bits >> (n + 17)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) Burst Mode
    #[inline(always)]
    pub fn tbm_iter(&self) -> impl Iterator<Item = TBM_R> + '_ {
        (0..5).map(move |n| TBM_R::new(((self.bits >> (n + 17)) & 1) != 0))
    }
    ///Bit 17 - Timer A Burst Mode
    #[inline(always)]
    pub fn tabm(&self) -> TBM_R {
        TBM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer B Burst Mode
    #[inline(always)]
    pub fn tbbm(&self) -> TBM_R {
        TBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer C Burst Mode
    #[inline(always)]
    pub fn tcbm(&self) -> TBM_R {
        TBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer D Burst Mode
    #[inline(always)]
    pub fn tdbm(&self) -> TBM_R {
        TBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer E Burst Mode
    #[inline(always)]
    pub fn tebm(&self) -> TBM_R {
        TBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Burst Mode Status
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMCR")
            .field("bmstat", &self.bmstat())
            .field("mtbm", &self.mtbm())
            .field("tabm", &self.tabm())
            .field("tbbm", &self.tbbm())
            .field("tcbm", &self.tcbm())
            .field("tdbm", &self.tdbm())
            .field("tebm", &self.tebm())
            .field("bmpren", &self.bmpren())
            .field("bmprsc", &self.bmprsc())
            .field("bmclk", &self.bmclk())
            .field("bmom", &self.bmom())
            .field("bme", &self.bme())
            .finish()
    }
}
impl W {
    ///Bit 0 - Burst Mode enable
    #[inline(always)]
    pub fn bme(&mut self) -> BME_W<'_, BMCRrs> {
        BME_W::new(self, 0)
    }
    ///Bit 1 - Burst Mode operating mode
    #[inline(always)]
    pub fn bmom(&mut self) -> BMOM_W<'_, BMCRrs> {
        BMOM_W::new(self, 1)
    }
    ///Bits 2:5 - Burst Mode Clock source
    #[inline(always)]
    pub fn bmclk(&mut self) -> BMCLK_W<'_, BMCRrs> {
        BMCLK_W::new(self, 2)
    }
    ///Bits 6:9 - Burst Mode Prescaler
    #[inline(always)]
    pub fn bmprsc(&mut self) -> BMPRSC_W<'_, BMCRrs> {
        BMPRSC_W::new(self, 6)
    }
    ///Bit 10 - Burst Mode Preload Enable
    #[inline(always)]
    pub fn bmpren(&mut self) -> BMPREN_W<'_, BMCRrs> {
        BMPREN_W::new(self, 10)
    }
    ///Bit 16 - Master Timer Burst Mode
    #[inline(always)]
    pub fn mtbm(&mut self) -> MTBM_W<'_, BMCRrs> {
        MTBM_W::new(self, 16)
    }
    ///Timer (A,B,C,D,E) Burst Mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TABM` field.</div>
    #[inline(always)]
    pub fn tbm(&mut self, n: u8) -> TBM_W<'_, BMCRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TBM_W::new(self, n + 17)
    }
    ///Bit 17 - Timer A Burst Mode
    #[inline(always)]
    pub fn tabm(&mut self) -> TBM_W<'_, BMCRrs> {
        TBM_W::new(self, 17)
    }
    ///Bit 18 - Timer B Burst Mode
    #[inline(always)]
    pub fn tbbm(&mut self) -> TBM_W<'_, BMCRrs> {
        TBM_W::new(self, 18)
    }
    ///Bit 19 - Timer C Burst Mode
    #[inline(always)]
    pub fn tcbm(&mut self) -> TBM_W<'_, BMCRrs> {
        TBM_W::new(self, 19)
    }
    ///Bit 20 - Timer D Burst Mode
    #[inline(always)]
    pub fn tdbm(&mut self) -> TBM_W<'_, BMCRrs> {
        TBM_W::new(self, 20)
    }
    ///Bit 21 - Timer E Burst Mode
    #[inline(always)]
    pub fn tebm(&mut self) -> TBM_W<'_, BMCRrs> {
        TBM_W::new(self, 21)
    }
    ///Bit 31 - Burst Mode Status
    #[inline(always)]
    pub fn bmstat(&mut self) -> BMSTAT_W<'_, BMCRrs> {
        BMSTAT_W::new(self, 31)
    }
}
/**Burst Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`bmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#HRTIM_Common:BMCR)*/
pub struct BMCRrs;
impl crate::RegisterSpec for BMCRrs {
    type Ux = u32;
}
///`read()` method returns [`bmcr::R`](R) reader structure
impl crate::Readable for BMCRrs {}
///`write(|w| ..)` method takes [`bmcr::W`](W) writer structure
impl crate::Writable for BMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_0000;
}
///`reset()` method sets BMCR to value 0
impl crate::Resettable for BMCRrs {}
