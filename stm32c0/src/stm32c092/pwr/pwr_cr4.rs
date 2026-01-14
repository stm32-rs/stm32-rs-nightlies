///Register `PWR_CR4` reader
pub type R = crate::R<PWR_CR4rs>;
///Register `PWR_CR4` writer
pub type W = crate::W<PWR_CR4rs>;
/**WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP1> for bool {
    #[inline(always)]
    fn from(variant: WP1) -> Self {
        variant as u8 != 0
    }
}
///Field `WP1` reader - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP1_R = crate::BitReader<WP1>;
impl WP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP1 {
        match self.bits {
            false => WP1::B0x0,
            true => WP1::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP1::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP1::B0x1
    }
}
///Field `WP1` writer - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG, WP1>;
impl<'a, REG> WP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::B0x1)
    }
}
/**WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP2> for bool {
    #[inline(always)]
    fn from(variant: WP2) -> Self {
        variant as u8 != 0
    }
}
///Field `WP2` reader - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP2_R = crate::BitReader<WP2>;
impl WP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP2 {
        match self.bits {
            false => WP2::B0x0,
            true => WP2::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP2::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP2::B0x1
    }
}
///Field `WP2` writer - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG, WP2>;
impl<'a, REG> WP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::B0x1)
    }
}
/**WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP3> for bool {
    #[inline(always)]
    fn from(variant: WP3) -> Self {
        variant as u8 != 0
    }
}
///Field `WP3` reader - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP3_R = crate::BitReader<WP3>;
impl WP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP3 {
        match self.bits {
            false => WP3::B0x0,
            true => WP3::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP3::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP3::B0x1
    }
}
///Field `WP3` writer - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG, WP3>;
impl<'a, REG> WP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::B0x1)
    }
}
/**WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP4 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP4> for bool {
    #[inline(always)]
    fn from(variant: WP4) -> Self {
        variant as u8 != 0
    }
}
///Field `WP4` reader - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP4_R = crate::BitReader<WP4>;
impl WP4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP4 {
        match self.bits {
            false => WP4::B0x0,
            true => WP4::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP4::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP4::B0x1
    }
}
///Field `WP4` writer - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP4_W<'a, REG> = crate::BitWriter<'a, REG, WP4>;
impl<'a, REG> WP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP4::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP4::B0x1)
    }
}
/**WKUP5 wakeup pin polarity WKUP5 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP5 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP5> for bool {
    #[inline(always)]
    fn from(variant: WP5) -> Self {
        variant as u8 != 0
    }
}
///Field `WP5` reader - WKUP5 wakeup pin polarity WKUP5 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP5_R = crate::BitReader<WP5>;
impl WP5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP5 {
        match self.bits {
            false => WP5::B0x0,
            true => WP5::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP5::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP5::B0x1
    }
}
///Field `WP5` writer - WKUP5 wakeup pin polarity WKUP5 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP5_W<'a, REG> = crate::BitWriter<'a, REG, WP5>;
impl<'a, REG> WP5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP5::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP5::B0x1)
    }
}
/**WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP6 {
    ///0: High level or rising edge
    B0x0 = 0,
    ///1: Low level or falling edge
    B0x1 = 1,
}
impl From<WP6> for bool {
    #[inline(always)]
    fn from(variant: WP6) -> Self {
        variant as u8 != 0
    }
}
///Field `WP6` reader - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP6_R = crate::BitReader<WP6>;
impl WP6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP6 {
        match self.bits {
            false => WP6::B0x0,
            true => WP6::B0x1,
        }
    }
    ///High level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WP6::B0x0
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WP6::B0x1
    }
}
///Field `WP6` writer - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:
pub type WP6_W<'a, REG> = crate::BitWriter<'a, REG, WP6>;
impl<'a, REG> WP6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WP6::B0x0)
    }
    ///Low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WP6::B0x1)
    }
}
impl R {
    ///Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WKUP5 wakeup pin polarity WKUP5 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR4")
            .field("wp1", &self.wp1())
            .field("wp2", &self.wp2())
            .field("wp3", &self.wp3())
            .field("wp4", &self.wp4())
            .field("wp5", &self.wp5())
            .field("wp6", &self.wp6())
            .finish()
    }
}
impl W {
    ///Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W<'_, PWR_CR4rs> {
        WP1_W::new(self, 0)
    }
    ///Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W<'_, PWR_CR4rs> {
        WP2_W::new(self, 1)
    }
    ///Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W<'_, PWR_CR4rs> {
        WP3_W::new(self, 2)
    }
    ///Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp4(&mut self) -> WP4_W<'_, PWR_CR4rs> {
        WP4_W::new(self, 3)
    }
    ///Bit 4 - WKUP5 wakeup pin polarity WKUP5 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W<'_, PWR_CR4rs> {
        WP5_W::new(self, 4)
    }
    ///Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:
    #[inline(always)]
    pub fn wp6(&mut self) -> WP6_W<'_, PWR_CR4rs> {
        WP6_W::new(self, 5)
    }
}
/**PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#PWR:PWR_CR4)*/
pub struct PWR_CR4rs;
impl crate::RegisterSpec for PWR_CR4rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr4::R`](R) reader structure
impl crate::Readable for PWR_CR4rs {}
///`write(|w| ..)` method takes [`pwr_cr4::W`](W) writer structure
impl crate::Writable for PWR_CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR_CR4 to value 0
impl crate::Resettable for PWR_CR4rs {}
