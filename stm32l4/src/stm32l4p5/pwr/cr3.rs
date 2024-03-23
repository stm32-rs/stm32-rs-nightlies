#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Enable Wakeup pin WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1 {
    #[doc = "0: External Wakeup pin WKUPx is disabled"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register"]
    Enabled = 1,
}
impl From<EWUP1> for bool {
    #[inline(always)]
    fn from(variant: EWUP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1"]
pub type EWUP1_R = crate::BitReader<EWUP1>;
impl EWUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWUP1 {
        match self.bits {
            false => EWUP1::Disabled,
            true => EWUP1::Enabled,
        }
    }
    #[doc = "External Wakeup pin WKUPx is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1::Disabled
    }
    #[doc = "When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1::Enabled
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG, EWUP1>;
impl<'a, REG> EWUP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Wakeup pin WKUPx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Disabled)
    }
    #[doc = "When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWUP1::Enabled)
    }
}
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2"]
pub use EWUP1_R as EWUP2_R;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3"]
pub use EWUP1_R as EWUP3_R;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4"]
pub use EWUP1_R as EWUP4_R;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5"]
pub use EWUP1_R as EWUP5_R;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2"]
pub use EWUP1_W as EWUP2_W;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3"]
pub use EWUP1_W as EWUP3_W;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4"]
pub use EWUP1_W as EWUP4_W;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5"]
pub use EWUP1_W as EWUP5_W;
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRS {
    #[doc = "0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)"]
    PoweredOff = 0,
    #[doc = "1: Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)"]
    PoweredOn = 1,
    #[doc = "2: Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)"]
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
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RRS_R = crate::FieldReader<RRS>;
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RRS> {
        match self.bits {
            0 => Some(RRS::PoweredOff),
            1 => Some(RRS::PoweredOn),
            2 => Some(RRS::PartialPoweredOn),
            _ => None,
        }
    }
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)"]
    #[inline(always)]
    pub fn is_powered_off(&self) -> bool {
        *self == RRS::PoweredOff
    }
    #[doc = "Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == RRS::PoweredOn
    }
    #[doc = "Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)"]
    #[inline(always)]
    pub fn is_partial_powered_on(&self) -> bool {
        *self == RRS::PartialPoweredOn
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RRS>;
impl<'a, REG> RRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)"]
    #[inline(always)]
    pub fn powered_off(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PoweredOff)
    }
    #[doc = "Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)"]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PoweredOn)
    }
    #[doc = "Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)"]
    #[inline(always)]
    pub fn partial_powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(RRS::PartialPoweredOn)
    }
}
#[doc = "Apply pull-up and pull-down configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC {
    #[doc = "0: Configurations are not applied"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode"]
    Enabled = 1,
}
impl From<APC> for bool {
    #[inline(always)]
    fn from(variant: APC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration"]
pub type APC_R = crate::BitReader<APC>;
impl APC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APC {
        match self.bits {
            false => APC::Disabled,
            true => APC::Enabled,
        }
    }
    #[doc = "Configurations are not applied"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC::Disabled
    }
    #[doc = "When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC::Enabled
    }
}
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration"]
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG, APC>;
impl<'a, REG> APC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configurations are not applied"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Disabled)
    }
    #[doc = "When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(APC::Enabled)
    }
}
#[doc = "Enable ULP sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENULP {
    #[doc = "0: Sampling disabled"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes"]
    Enabled = 1,
}
impl From<ENULP> for bool {
    #[inline(always)]
    fn from(variant: ENULP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENULP` reader - Enable ULP sampling"]
pub type ENULP_R = crate::BitReader<ENULP>;
impl ENULP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENULP {
        match self.bits {
            false => ENULP::Disabled,
            true => ENULP::Enabled,
        }
    }
    #[doc = "Sampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENULP::Disabled
    }
    #[doc = "When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENULP::Enabled
    }
}
#[doc = "Field `ENULP` writer - Enable ULP sampling"]
pub type ENULP_W<'a, REG> = crate::BitWriter<'a, REG, ENULP>;
impl<'a, REG> ENULP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENULP::Disabled)
    }
    #[doc = "When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENULP::Enabled)
    }
}
#[doc = "Enable Pull-down activation on DSI pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIPDEN {
    #[doc = "0: Pull-Down is disabled on DSI pins"]
    Disabled = 0,
    #[doc = "1: Pull-Down is enabled on DSI pins"]
    Enabled = 1,
}
impl From<DSIPDEN> for bool {
    #[inline(always)]
    fn from(variant: DSIPDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSIPDEN` reader - Enable Pull-down activation on DSI pins"]
pub type DSIPDEN_R = crate::BitReader<DSIPDEN>;
impl DSIPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSIPDEN {
        match self.bits {
            false => DSIPDEN::Disabled,
            true => DSIPDEN::Enabled,
        }
    }
    #[doc = "Pull-Down is disabled on DSI pins"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIPDEN::Disabled
    }
    #[doc = "Pull-Down is enabled on DSI pins"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIPDEN::Enabled
    }
}
#[doc = "Field `DSIPDEN` writer - Enable Pull-down activation on DSI pins"]
pub type DSIPDEN_W<'a, REG> = crate::BitWriter<'a, REG, DSIPDEN>;
impl<'a, REG> DSIPDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-Down is disabled on DSI pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIPDEN::Disabled)
    }
    #[doc = "Pull-Down is enabled on DSI pins"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIPDEN::Enabled)
    }
}
#[doc = "Enable internal wakeup line\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL {
    #[doc = "0: Internal wakeup line disable"]
    Disabled = 0,
    #[doc = "1: Internal wakeup line enable"]
    Enabled = 1,
}
impl From<EIWUL> for bool {
    #[inline(always)]
    fn from(variant: EIWUL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line"]
pub type EIWUL_R = crate::BitReader<EIWUL>;
impl EIWUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIWUL {
        match self.bits {
            false => EIWUL::Disabled,
            true => EIWUL::Enabled,
        }
    }
    #[doc = "Internal wakeup line disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL::Disabled
    }
    #[doc = "Internal wakeup line enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL::Enabled
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line"]
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG, EIWUL>;
impl<'a, REG> EIWUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal wakeup line disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Disabled)
    }
    #[doc = "Internal wakeup line enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable ULP sampling"]
    #[inline(always)]
    pub fn enulp(&self) -> ENULP_R {
        ENULP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Pull-down activation on DSI pins"]
    #[inline(always)]
    pub fn dsipden(&self) -> DSIPDEN_R {
        DSIPDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CR3rs> {
        EWUP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CR3rs> {
        EWUP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CR3rs> {
        EWUP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<CR3rs> {
        EWUP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<CR3rs> {
        EWUP5_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<CR3rs> {
        RRS_W::new(self, 8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<CR3rs> {
        APC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable ULP sampling"]
    #[inline(always)]
    #[must_use]
    pub fn enulp(&mut self) -> ENULP_W<CR3rs> {
        ENULP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Pull-down activation on DSI pins"]
    #[inline(always)]
    #[must_use]
    pub fn dsipden(&mut self) -> DSIPDEN_W<CR3rs> {
        DSIPDEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
