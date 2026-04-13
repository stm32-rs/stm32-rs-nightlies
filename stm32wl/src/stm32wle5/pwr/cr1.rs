///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Low-power mode selection for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    ///0: Stop 0 mode
    Stop0 = 0,
    ///1: Stop 1 mode
    Stop1 = 1,
    ///2: Stop 2 mode
    Stop2 = 2,
    ///3: Standby mode
    Standby = 3,
    ///4: Shutdown mode
    Shutdown = 4,
}
impl From<LPMS> for u8 {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMS {
    type Ux = u8;
}
impl crate::IsEnum for LPMS {}
///Field `LPMS` reader - Low-power mode selection for CPU1
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPMS> {
        match self.bits {
            0 => Some(LPMS::Stop0),
            1 => Some(LPMS::Stop1),
            2 => Some(LPMS::Stop2),
            3 => Some(LPMS::Standby),
            4 => Some(LPMS::Shutdown),
            _ => None,
        }
    }
    ///Stop 0 mode
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    ///Standby mode
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    ///Shutdown mode
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
///Field `LPMS` writer - Low-power mode selection for CPU1
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Stop 0 mode
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    ///Standby mode
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    ///Shutdown mode
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
/**sub-GHz SPI NSS source select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPINSSSEL {
    ///0: sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
    Subghzspicr = 0,
    ///1: sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
    Lptim3 = 1,
}
impl From<SUBGHZSPINSSSEL> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPINSSSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `SUBGHZSPINSSSEL` reader - sub-GHz SPI NSS source select
pub type SUBGHZSPINSSSEL_R = crate::BitReader<SUBGHZSPINSSSEL>;
impl SUBGHZSPINSSSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHZSPINSSSEL {
        match self.bits {
            false => SUBGHZSPINSSSEL::Subghzspicr,
            true => SUBGHZSPINSSSEL::Lptim3,
        }
    }
    ///sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
    #[inline(always)]
    pub fn is_subghzspicr(&self) -> bool {
        *self == SUBGHZSPINSSSEL::Subghzspicr
    }
    ///sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
    #[inline(always)]
    pub fn is_lptim3(&self) -> bool {
        *self == SUBGHZSPINSSSEL::Lptim3
    }
}
///Field `SUBGHZSPINSSSEL` writer - sub-GHz SPI NSS source select
pub type SUBGHZSPINSSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHZSPINSSSEL>;
impl<'a, REG> SUBGHZSPINSSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
    #[inline(always)]
    pub fn subghzspicr(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPINSSSEL::Subghzspicr)
    }
    ///sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
    #[inline(always)]
    pub fn lptim3(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPINSSSEL::Lptim3)
    }
}
/**Flash memory power down mode during LPRun for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDR {
    ///0: Flash memory in Idle mode when system is in LPRun mode
    Idle = 0,
    ///1: Flash memory in Power-down mode when system is in LPRun mode
    PowerDown = 1,
}
impl From<FPDR> for bool {
    #[inline(always)]
    fn from(variant: FPDR) -> Self {
        variant as u8 != 0
    }
}
///Field `FPDR` reader - Flash memory power down mode during LPRun for CPU1
pub type FPDR_R = crate::BitReader<FPDR>;
impl FPDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPDR {
        match self.bits {
            false => FPDR::Idle,
            true => FPDR::PowerDown,
        }
    }
    ///Flash memory in Idle mode when system is in LPRun mode
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDR::Idle
    }
    ///Flash memory in Power-down mode when system is in LPRun mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDR::PowerDown
    }
}
///Field `FPDR` writer - Flash memory power down mode during LPRun for CPU1
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG, FPDR>;
impl<'a, REG> FPDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory in Idle mode when system is in LPRun mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::Idle)
    }
    ///Flash memory in Power-down mode when system is in LPRun mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::PowerDown)
    }
}
/**Flash memory power down mode during LPSleep for CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS {
    ///0: Flash memory in Idle mode when system is in LPSleep mode
    Idle = 0,
    ///1: Flash memory in Power-down mode when system is in LPSleep mode
    PowerDown = 1,
}
impl From<FPDS> for bool {
    #[inline(always)]
    fn from(variant: FPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU1
pub type FPDS_R = crate::BitReader<FPDS>;
impl FPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPDS {
        match self.bits {
            false => FPDS::Idle,
            true => FPDS::PowerDown,
        }
    }
    ///Flash memory in Idle mode when system is in LPSleep mode
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS::Idle
    }
    ///Flash memory in Power-down mode when system is in LPSleep mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS::PowerDown
    }
}
///Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU1
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG, FPDS>;
impl<'a, REG> FPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory in Idle mode when system is in LPSleep mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::Idle)
    }
    ///Flash memory in Power-down mode when system is in LPSleep mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::PowerDown)
    }
}
/**Disable backup domain write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Access to RTC and backup registers disabled
    Disabled = 0,
    ///1: Access to RTC and backup registers enabled
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    ///Access to RTC and backup registers disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    ///Access to RTC and backup registers enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access to RTC and backup registers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    ///Access to RTC and backup registers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
/**Voltage scaling range selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///1: 1.2 V (range 1)
    V1_2 = 1,
    ///2: 1.0 V (range 2)
    V1_0 = 2,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
impl crate::IsEnum for VOS {}
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::V1_2),
            2 => Some(VOS::V1_0),
            _ => None,
        }
    }
    ///1.2 V (range 1)
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS::V1_2
    }
    ///1.0 V (range 2)
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        *self == VOS::V1_0
    }
}
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.2 V (range 1)
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_2)
    }
    ///1.0 V (range 2)
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_0)
    }
}
/**Low-power run

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR {
    ///0: Voltage regulator in Main mode in Low-power run mode
    MainMode = 0,
    ///1: Voltage regulator in low-power mode in Low-power run mode
    LowPowerMode = 1,
}
impl From<LPR> for bool {
    #[inline(always)]
    fn from(variant: LPR) -> Self {
        variant as u8 != 0
    }
}
///Field `LPR` reader - Low-power run
pub type LPR_R = crate::BitReader<LPR>;
impl LPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPR {
        match self.bits {
            false => LPR::MainMode,
            true => LPR::LowPowerMode,
        }
    }
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR::MainMode
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR::LowPowerMode
    }
}
///Field `LPR` writer - Low-power run
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG, LPR>;
impl<'a, REG> LPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::MainMode)
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::LowPowerMode)
    }
}
impl R {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - sub-GHz SPI NSS source select
    #[inline(always)]
    pub fn subghzspinsssel(&self) -> SUBGHZSPINSSSEL_R {
        SUBGHZSPINSSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash memory power down mode during LPRun for CPU1
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash memory power down mode during LPSleep for CPU1
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpr", &self.lpr())
            .field("vos", &self.vos())
            .field("dbp", &self.dbp())
            .field("fpds", &self.fpds())
            .field("fpdr", &self.fpdr())
            .field("subghzspinsssel", &self.subghzspinsssel())
            .field("lpms", &self.lpms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 3 - sub-GHz SPI NSS source select
    #[inline(always)]
    pub fn subghzspinsssel(&mut self) -> SUBGHZSPINSSSEL_W<'_, CR1rs> {
        SUBGHZSPINSSSEL_W::new(self, 3)
    }
    ///Bit 4 - Flash memory power down mode during LPRun for CPU1
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W<'_, CR1rs> {
        FPDR_W::new(self, 4)
    }
    ///Bit 5 - Flash memory power down mode during LPSleep for CPU1
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CR1rs> {
        FPDS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CR1rs> {
        VOS_W::new(self, 9)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&mut self) -> LPR_W<'_, CR1rs> {
        LPR_W::new(self, 14)
    }
}
/**Power control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x0200
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0200;
}
