///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    ///0: External Wakeup pin WKUPx is disabled
    Disabled = 0,
    ///1: When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register
    Enabled = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Disabled,
            true => EWUP1::Enabled,
        }
    }
    ///External Wakeup pin WKUPx is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1::Disabled
    }
    ///When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1::Enabled
    }
}
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External Wakeup pin WKUPx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Disabled)
    }
    ///When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Enabled)
    }
}
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
pub use EWUP1_R as EWUP2_R;
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
pub use EWUP1_R as EWUP3_R;
///Field `EWUP4` reader - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub use EWUP1_R as EWUP4_R;
///Field `EWUP5` reader - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
pub use EWUP1_R as EWUP5_R;
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
pub use EWUP1_W as EWUP2_W;
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
pub use EWUP1_W as EWUP3_W;
///Field `EWUP4` writer - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub use EWUP1_W as EWUP4_W;
///Field `EWUP5` writer - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
pub use EWUP1_W as EWUP5_W;
/**SRAM2 retention in Standby mode For STM32L4Rxxx and STM32L4Sxxx devices bit 9 is reserved For STM32L4P5xx and STM32L4Q5xx devices:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRS {
    ///0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)
    PoweredOff = 0,
    ///1: Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)
    PoweredOn = 1,
    ///2: Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)
    PartialPoweredOn = 2,
}
impl From<RRS> for u8 {
    #[inline(always)]
    fn from(variant: RRS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RRS {
    type Ux = u8;
}
impl crate::IsEnum for RRS {}
///Field `RRS` reader - SRAM2 retention in Standby mode For STM32L4Rxxx and STM32L4Sxxx devices bit 9 is reserved For STM32L4P5xx and STM32L4Q5xx devices:
pub type RRS_R = crate::FieldReader<RRS>;
impl RRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RRS> {
        match self.bits {
            0 => Some(RRS::PoweredOff),
            1 => Some(RRS::PoweredOn),
            2 => Some(RRS::PartialPoweredOn),
            _ => None,
        }
    }
    ///SRAM2 is powered off in Standby mode (SRAM2 content is lost)
    #[inline(always)]
    pub fn is_powered_off(&self) -> bool {
        *self == RRS::PoweredOff
    }
    ///Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == RRS::PoweredOn
    }
    ///Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)
    #[inline(always)]
    pub fn is_partial_powered_on(&self) -> bool {
        *self == RRS::PartialPoweredOn
    }
}
///Field `RRS` writer - SRAM2 retention in Standby mode For STM32L4Rxxx and STM32L4Sxxx devices bit 9 is reserved For STM32L4P5xx and STM32L4Q5xx devices:
pub type RRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RRS>;
impl<'a, REG> RRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SRAM2 is powered off in Standby mode (SRAM2 content is lost)
    #[inline(always)]
    pub fn powered_off(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PoweredOff)
    }
    ///Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PoweredOn)
    }
    ///Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)
    #[inline(always)]
    pub fn partial_powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PartialPoweredOn)
    }
}
/**Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC {
    ///0: Configurations are not applied
    Disabled = 0,
    ///1: When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode
    Enabled = 1,
}
impl From<APC> for bool {
    #[inline(always)]
    fn from(variant: APC) -> Self {
        variant as u8 != 0
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode.
pub type APC_R = crate::BitReader<APC>;
impl APC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APC {
        match self.bits {
            false => APC::Disabled,
            true => APC::Enabled,
        }
    }
    ///Configurations are not applied
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC::Disabled
    }
    ///When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC::Enabled
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG, APC>;
impl<'a, REG> APC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Configurations are not applied
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Disabled)
    }
    ///When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Enabled)
    }
}
/**Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes. Note: Available on STM32L4P5xx andSTM32L4Q5xx only.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENULP {
    ///0: Sampling disabled
    Disabled = 0,
    ///1: When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes
    Enabled = 1,
}
impl From<ENULP> for bool {
    #[inline(always)]
    fn from(variant: ENULP) -> Self {
        variant as u8 != 0
    }
}
///Field `ENULP` reader - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes. Note: Available on STM32L4P5xx andSTM32L4Q5xx only.
pub type ENULP_R = crate::BitReader<ENULP>;
impl ENULP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENULP {
        match self.bits {
            false => ENULP::Disabled,
            true => ENULP::Enabled,
        }
    }
    ///Sampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENULP::Disabled
    }
    ///When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENULP::Enabled
    }
}
///Field `ENULP` writer - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes. Note: Available on STM32L4P5xx andSTM32L4Q5xx only.
pub type ENULP_W<'a, REG> = crate::BitWriter<'a, REG, ENULP>;
impl<'a, REG> ENULP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENULP::Disabled)
    }
    ///When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENULP::Enabled)
    }
}
/**Enable Pull-down activation on DSI pins

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIPDEN {
    ///0: Pull-Down is disabled on DSI pins
    Disabled = 0,
    ///1: Pull-Down is enabled on DSI pins
    Enabled = 1,
}
impl From<DSIPDEN> for bool {
    #[inline(always)]
    fn from(variant: DSIPDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DSIPDEN` reader - Enable Pull-down activation on DSI pins
pub type DSIPDEN_R = crate::BitReader<DSIPDEN>;
impl DSIPDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSIPDEN {
        match self.bits {
            false => DSIPDEN::Disabled,
            true => DSIPDEN::Enabled,
        }
    }
    ///Pull-Down is disabled on DSI pins
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIPDEN::Disabled
    }
    ///Pull-Down is enabled on DSI pins
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIPDEN::Enabled
    }
}
///Field `DSIPDEN` writer - Enable Pull-down activation on DSI pins
pub type DSIPDEN_W<'a, REG> = crate::BitWriter<'a, REG, DSIPDEN>;
impl<'a, REG> DSIPDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-Down is disabled on DSI pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIPDEN::Disabled)
    }
    ///Pull-Down is enabled on DSI pins
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIPDEN::Enabled)
    }
}
/**Enable internal wakeup line

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL {
    ///0: Internal wakeup line disable
    Disabled = 0,
    ///1: Internal wakeup line enable
    Enabled = 1,
}
impl From<EIWUL> for bool {
    #[inline(always)]
    fn from(variant: EIWUL) -> Self {
        variant as u8 != 0
    }
}
///Field `EIWUL` reader - Enable internal wakeup line
pub type EIWUL_R = crate::BitReader<EIWUL>;
impl EIWUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIWUL {
        match self.bits {
            false => EIWUL::Disabled,
            true => EIWUL::Enabled,
        }
    }
    ///Internal wakeup line disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL::Disabled
    }
    ///Internal wakeup line enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL::Enabled
    }
}
///Field `EIWUL` writer - Enable internal wakeup line
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG, EIWUL>;
impl<'a, REG> EIWUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal wakeup line disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Disabled)
    }
    ///Internal wakeup line enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Enabled)
    }
}
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - SRAM2 retention in Standby mode For STM32L4Rxxx and STM32L4Sxxx devices bit 9 is reserved For STM32L4P5xx and STM32L4Q5xx devices:
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes. Note: Available on STM32L4P5xx andSTM32L4Q5xx only.
    #[inline(always)]
    pub fn enulp(&self) -> ENULP_R {
        ENULP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable Pull-down activation on DSI pins
    #[inline(always)]
    pub fn dsipden(&self) -> DSIPDEN_R {
        DSIPDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("rrs", &self.rrs())
            .field("apc", &self.apc())
            .field("enulp", &self.enulp())
            .field("dsipden", &self.dsipden())
            .field("eiwul", &self.eiwul())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1 When this bit is set, the external wakeup pin WKUP1 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 When this bit is set, the external wakeup pin WKUP2 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 When this bit is set, the external wakeup pin WKUP3 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 When this bit is set, the external wakeup pin WKUP4 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, CR3rs> {
        EWUP4_W::new(self, 3)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 When this bit is set, the external wakeup pin WKUP5 is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, CR3rs> {
        EWUP5_W::new(self, 4)
    }
    ///Bits 8:9 - SRAM2 retention in Standby mode For STM32L4Rxxx and STM32L4Sxxx devices bit 9 is reserved For STM32L4P5xx and STM32L4Q5xx devices:
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<'_, CR3rs> {
        RRS_W::new(self, 8)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode.
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, CR3rs> {
        APC_W::new(self, 10)
    }
    ///Bit 11 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes. Note: Available on STM32L4P5xx andSTM32L4Q5xx only.
    #[inline(always)]
    pub fn enulp(&mut self) -> ENULP_W<'_, CR3rs> {
        ENULP_W::new(self, 11)
    }
    ///Bit 12 - Enable Pull-down activation on DSI pins
    #[inline(always)]
    pub fn dsipden(&mut self) -> DSIPDEN_W<'_, CR3rs> {
        DSIPDEN_W::new(self, 12)
    }
    ///Bit 15 - Enable internal wakeup line
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<'_, CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**Power control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x8000
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
