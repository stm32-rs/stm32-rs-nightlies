///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Latency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    ///0: 0 wait states
    Ws0 = 0,
    ///1: 1 wait states
    Ws1 = 1,
    ///2: 2 wait states
    Ws2 = 2,
    ///3: 3 wait states
    Ws3 = 3,
    ///4: 4 wait states
    Ws4 = 4,
    ///5: 5 wait states
    Ws5 = 5,
    ///6: 6 wait states
    Ws6 = 6,
    ///7: 7 wait states
    Ws7 = 7,
    ///8: 8 wait states
    Ws8 = 8,
    ///9: 9 wait states
    Ws9 = 9,
    ///10: 10 wait states
    Ws10 = 10,
    ///11: 11 wait states
    Ws11 = 11,
    ///12: 12 wait states
    Ws12 = 12,
    ///13: 13 wait states
    Ws13 = 13,
    ///14: 14 wait states
    Ws14 = 14,
    ///15: 15 wait states
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
impl crate::IsEnum for LATENCY {}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
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
    ///0 wait states
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    ///1 wait states
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    ///2 wait states
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
    ///3 wait states
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY::Ws3
    }
    ///4 wait states
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY::Ws4
    }
    ///5 wait states
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY::Ws5
    }
    ///6 wait states
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY::Ws6
    }
    ///7 wait states
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY::Ws7
    }
    ///8 wait states
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY::Ws8
    }
    ///9 wait states
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY::Ws9
    }
    ///10 wait states
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY::Ws10
    }
    ///11 wait states
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY::Ws11
    }
    ///12 wait states
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY::Ws12
    }
    ///13 wait states
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY::Ws13
    }
    ///14 wait states
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY::Ws14
    }
    ///15 wait states
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY::Ws15
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LATENCY, crate::Safe>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0 wait states
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    ///1 wait states
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    ///2 wait states
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
    ///3 wait states
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws3)
    }
    ///4 wait states
    #[inline(always)]
    pub fn ws4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws4)
    }
    ///5 wait states
    #[inline(always)]
    pub fn ws5(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws5)
    }
    ///6 wait states
    #[inline(always)]
    pub fn ws6(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws6)
    }
    ///7 wait states
    #[inline(always)]
    pub fn ws7(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws7)
    }
    ///8 wait states
    #[inline(always)]
    pub fn ws8(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws8)
    }
    ///9 wait states
    #[inline(always)]
    pub fn ws9(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws9)
    }
    ///10 wait states
    #[inline(always)]
    pub fn ws10(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws10)
    }
    ///11 wait states
    #[inline(always)]
    pub fn ws11(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws11)
    }
    ///12 wait states
    #[inline(always)]
    pub fn ws12(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws12)
    }
    ///13 wait states
    #[inline(always)]
    pub fn ws13(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws13)
    }
    ///14 wait states
    #[inline(always)]
    pub fn ws14(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws14)
    }
    ///15 wait states
    #[inline(always)]
    pub fn ws15(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws15)
    }
}
/**Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: Prefetch is disabled
    Disabled = 0,
    ///1: Prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    ///Prefetch is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
/**Instruction cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN {
    ///0: Instruction cache is disabled
    Disabled = 0,
    ///1: Instruction cache is enabled
    Enabled = 1,
}
impl From<ICEN> for bool {
    #[inline(always)]
    fn from(variant: ICEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ICEN` reader - Instruction cache enable
pub type ICEN_R = crate::BitReader<ICEN>;
impl ICEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICEN {
        match self.bits {
            false => ICEN::Disabled,
            true => ICEN::Enabled,
        }
    }
    ///Instruction cache is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN::Disabled
    }
    ///Instruction cache is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN::Enabled
    }
}
///Field `ICEN` writer - Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG, ICEN>;
impl<'a, REG> ICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Instruction cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Disabled)
    }
    ///Instruction cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICEN::Enabled)
    }
}
/**Data cache enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN {
    ///0: Data cache is disabled
    Disabled = 0,
    ///1: Data cache is enabled
    Enabled = 1,
}
impl From<DCEN> for bool {
    #[inline(always)]
    fn from(variant: DCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCEN` reader - Data cache enable
pub type DCEN_R = crate::BitReader<DCEN>;
impl DCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCEN {
        match self.bits {
            false => DCEN::Disabled,
            true => DCEN::Enabled,
        }
    }
    ///Data cache is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN::Disabled
    }
    ///Data cache is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN::Enabled
    }
}
///Field `DCEN` writer - Data cache enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG, DCEN>;
impl<'a, REG> DCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Disabled)
    }
    ///Data cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN::Enabled)
    }
}
/**Instruction cache reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST {
    ///0: Instruction cache is not reset
    NotReset = 0,
    ///1: Instruction cache is reset
    Reset = 1,
}
impl From<ICRST> for bool {
    #[inline(always)]
    fn from(variant: ICRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ICRST` reader - Instruction cache reset
pub type ICRST_R = crate::BitReader<ICRST>;
impl ICRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICRST {
        match self.bits {
            false => ICRST::NotReset,
            true => ICRST::Reset,
        }
    }
    ///Instruction cache is not reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST::NotReset
    }
    ///Instruction cache is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST::Reset
    }
}
///Field `ICRST` writer - Instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG, ICRST>;
impl<'a, REG> ICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Instruction cache is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::NotReset)
    }
    ///Instruction cache is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ICRST::Reset)
    }
}
/**Data cache reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRST {
    ///0: Data cache is not reset
    NotReset = 0,
    ///1: Data cache is reset
    Reset = 1,
}
impl From<DCRST> for bool {
    #[inline(always)]
    fn from(variant: DCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DCRST` reader - Data cache reset
pub type DCRST_R = crate::BitReader<DCRST>;
impl DCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCRST {
        match self.bits {
            false => DCRST::NotReset,
            true => DCRST::Reset,
        }
    }
    ///Data cache is not reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST::NotReset
    }
    ///Data cache is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST::Reset
    }
}
///Field `DCRST` writer - Data cache reset
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG, DCRST>;
impl<'a, REG> DCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data cache is not reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::NotReset)
    }
    ///Data cache is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCRST::Reset)
    }
}
/**Flash Power-down mode during Low-power run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_PD {
    ///0: Flash in idle mode
    Idle = 0,
    ///1: Flash in Power-down mode
    PowerDown = 1,
}
impl From<RUN_PD> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD) -> Self {
        variant as u8 != 0
    }
}
///Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode
pub type RUN_PD_R = crate::BitReader<RUN_PD>;
impl RUN_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RUN_PD {
        match self.bits {
            false => RUN_PD::Idle,
            true => RUN_PD::PowerDown,
        }
    }
    ///Flash in idle mode
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RUN_PD::Idle
    }
    ///Flash in Power-down mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == RUN_PD::PowerDown
    }
}
///Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG, RUN_PD>;
impl<'a, REG> RUN_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash in idle mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::Idle)
    }
    ///Flash in Power-down mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::PowerDown)
    }
}
/**Flash Power-down mode during Low-power sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_PD {
    ///0: Flash in idle mode during Sleep and Low-power sleep modes
    Idle = 0,
    ///1: Flash in Power-down mode during Sleep and Low-power sleep modes
    PowerDown = 1,
}
impl From<SLEEP_PD> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_R = crate::BitReader<SLEEP_PD>;
impl SLEEP_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_PD {
        match self.bits {
            false => SLEEP_PD::Idle,
            true => SLEEP_PD::PowerDown,
        }
    }
    ///Flash in idle mode during Sleep and Low-power sleep modes
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEP_PD::Idle
    }
    ///Flash in Power-down mode during Sleep and Low-power sleep modes
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == SLEEP_PD::PowerDown
    }
}
///Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_PD>;
impl<'a, REG> SLEEP_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash in idle mode during Sleep and Low-power sleep modes
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::Idle)
    }
    ///Flash in Power-down mode during Sleep and Low-power sleep modes
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::PowerDown)
    }
}
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("dcen", &self.dcen())
            .field("icrst", &self.icrst())
            .field("dcrst", &self.dcrst())
            .field("run_pd", &self.run_pd())
            .field("sleep_pd", &self.sleep_pd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, ACRrs> {
        DCEN_W::new(self, 10)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W<'_, ACRrs> {
        DCRST_W::new(self, 12)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0x0600
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
