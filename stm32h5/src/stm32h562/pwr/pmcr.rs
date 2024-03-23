#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCRrs>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCRrs>;
#[doc = "low-power mode selection This bit defines the Deepsleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMS {
    #[doc = "0: Keeps Stop mode when entering DeepSleep"]
    StopMode = 0,
    #[doc = "1: Allows Standby mode when entering DeepSleep"]
    StandbyMode = 1,
}
impl From<LPMS> for bool {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMS` reader - low-power mode selection This bit defines the Deepsleep mode."]
pub type LPMS_R = crate::BitReader<LPMS>;
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPMS {
        match self.bits {
            false => LPMS::StopMode,
            true => LPMS::StandbyMode,
        }
    }
    #[doc = "Keeps Stop mode when entering DeepSleep"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == LPMS::StopMode
    }
    #[doc = "Allows Standby mode when entering DeepSleep"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == LPMS::StandbyMode
    }
}
#[doc = "Field `LPMS` writer - low-power mode selection This bit defines the Deepsleep mode."]
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keeps Stop mode when entering DeepSleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::StopMode)
    }
    #[doc = "Allows Standby mode when entering DeepSleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::StandbyMode)
    }
}
#[doc = "system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVOS {
    #[doc = "1: SVOS5 scale 5"]
    Scale5 = 1,
    #[doc = "2: SVOS4 scale 4"]
    Scale4 = 2,
    #[doc = "3: SVOS3 scale 3"]
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
#[doc = "Field `SVOS` reader - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_R = crate::FieldReader<SVOS>;
impl SVOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SVOS> {
        match self.bits {
            1 => Some(SVOS::Scale5),
            2 => Some(SVOS::Scale4),
            3 => Some(SVOS::Scale3),
            _ => None,
        }
    }
    #[doc = "SVOS5 scale 5"]
    #[inline(always)]
    pub fn is_scale5(&self) -> bool {
        *self == SVOS::Scale5
    }
    #[doc = "SVOS4 scale 4"]
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == SVOS::Scale4
    }
    #[doc = "SVOS3 scale 3"]
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == SVOS::Scale3
    }
}
#[doc = "Field `SVOS` writer - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SVOS>;
impl<'a, REG> SVOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVOS5 scale 5"]
    #[inline(always)]
    pub fn scale5(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale5)
    }
    #[doc = "SVOS4 scale 4"]
    #[inline(always)]
    pub fn scale4(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale4)
    }
    #[doc = "SVOS3 scale 3"]
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(SVOS::Scale3)
    }
}
#[doc = "clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    #[doc = "1: STOPF and SBF flags cleared"]
    Clear = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSSF> {
        match self.bits {
            true => Some(CSSF::Clear),
            _ => None,
        }
    }
    #[doc = "STOPF and SBF flags cleared"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSSF::Clear
    }
}
#[doc = "Field `CSSF` writer - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG, CSSF>;
impl<'a, REG> CSSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOPF and SBF flags cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSSF::Clear)
    }
}
#[doc = "Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLPS {
    #[doc = "0: Flash memory remains in normal mode when the system enters Stop mode"]
    NormalMode = 0,
    #[doc = "1: Flash memory enters low-power mode when the system enters Stop mode"]
    LowPowerMode = 1,
}
impl From<FLPS> for bool {
    #[inline(always)]
    fn from(variant: FLPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLPS` reader - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
pub type FLPS_R = crate::BitReader<FLPS>;
impl FLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLPS {
        match self.bits {
            false => FLPS::NormalMode,
            true => FLPS::LowPowerMode,
        }
    }
    #[doc = "Flash memory remains in normal mode when the system enters Stop mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == FLPS::NormalMode
    }
    #[doc = "Flash memory enters low-power mode when the system enters Stop mode"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == FLPS::LowPowerMode
    }
}
#[doc = "Field `FLPS` writer - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG, FLPS>;
impl<'a, REG> FLPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory remains in normal mode when the system enters Stop mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS::NormalMode)
    }
    #[doc = "Flash memory enters low-power mode when the system enters Stop mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(FLPS::LowPowerMode)
    }
}
#[doc = "analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTE {
    #[doc = "0: Booster disabled"]
    Disabled = 0,
    #[doc = "1: Booster enabled if analog voltage ready (AVD_READY = 1)"]
    Enabled = 1,
}
impl From<BOOSTE> for bool {
    #[inline(always)]
    fn from(variant: BOOSTE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTE` reader - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits."]
pub type BOOSTE_R = crate::BitReader<BOOSTE>;
impl BOOSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOSTE {
        match self.bits {
            false => BOOSTE::Disabled,
            true => BOOSTE::Enabled,
        }
    }
    #[doc = "Booster disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTE::Disabled
    }
    #[doc = "Booster enabled if analog voltage ready (AVD_READY = 1)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTE::Enabled
    }
}
#[doc = "Field `BOOSTE` writer - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits."]
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG, BOOSTE>;
impl<'a, REG> BOOSTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Booster disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE::Disabled)
    }
    #[doc = "Booster enabled if analog voltage ready (AVD_READY = 1)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOOSTE::Enabled)
    }
}
#[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVD_READY {
    #[doc = "0: Peripheral analog voltage VDDA not ready (default)"]
    NotReady = 0,
    #[doc = "1: Peripheral analog voltage VDDA ready"]
    Ready = 1,
}
impl From<AVD_READY> for bool {
    #[inline(always)]
    fn from(variant: AVD_READY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVD_READY` reader - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
pub type AVD_READY_R = crate::BitReader<AVD_READY>;
impl AVD_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AVD_READY {
        match self.bits {
            false => AVD_READY::NotReady,
            true => AVD_READY::Ready,
        }
    }
    #[doc = "Peripheral analog voltage VDDA not ready (default)"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == AVD_READY::NotReady
    }
    #[doc = "Peripheral analog voltage VDDA ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == AVD_READY::Ready
    }
}
#[doc = "Field `AVD_READY` writer - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
pub type AVD_READY_W<'a, REG> = crate::BitWriter<'a, REG, AVD_READY>;
impl<'a, REG> AVD_READY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral analog voltage VDDA not ready (default)"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY::NotReady)
    }
    #[doc = "Peripheral analog voltage VDDA ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(AVD_READY::Ready)
    }
}
#[doc = "Field `SRAM3SO` reader - AHB SRAM3 shut-off in Stop mode."]
pub type SRAM3SO_R = crate::BitReader;
#[doc = "Field `SRAM3SO` writer - AHB SRAM3 shut-off in Stop mode."]
pub type SRAM3SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AHB SRAM2 16-Kbyte shut-off in Stop mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_16SO {
    #[doc = "0: AHB RAM2 content is kept in Stop mode"]
    Kept = 0,
    #[doc = "1: AHB RAM2 content is lost in Stop mode"]
    Lost = 1,
}
impl From<SRAM2_16SO> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_16SO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_16SO` reader - AHB SRAM2 16-Kbyte shut-off in Stop mode."]
pub type SRAM2_16SO_R = crate::BitReader<SRAM2_16SO>;
impl SRAM2_16SO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2_16SO {
        match self.bits {
            false => SRAM2_16SO::Kept,
            true => SRAM2_16SO::Lost,
        }
    }
    #[doc = "AHB RAM2 content is kept in Stop mode"]
    #[inline(always)]
    pub fn is_kept(&self) -> bool {
        *self == SRAM2_16SO::Kept
    }
    #[doc = "AHB RAM2 content is lost in Stop mode"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == SRAM2_16SO::Lost
    }
}
#[doc = "Field `SRAM2_16SO` writer - AHB SRAM2 16-Kbyte shut-off in Stop mode."]
pub type SRAM2_16SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2_16SO>;
impl<'a, REG> SRAM2_16SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB RAM2 content is kept in Stop mode"]
    #[inline(always)]
    pub fn kept(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_16SO::Kept)
    }
    #[doc = "AHB RAM2 content is lost in Stop mode"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2_16SO::Lost)
    }
}
#[doc = "Field `SRAM2_48SO` reader - AHB SRAM2 48-Kbyte shut-off in Stop mode."]
pub use SRAM2_16SO_R as SRAM2_48SO_R;
#[doc = "Field `SRAM2_48SO` writer - AHB SRAM2 48-Kbyte shut-off in Stop mode."]
pub use SRAM2_16SO_W as SRAM2_48SO_W;
#[doc = "AHB SRAM1 shut-off in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SO {
    #[doc = "0: AHB RAM1 content is kept in Stop mode"]
    Kept = 0,
    #[doc = "1: AHB RAM1 content is lost in Stop mode"]
    Lost = 1,
}
impl From<SRAM1SO> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1SO` reader - AHB SRAM1 shut-off in Stop mode"]
pub type SRAM1SO_R = crate::BitReader<SRAM1SO>;
impl SRAM1SO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1SO {
        match self.bits {
            false => SRAM1SO::Kept,
            true => SRAM1SO::Lost,
        }
    }
    #[doc = "AHB RAM1 content is kept in Stop mode"]
    #[inline(always)]
    pub fn is_kept(&self) -> bool {
        *self == SRAM1SO::Kept
    }
    #[doc = "AHB RAM1 content is lost in Stop mode"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == SRAM1SO::Lost
    }
}
#[doc = "Field `SRAM1SO` writer - AHB SRAM1 shut-off in Stop mode"]
pub type SRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1SO>;
impl<'a, REG> SRAM1SO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHB RAM1 content is kept in Stop mode"]
    #[inline(always)]
    pub fn kept(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO::Kept)
    }
    #[doc = "AHB RAM1 content is lost in Stop mode"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SO::Lost)
    }
}
impl R {
    #[doc = "Bit 0 - low-power mode selection This bit defines the Deepsleep mode."]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits."]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB SRAM3 shut-off in Stop mode."]
    #[inline(always)]
    pub fn sram3so(&self) -> SRAM3SO_R {
        SRAM3SO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AHB SRAM2 16-Kbyte shut-off in Stop mode."]
    #[inline(always)]
    pub fn sram2_16so(&self) -> SRAM2_16SO_R {
        SRAM2_16SO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AHB SRAM2 48-Kbyte shut-off in Stop mode."]
    #[inline(always)]
    pub fn sram2_48so(&self) -> SRAM2_48SO_R {
        SRAM2_48SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AHB SRAM1 shut-off in Stop mode"]
    #[inline(always)]
    pub fn sram1so(&self) -> SRAM1SO_R {
        SRAM1SO_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - low-power mode selection This bit defines the Deepsleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<PMCRrs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - system Stop mode voltage scaling selection These bits control the V&lt;sub>CORE&lt;/sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    #[must_use]
    pub fn svos(&mut self) -> SVOS_W<PMCRrs> {
        SVOS_W::new(self, 2)
    }
    #[doc = "Bit 7 - clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<PMCRrs> {
        CSSF_W::new(self, 7)
    }
    #[doc = "Bit 9 - Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
    #[inline(always)]
    #[must_use]
    pub fn flps(&mut self) -> FLPS_W<PMCRrs> {
        FLPS_W::new(self, 9)
    }
    #[doc = "Bit 12 - analog switch V&lt;sub>BOOST&lt;/sub> control This bit enables the booster to guarantee the analog switch AC performance when the V&lt;sub>DD&lt;/sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V&lt;sub>DD&lt;/sub> supply voltage can be monitored through the PVD and the PLS bits."]
    #[inline(always)]
    #[must_use]
    pub fn booste(&mut self) -> BOOSTE_W<PMCRrs> {
        BOOSTE_W::new(self, 12)
    }
    #[doc = "Bit 13 - analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V&lt;sub>DDA&lt;/sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
    #[inline(always)]
    #[must_use]
    pub fn avd_ready(&mut self) -> AVD_READY_W<PMCRrs> {
        AVD_READY_W::new(self, 13)
    }
    #[doc = "Bit 23 - AHB SRAM3 shut-off in Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn sram3so(&mut self) -> SRAM3SO_W<PMCRrs> {
        SRAM3SO_W::new(self, 23)
    }
    #[doc = "Bit 24 - AHB SRAM2 16-Kbyte shut-off in Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn sram2_16so(&mut self) -> SRAM2_16SO_W<PMCRrs> {
        SRAM2_16SO_W::new(self, 24)
    }
    #[doc = "Bit 25 - AHB SRAM2 48-Kbyte shut-off in Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn sram2_48so(&mut self) -> SRAM2_48SO_W<PMCRrs> {
        SRAM2_48SO_W::new(self, 25)
    }
    #[doc = "Bit 26 - AHB SRAM1 shut-off in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1so(&mut self) -> SRAM1SO_W<PMCRrs> {
        SRAM1SO_W::new(self, 26)
    }
}
#[doc = "PWR power mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PMCRrs {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMCR to value 0x0c"]
impl crate::Resettable for PMCRrs {
    const RESET_VALUE: u32 = 0x0c;
}
