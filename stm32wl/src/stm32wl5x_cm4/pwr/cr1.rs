#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Low-power mode selection for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    #[doc = "0: Stop 0 mode"]
    Stop0 = 0,
    #[doc = "1: Stop 1 mode"]
    Stop1 = 1,
    #[doc = "2: Stop 2 mode"]
    Stop2 = 2,
    #[doc = "3: Standby mode"]
    Standby = 3,
    #[doc = "4: Shutdown mode"]
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
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU1"]
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU1"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
#[doc = "sub-GHz SPI NSS source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPINSSSEL {
    #[doc = "0: sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
    Subghzspicr = 0,
    #[doc = "1: sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
    Lptim3 = 1,
}
impl From<SUBGHZSPINSSSEL> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPINSSSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHZSPINSSSEL` reader - sub-GHz SPI NSS source select"]
pub type SUBGHZSPINSSSEL_R = crate::BitReader<SUBGHZSPINSSSEL>;
impl SUBGHZSPINSSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHZSPINSSSEL {
        match self.bits {
            false => SUBGHZSPINSSSEL::Subghzspicr,
            true => SUBGHZSPINSSSEL::Lptim3,
        }
    }
    #[doc = "sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
    #[inline(always)]
    pub fn is_subghzspicr(&self) -> bool {
        *self == SUBGHZSPINSSSEL::Subghzspicr
    }
    #[doc = "sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
    #[inline(always)]
    pub fn is_lptim3(&self) -> bool {
        *self == SUBGHZSPINSSSEL::Lptim3
    }
}
#[doc = "Field `SUBGHZSPINSSSEL` writer - sub-GHz SPI NSS source select"]
pub type SUBGHZSPINSSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHZSPINSSSEL>;
impl<'a, REG> SUBGHZSPINSSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
    #[inline(always)]
    pub fn subghzspicr(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPINSSSEL::Subghzspicr)
    }
    #[doc = "sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
    #[inline(always)]
    pub fn lptim3(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPINSSSEL::Lptim3)
    }
}
#[doc = "Flash memory power down mode during LPRun for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDR {
    #[doc = "0: Flash memory in Idle mode when system is in LPRun mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPRun mode"]
    PowerDown = 1,
}
impl From<FPDR> for bool {
    #[inline(always)]
    fn from(variant: FPDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDR` reader - Flash memory power down mode during LPRun for CPU1"]
pub type FPDR_R = crate::BitReader<FPDR>;
impl FPDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDR {
        match self.bits {
            false => FPDR::Idle,
            true => FPDR::PowerDown,
        }
    }
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDR::Idle
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDR::PowerDown
    }
}
#[doc = "Field `FPDR` writer - Flash memory power down mode during LPRun for CPU1"]
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG, FPDR>;
impl<'a, REG> FPDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::PowerDown)
    }
}
#[doc = "Flash memory power down mode during LPSleep for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS {
    #[doc = "0: Flash memory in Idle mode when system is in LPSleep mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPSleep mode"]
    PowerDown = 1,
}
impl From<FPDS> for bool {
    #[inline(always)]
    fn from(variant: FPDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU1"]
pub type FPDS_R = crate::BitReader<FPDS>;
impl FPDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDS {
        match self.bits {
            false => FPDS::Idle,
            true => FPDS::PowerDown,
        }
    }
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS::Idle
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS::PowerDown
    }
}
#[doc = "Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU1"]
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG, FPDS>;
impl<'a, REG> FPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::PowerDown)
    }
}
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    #[doc = "0: Access to RTC and backup registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to RTC and backup registers enabled"]
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    #[doc = "Access to RTC and backup registers disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    #[doc = "Access to RTC and backup registers enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to RTC and backup registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    #[doc = "Access to RTC and backup registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    #[doc = "1: 1.2 V (range 1)"]
    V1_2 = 1,
    #[doc = "2: 1.0 V (range 2)"]
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
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::V1_2),
            2 => Some(VOS::V1_0),
            _ => None,
        }
    }
    #[doc = "1.2 V (range 1)"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS::V1_2
    }
    #[doc = "1.0 V (range 2)"]
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        *self == VOS::V1_0
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.2 V (range 1)"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_2)
    }
    #[doc = "1.0 V (range 2)"]
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::V1_0)
    }
}
#[doc = "Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR {
    #[doc = "0: Voltage regulator in Main mode in Low-power run mode"]
    MainMode = 0,
    #[doc = "1: Voltage regulator in low-power mode in Low-power run mode"]
    LowPowerMode = 1,
}
impl From<LPR> for bool {
    #[inline(always)]
    fn from(variant: LPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader<LPR>;
impl LPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPR {
        match self.bits {
            false => LPR::MainMode,
            true => LPR::LowPowerMode,
        }
    }
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR::MainMode
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR::LowPowerMode
    }
}
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG, LPR>;
impl<'a, REG> LPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::MainMode)
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::LowPowerMode)
    }
}
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU1"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - sub-GHz SPI NSS source select"]
    #[inline(always)]
    pub fn subghzspinsssel(&self) -> SUBGHZSPINSSSEL_R {
        SUBGHZSPINSSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU1"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU1"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<CR1rs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 3 - sub-GHz SPI NSS source select"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspinsssel(&mut self) -> SUBGHZSPINSSSEL_W<CR1rs> {
        SUBGHZSPINSSSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<CR1rs> {
        FPDR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<CR1rs> {
        FPDS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CR1rs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CR1rs> {
        VOS_W::new(self, 9)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<CR1rs> {
        LPR_W::new(self, 14)
    }
}
#[doc = "Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0200"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0200;
}
