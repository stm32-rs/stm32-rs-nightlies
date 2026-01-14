///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
/**low-power mode selection This bit defines the Deepsleep mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMS {
    ///0: Keeps Stop mode when entering DeepSleep
    StopMode = 0,
    ///1: Allows Standby mode when entering DeepSleep
    StandbyMode = 1,
}
impl From<LPMS> for bool {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMS` reader - low-power mode selection This bit defines the Deepsleep mode.
pub type LPMS_R = crate::BitReader<LPMS>;
impl LPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMS {
        match self.bits {
            false => LPMS::StopMode,
            true => LPMS::StandbyMode,
        }
    }
    ///Keeps Stop mode when entering DeepSleep
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == LPMS::StopMode
    }
    ///Allows Standby mode when entering DeepSleep
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == LPMS::StandbyMode
    }
}
///Field `LPMS` writer - low-power mode selection This bit defines the Deepsleep mode.
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Keeps Stop mode when entering DeepSleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::StopMode)
    }
    ///Allows Standby mode when entering DeepSleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::StandbyMode)
    }
}
/**system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVOS {
    ///1: SVOS5 scale 5
    Scale5 = 1,
    ///2: SVOS4 scale 4
    Scale4 = 2,
    ///3: SVOS3 scale 3
    Scale3 = 3,
}
impl From<SVOS> for u8 {
    #[inline(always)]
    fn from(variant: SVOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SVOS {
    type Ux = u8;
}
impl crate::IsEnum for SVOS {}
///Field `SVOS` reader - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_R = crate::FieldReader<SVOS>;
impl SVOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SVOS> {
        match self.bits {
            1 => Some(SVOS::Scale5),
            2 => Some(SVOS::Scale4),
            3 => Some(SVOS::Scale3),
            _ => None,
        }
    }
    ///SVOS5 scale 5
    #[inline(always)]
    pub fn is_scale5(&self) -> bool {
        *self == SVOS::Scale5
    }
    ///SVOS4 scale 4
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == SVOS::Scale4
    }
    ///SVOS3 scale 3
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == SVOS::Scale3
    }
}
///Field `SVOS` writer - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SVOS>;
impl<'a, REG> SVOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SVOS5 scale 5
    #[inline(always)]
    pub fn scale5(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale5)
    }
    ///SVOS4 scale 4
    #[inline(always)]
    pub fn scale4(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale4)
    }
    ///SVOS3 scale 3
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale3)
    }
}
/**clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    ///1: STOPF and SBF flags cleared
    Clear = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSSF> {
        match self.bits {
            true => Some(CSSF::Clear),
            _ => None,
        }
    }
    ///STOPF and SBF flags cleared
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSSF::Clear
    }
}
///Field `CSSF` writer - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG, CSSF>;
impl<'a, REG> CSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///STOPF and SBF flags cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSF::Clear)
    }
}
/**Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLPS {
    ///0: Flash memory remains in normal mode when the system enters Stop mode
    NormalMode = 0,
    ///1: Flash memory enters low-power mode when the system enters Stop mode
    LowPowerMode = 1,
}
impl From<FLPS> for bool {
    #[inline(always)]
    fn from(variant: FLPS) -> Self {
        variant as u8 != 0
    }
}
///Field `FLPS` reader - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
pub type FLPS_R = crate::BitReader<FLPS>;
impl FLPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLPS {
        match self.bits {
            false => FLPS::NormalMode,
            true => FLPS::LowPowerMode,
        }
    }
    ///Flash memory remains in normal mode when the system enters Stop mode
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == FLPS::NormalMode
    }
    ///Flash memory enters low-power mode when the system enters Stop mode
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == FLPS::LowPowerMode
    }
}
///Field `FLPS` writer - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG, FLPS>;
impl<'a, REG> FLPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory remains in normal mode when the system enters Stop mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS::NormalMode)
    }
    ///Flash memory enters low-power mode when the system enters Stop mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS::LowPowerMode)
    }
}
/**analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTE {
    ///0: Booster disabled
    Disabled = 0,
    ///1: Booster enabled if analog voltage ready (AVD_READY = 1)
    Enabled = 1,
}
impl From<BOOSTE> for bool {
    #[inline(always)]
    fn from(variant: BOOSTE) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOSTE` reader - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.
pub type BOOSTE_R = crate::BitReader<BOOSTE>;
impl BOOSTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTE {
        match self.bits {
            false => BOOSTE::Disabled,
            true => BOOSTE::Enabled,
        }
    }
    ///Booster disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTE::Disabled
    }
    ///Booster enabled if analog voltage ready (AVD_READY = 1)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTE::Enabled
    }
}
///Field `BOOSTE` writer - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTE>;
impl<'a, REG> BOOSTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Booster disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE::Disabled)
    }
    ///Booster enabled if analog voltage ready (AVD_READY = 1)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE::Enabled)
    }
}
/**analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVD_READY {
    ///0: Peripheral analog voltage VDDA not ready (default)
    NotReady = 0,
    ///1: Peripheral analog voltage VDDA ready
    Ready = 1,
}
impl From<AVD_READY> for bool {
    #[inline(always)]
    fn from(variant: AVD_READY) -> Self {
        variant as u8 != 0
    }
}
///Field `AVD_READY` reader - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
pub type AVD_READY_R = crate::BitReader<AVD_READY>;
impl AVD_READY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVD_READY {
        match self.bits {
            false => AVD_READY::NotReady,
            true => AVD_READY::Ready,
        }
    }
    ///Peripheral analog voltage VDDA not ready (default)
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == AVD_READY::NotReady
    }
    ///Peripheral analog voltage VDDA ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == AVD_READY::Ready
    }
}
///Field `AVD_READY` writer - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
pub type AVD_READY_W<'a, REG> = crate::BitWriter<'a, REG, AVD_READY>;
impl<'a, REG> AVD_READY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral analog voltage VDDA not ready (default)
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY::NotReady)
    }
    ///Peripheral analog voltage VDDA ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY::Ready)
    }
}
/**AHB SRAM2 shut-off in Stop mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SO {
    ///0: AHB RAM2 content is kept in Stop mode
    Kept = 0,
    ///1: AHB RAM2 content is lost in Stop mode
    Lost = 1,
}
impl From<SRAM2SO> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SO) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2SO` reader - AHB SRAM2 shut-off in Stop mode.
pub type SRAM2SO_R = crate::BitReader<SRAM2SO>;
impl SRAM2SO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2SO {
        match self.bits {
            false => SRAM2SO::Kept,
            true => SRAM2SO::Lost,
        }
    }
    ///AHB RAM2 content is kept in Stop mode
    #[inline(always)]
    pub fn is_kept(&self) -> bool {
        *self == SRAM2SO::Kept
    }
    ///AHB RAM2 content is lost in Stop mode
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == SRAM2SO::Lost
    }
}
///Field `SRAM2SO` writer - AHB SRAM2 shut-off in Stop mode.
pub type SRAM2SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2SO>;
impl<'a, REG> SRAM2SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB RAM2 content is kept in Stop mode
    #[inline(always)]
    pub fn kept(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SO::Kept)
    }
    ///AHB RAM2 content is lost in Stop mode
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SO::Lost)
    }
}
/**AHB SRAM1 shut-off in Stop mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SO {
    ///0: AHB RAM1 content is kept in Stop mode
    Kept = 0,
    ///1: AHB RAM1 content is lost in Stop mode
    Lost = 1,
}
impl From<SRAM1SO> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SO) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1SO` reader - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_R = crate::BitReader<SRAM1SO>;
impl SRAM1SO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1SO {
        match self.bits {
            false => SRAM1SO::Kept,
            true => SRAM1SO::Lost,
        }
    }
    ///AHB RAM1 content is kept in Stop mode
    #[inline(always)]
    pub fn is_kept(&self) -> bool {
        *self == SRAM1SO::Kept
    }
    ///AHB RAM1 content is lost in Stop mode
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == SRAM1SO::Lost
    }
}
///Field `SRAM1SO` writer - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1SO>;
impl<'a, REG> SRAM1SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB RAM1 content is kept in Stop mode
    #[inline(always)]
    pub fn kept(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO::Kept)
    }
    ///AHB RAM1 content is lost in Stop mode
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO::Lost)
    }
}
impl R {
    ///Bit 0 - low-power mode selection This bit defines the Deepsleep mode.
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 25 - AHB SRAM2 shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2so(&self) -> SRAM2SO_R {
        SRAM2SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    pub fn sram1so(&self) -> SRAM1SO_R {
        SRAM1SO_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("lpms", &self.lpms())
            .field("svos", &self.svos())
            .field("cssf", &self.cssf())
            .field("flps", &self.flps())
            .field("booste", &self.booste())
            .field("avd_ready", &self.avd_ready())
            .field("sram2so", &self.sram2so())
            .field("sram1so", &self.sram1so())
            .finish()
    }
}
impl W {
    ///Bit 0 - low-power mode selection This bit defines the Deepsleep mode.
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, PMCRrs> {
        LPMS_W::new(self, 0)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection These bits control the V CORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    pub fn svos(&mut self) -> SVOS_W<'_, PMCRrs> {
        SVOS_W::new(self, 2)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, PMCRrs> {
        CSSF_W::new(self, 7)
    }
    ///Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.
    #[inline(always)]
    pub fn flps(&mut self) -> FLPS_W<'_, PMCRrs> {
        FLPS_W::new(self, 9)
    }
    ///Bit 12 - analog switch V BOOST control This bit enables the booster to guarantee the analog switch AC performance when the V DD supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V DD supply voltage can be monitored through the PVD and the PLS bits.
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W<'_, PMCRrs> {
        BOOSTE_W::new(self, 12)
    }
    ///Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V DDA analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).
    #[inline(always)]
    pub fn avd_ready(&mut self) -> AVD_READY_W<'_, PMCRrs> {
        AVD_READY_W::new(self, 13)
    }
    ///Bit 25 - AHB SRAM2 shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2so(&mut self) -> SRAM2SO_W<'_, PMCRrs> {
        SRAM2SO_W::new(self, 25)
    }
    ///Bit 26 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    pub fn sram1so(&mut self) -> SRAM1SO_W<'_, PMCRrs> {
        SRAM1SO_W::new(self, 26)
    }
}
/**PWR power mode control register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#PWR:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0x0c
impl crate::Resettable for PMCRrs {
    const RESET_VALUE: u32 = 0x0c;
}
