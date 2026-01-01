///Register `RTSR1` reader
pub type R = crate::R<RTSR1rs>;
///Register `RTSR1` writer
pub type W = crate::W<RTSR1rs>;
/**Rising trigger event configuration bit of Configurable Event line

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISING_TRIGGER {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<RISING_TRIGGER> for bool {
    #[inline(always)]
    fn from(variant: RISING_TRIGGER) -> Self {
        variant as u8 != 0
    }
}
///Field `RT0` reader - Rising trigger event configuration bit of Configurable Event line
pub type RT0_R = crate::BitReader<RISING_TRIGGER>;
impl RT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RISING_TRIGGER {
        match self.bits {
            false => RISING_TRIGGER::Disabled,
            true => RISING_TRIGGER::Enabled,
        }
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RISING_TRIGGER::Disabled
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RISING_TRIGGER::Enabled
    }
}
///Field `RT0` writer - Rising trigger event configuration bit of Configurable Event line
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG, RISING_TRIGGER>;
impl<'a, REG> RT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RISING_TRIGGER::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RISING_TRIGGER::Enabled)
    }
}
///Field `RT1` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT1_R;
///Field `RT2` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT2_R;
///Field `RT3` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT3_R;
///Field `RT4` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT4_R;
///Field `RT5` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT5_R;
///Field `RT6` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT6_R;
///Field `RT7` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT7_R;
///Field `RT8` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT8_R;
///Field `RT9` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT9_R;
///Field `RT10` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT10_R;
///Field `RT11` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT11_R;
///Field `RT12` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT12_R;
///Field `RT13` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT13_R;
///Field `RT14` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT14_R;
///Field `RT15` reader - Rising trigger event configuration bit of Configurable Event line
pub use RT0_R as RT15_R;
///Field `RT1` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT1_W;
///Field `RT2` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT2_W;
///Field `RT3` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT3_W;
///Field `RT4` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT4_W;
///Field `RT5` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT5_W;
///Field `RT6` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT6_W;
///Field `RT7` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT7_W;
///Field `RT8` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT8_W;
///Field `RT9` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT9_W;
///Field `RT10` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT10_W;
///Field `RT11` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT11_W;
///Field `RT12` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT12_W;
///Field `RT13` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT13_W;
///Field `RT14` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT14_W;
///Field `RT15` writer - Rising trigger event configuration bit of Configurable Event line
pub use RT0_W as RT15_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR1")
            .field("rt0", &self.rt0())
            .field("rt1", &self.rt1())
            .field("rt2", &self.rt2())
            .field("rt3", &self.rt3())
            .field("rt4", &self.rt4())
            .field("rt5", &self.rt5())
            .field("rt6", &self.rt6())
            .field("rt7", &self.rt7())
            .field("rt8", &self.rt8())
            .field("rt9", &self.rt9())
            .field("rt10", &self.rt10())
            .field("rt11", &self.rt11())
            .field("rt12", &self.rt12())
            .field("rt13", &self.rt13())
            .field("rt14", &self.rt14())
            .field("rt15", &self.rt15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W<'_, RTSR1rs> {
        RT0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W<'_, RTSR1rs> {
        RT1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W<'_, RTSR1rs> {
        RT2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W<'_, RTSR1rs> {
        RT3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W<'_, RTSR1rs> {
        RT4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W<'_, RTSR1rs> {
        RT5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W<'_, RTSR1rs> {
        RT6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W<'_, RTSR1rs> {
        RT7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W<'_, RTSR1rs> {
        RT8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W<'_, RTSR1rs> {
        RT9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W<'_, RTSR1rs> {
        RT10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W<'_, RTSR1rs> {
        RT11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W<'_, RTSR1rs> {
        RT12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W<'_, RTSR1rs> {
        RT13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W<'_, RTSR1rs> {
        RT14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event line
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W<'_, RTSR1rs> {
        RT15_W::new(self, 15)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#EXTI:RTSR1)*/
pub struct RTSR1rs;
impl crate::RegisterSpec for RTSR1rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr1::R`](R) reader structure
impl crate::Readable for RTSR1rs {}
///`write(|w| ..)` method takes [`rtsr1::W`](W) writer structure
impl crate::Writable for RTSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR1 to value 0
impl crate::Resettable for RTSR1rs {}
