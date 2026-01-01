///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
/**Wakeup pin WKUP1 polarity

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
///Field `WP1` reader - Wakeup pin WKUP1 polarity
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
///Field `WP1` writer - Wakeup pin WKUP1 polarity
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
/**Wakeup pin WKUP2 polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2 {
    ///0: Detection on high level (rising edge)
    RisingEdge = 0,
    ///1: Detection on low level (falling edge)
    FallingEdge = 1,
}
impl From<WP2> for bool {
    #[inline(always)]
    fn from(variant: WP2) -> Self {
        variant as u8 != 0
    }
}
///Field `WP2` reader - Wakeup pin WKUP2 polarity
pub type WP2_R = crate::BitReader<WP2>;
impl WP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP2 {
        match self.bits {
            false => WP2::RisingEdge,
            true => WP2::FallingEdge,
        }
    }
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP2::RisingEdge
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP2::FallingEdge
    }
}
///Field `WP2` writer - Wakeup pin WKUP2 polarity
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG, WP2>;
impl<'a, REG> WP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::RisingEdge)
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::FallingEdge)
    }
}
/**Wakeup pin WKUP3 polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3 {
    ///0: Detection on high level (rising edge)
    RisingEdge = 0,
    ///1: Detection on low level (falling edge)
    FallingEdge = 1,
}
impl From<WP3> for bool {
    #[inline(always)]
    fn from(variant: WP3) -> Self {
        variant as u8 != 0
    }
}
///Field `WP3` reader - Wakeup pin WKUP3 polarity
pub type WP3_R = crate::BitReader<WP3>;
impl WP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP3 {
        match self.bits {
            false => WP3::RisingEdge,
            true => WP3::FallingEdge,
        }
    }
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP3::RisingEdge
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP3::FallingEdge
    }
}
///Field `WP3` writer - Wakeup pin WKUP3 polarity
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG, WP3>;
impl<'a, REG> WP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::RisingEdge)
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::FallingEdge)
    }
}
/**VBAT battery charging enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    ///0: VBAT battery charging disabled
    Disabled = 0,
    ///1: VBAT battery charging enabled
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
///Field `VBE` reader - VBAT battery charging enable
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
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
///Field `VBE` writer - VBAT battery charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT battery charging disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    ///VBAT battery charging enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
/**VBAT battery charging resistor selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    ///0: VBAT charging through a 5 kΩ resistor
    R5k = 0,
    ///1: VBAT charging through a 1.5 kΩ resistor
    R1_5k = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
///Field `VBRS` reader - VBAT battery charging resistor selection
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::R5k,
            true => VBRS::R1_5k,
        }
    }
    ///VBAT charging through a 5 kΩ resistor
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == VBRS::R5k
    }
    ///VBAT charging through a 1.5 kΩ resistor
    #[inline(always)]
    pub fn is_r1_5k(&self) -> bool {
        *self == VBRS::R1_5k
    }
}
///Field `VBRS` writer - VBAT battery charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBAT charging through a 5 kΩ resistor
    #[inline(always)]
    pub fn r5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R5k)
    }
    ///VBAT charging through a 1.5 kΩ resistor
    #[inline(always)]
    pub fn r1_5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R1_5k)
    }
}
/**Wakeup Radio BUSY polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYP {
    ///0: Detection on high level (rising edge)
    RisingEdge = 0,
    ///1: Detection on low level (falling edge)
    FallingEdge = 1,
}
impl From<WRFBUSYP> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYP) -> Self {
        variant as u8 != 0
    }
}
///Field `WRFBUSYP` reader - Wakeup Radio BUSY polarity
pub type WRFBUSYP_R = crate::BitReader<WRFBUSYP>;
impl WRFBUSYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRFBUSYP {
        match self.bits {
            false => WRFBUSYP::RisingEdge,
            true => WRFBUSYP::FallingEdge,
        }
    }
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WRFBUSYP::RisingEdge
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WRFBUSYP::FallingEdge
    }
}
///Field `WRFBUSYP` writer - Wakeup Radio BUSY polarity
pub type WRFBUSYP_W<'a, REG> = crate::BitWriter<'a, REG, WRFBUSYP>;
impl<'a, REG> WRFBUSYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection on high level (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WRFBUSYP::RisingEdge)
    }
    ///Detection on low level (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WRFBUSYP::FallingEdge)
    }
}
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wakeup Radio BUSY polarity
    #[inline(always)]
    pub fn wrfbusyp(&self) -> WRFBUSYP_R {
        WRFBUSYP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("wrfbusyp", &self.wrfbusyp())
            .field("vbrs", &self.vbrs())
            .field("vbe", &self.vbe())
            .field("wp3", &self.wp3())
            .field("wp2", &self.wp2())
            .field("wp1", &self.wp1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W<'_, CR4rs> {
        WP1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W<'_, CR4rs> {
        WP2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W<'_, CR4rs> {
        WP3_W::new(self, 2)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, CR4rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, CR4rs> {
        VBRS_W::new(self, 9)
    }
    ///Bit 11 - Wakeup Radio BUSY polarity
    #[inline(always)]
    pub fn wrfbusyp(&mut self) -> WRFBUSYP_W<'_, CR4rs> {
        WRFBUSYP_W::new(self, 11)
    }
}
/**Power control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:CR4)*/
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
