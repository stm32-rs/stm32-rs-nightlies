///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
/**Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1 {
    ///0: Detection on high level (rising edge)
    RisingEdge = 0,
    ///1: Detection on low level (falling edge)
    FallingEdge = 1,
}
impl From<WP1> for bool {
    #[inline(always)]
    fn from(variant: WP1) -> Self {
        variant as u8 != 0
    }
}
///Field `WP1` reader - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
pub type WP1_R = crate::BitReader<WP1>;
impl WP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP1 {
        match self.bits {
            false => WP1::RisingEdge,
            true => WP1::FallingEdge,
        }
    }
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP1::RisingEdge
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP1::FallingEdge
    }
}
///Field `WP1` writer - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG, WP1>;
impl<'a, REG> WP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::RisingEdge)
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::FallingEdge)
    }
}
///Field `WP2` reader - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
pub use WP1_R as WP2_R;
///Field `WP3` reader - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
pub use WP1_R as WP3_R;
///Field `WP4` reader - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
pub use WP1_R as WP4_R;
///Field `WP5` reader - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
pub use WP1_R as WP5_R;
///Field `WP2` writer - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
pub use WP1_W as WP2_W;
///Field `WP3` writer - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
pub use WP1_W as WP3_W;
///Field `WP4` writer - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
pub use WP1_W as WP4_W;
///Field `WP5` writer - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
pub use WP1_W as WP5_W;
/**V<sub>BAT</sub> battery charging enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    ///0: VBAT battery charging disable
    Disabled = 0,
    ///1: VBAT battery charging enable
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
///Field `VBE` reader - V<sub>BAT</sub> battery charging enable
pub type VBE_R = crate::BitReader<VBE>;
impl VBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBE {
        match self.bits {
            false => VBE::Disabled,
            true => VBE::Enabled,
        }
    }
    ///VBAT battery charging disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    ///VBAT battery charging enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
///Field `VBE` writer - V<sub>BAT</sub> battery charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT battery charging disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    ///VBAT battery charging enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
/**V<sub>BAT</sub> battery charging resistor selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    ///0: Charge VBAT through a 5 kOhms resistor
    R5k = 0,
    ///1: Charge VBAT through a 1.5 kOhms resistor
    R1k5 = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
///Field `VBRS` reader - V<sub>BAT</sub> battery charging resistor selection
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::R5k,
            true => VBRS::R1k5,
        }
    }
    ///Charge VBAT through a 5 kOhms resistor
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == VBRS::R5k
    }
    ///Charge VBAT through a 1.5 kOhms resistor
    #[inline(always)]
    pub fn is_r1k5(&self) -> bool {
        *self == VBRS::R1k5
    }
}
///Field `VBRS` writer - V<sub>BAT</sub> battery charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Charge VBAT through a 5 kOhms resistor
    #[inline(always)]
    pub fn r5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R5k)
    }
    ///Charge VBAT through a 1.5 kOhms resistor
    #[inline(always)]
    pub fn r1k5(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R1k5)
    }
}
/**external SMPS on. This bit informs the internal regulator about external SMPS switch status to decrease regulator output to 0.95 V in Range 2, allowing the external SMPS output down to 1.00 V. Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_SMPS_ON {
    ///0: The external SMPS switch is open
    Disabled = 0,
    ///1: The external SMPS switch is closed, internal regulator output is set to 0.95 V
    Enabled = 1,
}
impl From<EXT_SMPS_ON> for bool {
    #[inline(always)]
    fn from(variant: EXT_SMPS_ON) -> Self {
        variant as u8 != 0
    }
}
///Field `EXT_SMPS_ON` reader - external SMPS on. This bit informs the internal regulator about external SMPS switch status to decrease regulator output to 0.95 V in Range 2, allowing the external SMPS output down to 1.00 V. Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
pub type EXT_SMPS_ON_R = crate::BitReader<EXT_SMPS_ON>;
impl EXT_SMPS_ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXT_SMPS_ON {
        match self.bits {
            false => EXT_SMPS_ON::Disabled,
            true => EXT_SMPS_ON::Enabled,
        }
    }
    ///The external SMPS switch is open
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXT_SMPS_ON::Disabled
    }
    ///The external SMPS switch is closed, internal regulator output is set to 0.95 V
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXT_SMPS_ON::Enabled
    }
}
///Field `EXT_SMPS_ON` writer - external SMPS on. This bit informs the internal regulator about external SMPS switch status to decrease regulator output to 0.95 V in Range 2, allowing the external SMPS output down to 1.00 V. Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
pub type EXT_SMPS_ON_W<'a, REG> = crate::BitWriter<'a, REG, EXT_SMPS_ON>;
impl<'a, REG> EXT_SMPS_ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The external SMPS switch is open
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_SMPS_ON::Disabled)
    }
    ///The external SMPS switch is closed, internal regulator output is set to 0.95 V
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_SMPS_ON::Enabled)
    }
}
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - V<sub>BAT</sub> battery charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - V<sub>BAT</sub> battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - external SMPS on. This bit informs the internal regulator about external SMPS switch status to decrease regulator output to 0.95 V in Range 2, allowing the external SMPS output down to 1.00 V. Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
    #[inline(always)]
    pub fn ext_smps_on(&self) -> EXT_SMPS_ON_R {
        EXT_SMPS_ON_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("wp1", &self.wp1())
            .field("wp2", &self.wp2())
            .field("wp3", &self.wp3())
            .field("wp4", &self.wp4())
            .field("wp5", &self.wp5())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .field("ext_smps_on", &self.ext_smps_on())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W<'_, CR4rs> {
        WP1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W<'_, CR4rs> {
        WP2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W<'_, CR4rs> {
        WP3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
    #[inline(always)]
    pub fn wp4(&mut self) -> WP4_W<'_, CR4rs> {
        WP4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W<'_, CR4rs> {
        WP5_W::new(self, 4)
    }
    ///Bit 8 - V<sub>BAT</sub> battery charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, CR4rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - V<sub>BAT</sub> battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, CR4rs> {
        VBRS_W::new(self, 9)
    }
    ///Bit 13 - external SMPS on. This bit informs the internal regulator about external SMPS switch status to decrease regulator output to 0.95 V in Range 2, allowing the external SMPS output down to 1.00 V. Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
    #[inline(always)]
    pub fn ext_smps_on(&mut self) -> EXT_SMPS_ON_W<'_, CR4rs> {
        EXT_SMPS_ON_W::new(self, 13)
    }
}
/**Power control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#PWR:CR4)*/
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
///`read()` method returns [`cr4::R`](R) reader structure
impl crate::Readable for CR4rs {}
///`write(|w| ..)` method takes [`cr4::W`](W) writer structure
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4rs {}
