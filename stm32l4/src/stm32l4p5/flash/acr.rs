#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
    #[doc = "3: 3 wait states"]
    Ws3 = 3,
    #[doc = "4: 4 wait states"]
    Ws4 = 4,
    #[doc = "5: 5 wait states"]
    Ws5 = 5,
    #[doc = "6: 6 wait states"]
    Ws6 = 6,
    #[doc = "7: 7 wait states"]
    Ws7 = 7,
    #[doc = "8: 8 wait states"]
    Ws8 = 8,
    #[doc = "9: 9 wait states"]
    Ws9 = 9,
    #[doc = "10: 10 wait states"]
    Ws10 = 10,
    #[doc = "11: 11 wait states"]
    Ws11 = 11,
    #[doc = "12: 12 wait states"]
    Ws12 = 12,
    #[doc = "13: 13 wait states"]
    Ws13 = 13,
    #[doc = "14: 14 wait states"]
    Ws14 = 14,
    #[doc = "15: 15 wait states"]
    Ws15 = 15,
}
impl From<LATENCY> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY {
    type Ux = u8;
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LATENCY {
        match self.bits {
            0 => LATENCY::Ws0,
            1 => LATENCY::Ws1,
            2 => LATENCY::Ws2,
            3 => LATENCY::Ws3,
            4 => LATENCY::Ws4,
            5 => LATENCY::Ws5,
            6 => LATENCY::Ws6,
            7 => LATENCY::Ws7,
            8 => LATENCY::Ws8,
            9 => LATENCY::Ws9,
            10 => LATENCY::Ws10,
            11 => LATENCY::Ws11,
            12 => LATENCY::Ws12,
            13 => LATENCY::Ws13,
            14 => LATENCY::Ws14,
            15 => LATENCY::Ws15,
            _ => unreachable!(),
        }
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY::Ws3
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY::Ws4
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY::Ws5
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY::Ws6
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY::Ws7
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY::Ws8
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY::Ws9
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY::Ws10
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY::Ws11
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY::Ws12
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY::Ws13
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY::Ws14
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY::Ws15
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn ws8(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn ws9(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn ws10(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn ws11(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn ws12(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn ws13(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn ws14(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn ws15(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws15)
    }
}
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
#[doc = "Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    #[doc = "0: Instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: Instruction cache is enabled"]
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::Disabled,
            true => ICEN::Enabled,
        }
    }
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
#[doc = "Data cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN {
    #[doc = "0: Data cache is disabled"]
    Disabled = 0,
    #[doc = "1: Data cache is enabled"]
    Enabled = 1,
}
impl From<DCEN> for bool {
    #[inline(always)]
    fn from(variant: DCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader<DCEN>;
impl DCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN {
        match self.bits {
            false => DCEN::Disabled,
            true => DCEN::Enabled,
        }
    }
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN::Disabled
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN::Enabled
    }
}
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG, DCEN>;
impl<'a, REG> DCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Disabled)
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Enabled)
    }
}
#[doc = "Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    #[doc = "0: Instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: Instruction cache is reset"]
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::NotReset,
            true => ICRST::Reset,
        }
    }
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
#[doc = "Data cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRST {
    #[doc = "0: Data cache is not reset"]
    NotReset = 0,
    #[doc = "1: Data cache is reset"]
    Reset = 1,
}
impl From<DCRST> for bool {
    #[inline(always)]
    fn from(variant: DCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader<DCRST>;
impl DCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCRST {
        match self.bits {
            false => DCRST::NotReset,
            true => DCRST::Reset,
        }
    }
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST::NotReset
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST::Reset
    }
}
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG, DCRST>;
impl<'a, REG> DCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::NotReset)
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::Reset)
    }
}
#[doc = "Flash Power-down mode during Low-power run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_PD {
    #[doc = "0: Flash in idle mode"]
    Idle = 0,
    #[doc = "1: Flash in Power-down mode"]
    PowerDown = 1,
}
impl From<RUN_PD> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_R = crate::BitReader<RUN_PD>;
impl RUN_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN_PD {
        match self.bits {
            false => RUN_PD::Idle,
            true => RUN_PD::PowerDown,
        }
    }
    #[doc = "Flash in idle mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RUN_PD::Idle
    }
    #[doc = "Flash in Power-down mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == RUN_PD::PowerDown
    }
}
#[doc = "Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG, RUN_PD>;
impl<'a, REG> RUN_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash in idle mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::Idle)
    }
    #[doc = "Flash in Power-down mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::PowerDown)
    }
}
#[doc = "Flash Power-down mode during Low-power sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_PD {
    #[doc = "0: Flash in idle mode during Sleep and Low-power sleep modes"]
    Idle = 0,
    #[doc = "1: Flash in Power-down mode during Sleep and Low-power sleep modes"]
    PowerDown = 1,
}
impl From<SLEEP_PD> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_R = crate::BitReader<SLEEP_PD>;
impl SLEEP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_PD {
        match self.bits {
            false => SLEEP_PD::Idle,
            true => SLEEP_PD::PowerDown,
        }
    }
    #[doc = "Flash in idle mode during Sleep and Low-power sleep modes"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEP_PD::Idle
    }
    #[doc = "Flash in Power-down mode during Sleep and Low-power sleep modes"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SLEEP_PD::PowerDown
    }
}
#[doc = "Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_PD>;
impl<'a, REG> SLEEP_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash in idle mode during Sleep and Low-power sleep modes"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::Idle)
    }
    #[doc = "Flash in Power-down mode during Sleep and Low-power sleep modes"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::PowerDown)
    }
}
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<ACRrs> {
        ICEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<ACRrs> {
        DCEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<ACRrs> {
        ICRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<ACRrs> {
        DCRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
