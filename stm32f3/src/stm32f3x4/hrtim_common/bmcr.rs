#[doc = "Register `BMCR` reader"]
pub type R = crate::R<BMCRrs>;
#[doc = "Register `BMCR` writer"]
pub type W = crate::W<BMCRrs>;
#[doc = "Burst Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BME {
    #[doc = "0: Burst mode disabled"]
    Disabled = 0,
    #[doc = "1: Burst mode enabled"]
    Enabled = 1,
}
impl From<BME> for bool {
    #[inline(always)]
    fn from(variant: BME) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BME` reader - Burst Mode enable"]
pub type BME_R = crate::BitReader<BME>;
impl BME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BME {
        match self.bits {
            false => BME::Disabled,
            true => BME::Enabled,
        }
    }
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BME::Disabled
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BME::Enabled
    }
}
#[doc = "Field `BME` writer - Burst Mode enable"]
pub type BME_W<'a, REG> = crate::BitWriter<'a, REG, BME>;
impl<'a, REG> BME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BME::Disabled)
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BME::Enabled)
    }
}
#[doc = "Burst Mode operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMOM {
    #[doc = "0: Single-shot mode"]
    SingleShot = 0,
    #[doc = "1: Continuous operation"]
    Continuous = 1,
}
impl From<BMOM> for bool {
    #[inline(always)]
    fn from(variant: BMOM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub type BMOM_R = crate::BitReader<BMOM>;
impl BMOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMOM {
        match self.bits {
            false => BMOM::SingleShot,
            true => BMOM::Continuous,
        }
    }
    #[doc = "Single-shot mode"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == BMOM::SingleShot
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == BMOM::Continuous
    }
}
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub type BMOM_W<'a, REG> = crate::BitWriter<'a, REG, BMOM>;
impl<'a, REG> BMOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-shot mode"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut crate::W<REG> {
        self.variant(BMOM::SingleShot)
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(BMOM::Continuous)
    }
}
#[doc = "Burst Mode Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCLK {
    #[doc = "0: Master timer reset/roll-over"]
    Master = 0,
    #[doc = "1: Timer A counter reset/roll-over"]
    TimerA = 1,
    #[doc = "2: Timer B counter reset/roll-over"]
    TimerB = 2,
    #[doc = "3: Timer C counter reset/roll-over"]
    TimerC = 3,
    #[doc = "4: Timer D counter reset/roll-over"]
    TimerD = 4,
    #[doc = "5: Timer E counter reset/roll-over"]
    TimerE = 5,
    #[doc = "6: On-chip Event 1 (BMClk\\[1\\]), acting as a burst mode counter clock"]
    Event1 = 6,
    #[doc = "7: On-chip Event 2 (BMClk\\[2\\]), acting as a burst mode counter clock"]
    Event2 = 7,
    #[doc = "8: On-chip Event 3 (BMClk\\[3\\]), acting as a burst mode counter clock"]
    Event3 = 8,
    #[doc = "9: On-chip Event 4 (BMClk\\[4\\]), acting as a burst mode counter clock"]
    Event4 = 9,
    #[doc = "10: Prescaled f_HRTIM clock (as per BMPRSC\\[3:0\\]
setting"]
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
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub type BMCLK_R = crate::FieldReader<BMCLK>;
impl BMCLK_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Master timer reset/roll-over"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == BMCLK::Master
    }
    #[doc = "Timer A counter reset/roll-over"]
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == BMCLK::TimerA
    }
    #[doc = "Timer B counter reset/roll-over"]
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == BMCLK::TimerB
    }
    #[doc = "Timer C counter reset/roll-over"]
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == BMCLK::TimerC
    }
    #[doc = "Timer D counter reset/roll-over"]
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == BMCLK::TimerD
    }
    #[doc = "Timer E counter reset/roll-over"]
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == BMCLK::TimerE
    }
    #[doc = "On-chip Event 1 (BMClk\\[1\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == BMCLK::Event1
    }
    #[doc = "On-chip Event 2 (BMClk\\[2\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn is_event2(&self) -> bool {
        *self == BMCLK::Event2
    }
    #[doc = "On-chip Event 3 (BMClk\\[3\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn is_event3(&self) -> bool {
        *self == BMCLK::Event3
    }
    #[doc = "On-chip Event 4 (BMClk\\[4\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn is_event4(&self) -> bool {
        *self == BMCLK::Event4
    }
    #[doc = "Prescaled f_HRTIM clock (as per BMPRSC\\[3:0\\]
setting"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == BMCLK::Clock
    }
}
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub type BMCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BMCLK>;
impl<'a, REG> BMCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master timer reset/roll-over"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Master)
    }
    #[doc = "Timer A counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerA)
    }
    #[doc = "Timer B counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerB)
    }
    #[doc = "Timer C counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerC)
    }
    #[doc = "Timer D counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerD)
    }
    #[doc = "Timer E counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::TimerE)
    }
    #[doc = "On-chip Event 1 (BMClk\\[1\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event1(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event1)
    }
    #[doc = "On-chip Event 2 (BMClk\\[2\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event2(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event2)
    }
    #[doc = "On-chip Event 3 (BMClk\\[3\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event3(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event3)
    }
    #[doc = "On-chip Event 4 (BMClk\\[4\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event4(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Event4)
    }
    #[doc = "Prescaled f_HRTIM clock (as per BMPRSC\\[3:0\\]
setting"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut crate::W<REG> {
        self.variant(BMCLK::Clock)
    }
}
#[doc = "Burst Mode Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMPRSC {
    #[doc = "0: Clock not divided"]
    Div1 = 0,
    #[doc = "1: Division by 2"]
    Div2 = 1,
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
    Div8 = 3,
    #[doc = "4: Division by 16"]
    Div16 = 4,
    #[doc = "5: Division by 32"]
    Div32 = 5,
    #[doc = "6: Division by 64"]
    Div64 = 6,
    #[doc = "7: Division by 128"]
    Div128 = 7,
    #[doc = "8: Division by 256"]
    Div256 = 8,
    #[doc = "9: Division by 512"]
    Div512 = 9,
    #[doc = "10: Division by 1024"]
    Div1024 = 10,
    #[doc = "11: Division by 2048"]
    Div2048 = 11,
    #[doc = "12: Division by 4096"]
    Div4096 = 12,
    #[doc = "13: Division by 8192"]
    Div8192 = 13,
    #[doc = "14: Division by 16384"]
    Div16384 = 14,
    #[doc = "15: Division by 32768"]
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
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub type BMPRSC_R = crate::FieldReader<BMPRSC>;
impl BMPRSC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == BMPRSC::Div1
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BMPRSC::Div2
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BMPRSC::Div4
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BMPRSC::Div8
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BMPRSC::Div16
    }
    #[doc = "Division by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BMPRSC::Div32
    }
    #[doc = "Division by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BMPRSC::Div64
    }
    #[doc = "Division by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BMPRSC::Div128
    }
    #[doc = "Division by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BMPRSC::Div256
    }
    #[doc = "Division by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == BMPRSC::Div512
    }
    #[doc = "Division by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == BMPRSC::Div1024
    }
    #[doc = "Division by 2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == BMPRSC::Div2048
    }
    #[doc = "Division by 4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == BMPRSC::Div4096
    }
    #[doc = "Division by 8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == BMPRSC::Div8192
    }
    #[doc = "Division by 16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == BMPRSC::Div16384
    }
    #[doc = "Division by 32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == BMPRSC::Div32768
    }
}
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub type BMPRSC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, BMPRSC>;
impl<'a, REG> BMPRSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div8)
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div16)
    }
    #[doc = "Division by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div32)
    }
    #[doc = "Division by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div64)
    }
    #[doc = "Division by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div128)
    }
    #[doc = "Division by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div256)
    }
    #[doc = "Division by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div512)
    }
    #[doc = "Division by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div1024)
    }
    #[doc = "Division by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div2048)
    }
    #[doc = "Division by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div4096)
    }
    #[doc = "Division by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div8192)
    }
    #[doc = "Division by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div16384)
    }
    #[doc = "Division by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(BMPRSC::Div32768)
    }
}
#[doc = "Burst Mode Preload Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPREN {
    #[doc = "0: Preload disabled: the write access is directly done into active registers"]
    Disabled = 0,
    #[doc = "1: Preload enabled: the write access is done into preload registers"]
    Enabled = 1,
}
impl From<BMPREN> for bool {
    #[inline(always)]
    fn from(variant: BMPREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub type BMPREN_R = crate::BitReader<BMPREN>;
impl BMPREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMPREN {
        match self.bits {
            false => BMPREN::Disabled,
            true => BMPREN::Enabled,
        }
    }
    #[doc = "Preload disabled: the write access is directly done into active registers"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPREN::Disabled
    }
    #[doc = "Preload enabled: the write access is done into preload registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPREN::Enabled
    }
}
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub type BMPREN_W<'a, REG> = crate::BitWriter<'a, REG, BMPREN>;
impl<'a, REG> BMPREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload disabled: the write access is directly done into active registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPREN::Disabled)
    }
    #[doc = "Preload enabled: the write access is done into preload registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BMPREN::Enabled)
    }
}
#[doc = "Master Timer Burst Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTBM {
    #[doc = "0: Counter clock is maintained and timer operates normally"]
    Normal = 0,
    #[doc = "1: Counter clock is stopped and counter is reset"]
    Stopped = 1,
}
impl From<MTBM> for bool {
    #[inline(always)]
    fn from(variant: MTBM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub type MTBM_R = crate::BitReader<MTBM>;
impl MTBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MTBM {
        match self.bits {
            false => MTBM::Normal,
            true => MTBM::Stopped,
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MTBM::Normal
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == MTBM::Stopped
    }
}
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub type MTBM_W<'a, REG> = crate::BitWriter<'a, REG, MTBM>;
impl<'a, REG> MTBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MTBM::Normal)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(MTBM::Stopped)
    }
}
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub use MTBM_R as TABM_R;
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub use MTBM_R as TBBM_R;
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub use MTBM_R as TCBM_R;
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub use MTBM_R as TDBM_R;
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub use MTBM_R as TEBM_R;
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub use MTBM_W as TABM_W;
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub use MTBM_W as TBBM_W;
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub use MTBM_W as TCBM_W;
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub use MTBM_W as TDBM_W;
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub use MTBM_W as TEBM_W;
#[doc = "Burst Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATR {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Burst operation ongoing"]
    Burst = 1,
}
impl From<BMSTATR> for bool {
    #[inline(always)]
    fn from(variant: BMSTATR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub type BMSTAT_R = crate::BitReader<BMSTATR>;
impl BMSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMSTATR {
        match self.bits {
            false => BMSTATR::Normal,
            true => BMSTATR::Burst,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BMSTATR::Normal
    }
    #[doc = "Burst operation ongoing"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BMSTATR::Burst
    }
}
#[doc = "Burst Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATW {
    #[doc = "0: Terminate burst mode"]
    Cancel = 0,
}
impl From<BMSTATW> for bool {
    #[inline(always)]
    fn from(variant: BMSTATW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub type BMSTAT_W<'a, REG> = crate::BitWriter<'a, REG, BMSTATW>;
impl<'a, REG> BMSTAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminate burst mode"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut crate::W<REG> {
        self.variant(BMSTATW::Cancel)
    }
}
impl R {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bme(&mut self) -> BME_W<BMCRrs> {
        BME_W::new(self, 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmom(&mut self) -> BMOM_W<BMCRrs> {
        BMOM_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn bmclk(&mut self) -> BMCLK_W<BMCRrs> {
        BMCLK_W::new(self, 2)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn bmprsc(&mut self) -> BMPRSC_W<BMCRrs> {
        BMPRSC_W::new(self, 6)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmpren(&mut self) -> BMPREN_W<BMCRrs> {
        BMPREN_W::new(self, 10)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtbm(&mut self) -> MTBM_W<BMCRrs> {
        MTBM_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tabm(&mut self) -> TABM_W<BMCRrs> {
        TABM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbbm(&mut self) -> TBBM_W<BMCRrs> {
        TBBM_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcbm(&mut self) -> TCBM_W<BMCRrs> {
        TCBM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdbm(&mut self) -> TDBM_W<BMCRrs> {
        TDBM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tebm(&mut self) -> TEBM_W<BMCRrs> {
        TEBM_W::new(self, 21)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    #[must_use]
    pub fn bmstat(&mut self) -> BMSTAT_W<BMCRrs> {
        BMSTAT_W::new(self, 31)
    }
}
#[doc = "Burst Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCRrs;
impl crate::RegisterSpec for BMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcr::R`](R) reader structure"]
impl crate::Readable for BMCRrs {}
#[doc = "`write(|w| ..)` method takes [`bmcr::W`](W) writer structure"]
impl crate::Writable for BMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BMCRrs {
    const RESET_VALUE: u32 = 0;
}
