///Register `RTSR1` reader
pub type R = crate::R<RTSR1rs>;
///Register `RTSR1` writer
pub type W = crate::W<RTSR1rs>;
/**rising trigger event configuration bit of configurable event input 0

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
///Field `RT0` reader - rising trigger event configuration bit of configurable event input 0
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
///Field `RT0` writer - rising trigger event configuration bit of configurable event input 0
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
///Field `RT1` reader - rising trigger event configuration bit of configurable event input 1
pub use RT0_R as RT1_R;
///Field `RT2` reader - rising trigger event configuration bit of configurable event input 2
pub use RT0_R as RT2_R;
///Field `RT3` reader - rising trigger event configuration bit of configurable event input 3
pub use RT0_R as RT3_R;
///Field `RT4` reader - rising trigger event configuration bit of configurable event input 4
pub use RT0_R as RT4_R;
///Field `RT5` reader - rising trigger event configuration bit of configurable event input 5
pub use RT0_R as RT5_R;
///Field `RT6` reader - rising trigger event configuration bit of configurable event input 6
pub use RT0_R as RT6_R;
///Field `RT7` reader - rising trigger event configuration bit of configurable event input 7
pub use RT0_R as RT7_R;
///Field `RT8` reader - rising trigger event configuration bit of configurable event input 8
pub use RT0_R as RT8_R;
///Field `RT9` reader - rising trigger event configuration bit of configurable event input 9
pub use RT0_R as RT9_R;
///Field `RT10` reader - rising trigger event configuration bit of configurable event input 10
pub use RT0_R as RT10_R;
///Field `RT11` reader - rising trigger event configuration bit of configurable event input 11
pub use RT0_R as RT11_R;
///Field `RT12` reader - rising trigger event configuration bit of configurable event input 12
pub use RT0_R as RT12_R;
///Field `RT13` reader - rising trigger event configuration bit of configurable event input 13
pub use RT0_R as RT13_R;
///Field `RT14` reader - rising trigger event configuration bit of configurable event input 14
pub use RT0_R as RT14_R;
///Field `RT15` reader - rising trigger event configuration bit of configurable event input 15
pub use RT0_R as RT15_R;
///Field `RT16` reader - rising trigger event configuration bit of configurable event input 16
pub use RT0_R as RT16_R;
///Field `RT21` reader - rising trigger event configuration bit of configurable event input 21
pub use RT0_R as RT21_R;
///Field `RT22` reader - rising trigger event configuration bit of configurable event input 22 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
pub use RT0_R as RT22_R;
///Field `RT1` writer - rising trigger event configuration bit of configurable event input 1
pub use RT0_W as RT1_W;
///Field `RT2` writer - rising trigger event configuration bit of configurable event input 2
pub use RT0_W as RT2_W;
///Field `RT3` writer - rising trigger event configuration bit of configurable event input 3
pub use RT0_W as RT3_W;
///Field `RT4` writer - rising trigger event configuration bit of configurable event input 4
pub use RT0_W as RT4_W;
///Field `RT5` writer - rising trigger event configuration bit of configurable event input 5
pub use RT0_W as RT5_W;
///Field `RT6` writer - rising trigger event configuration bit of configurable event input 6
pub use RT0_W as RT6_W;
///Field `RT7` writer - rising trigger event configuration bit of configurable event input 7
pub use RT0_W as RT7_W;
///Field `RT8` writer - rising trigger event configuration bit of configurable event input 8
pub use RT0_W as RT8_W;
///Field `RT9` writer - rising trigger event configuration bit of configurable event input 9
pub use RT0_W as RT9_W;
///Field `RT10` writer - rising trigger event configuration bit of configurable event input 10
pub use RT0_W as RT10_W;
///Field `RT11` writer - rising trigger event configuration bit of configurable event input 11
pub use RT0_W as RT11_W;
///Field `RT12` writer - rising trigger event configuration bit of configurable event input 12
pub use RT0_W as RT12_W;
///Field `RT13` writer - rising trigger event configuration bit of configurable event input 13
pub use RT0_W as RT13_W;
///Field `RT14` writer - rising trigger event configuration bit of configurable event input 14
pub use RT0_W as RT14_W;
///Field `RT15` writer - rising trigger event configuration bit of configurable event input 15
pub use RT0_W as RT15_W;
///Field `RT16` writer - rising trigger event configuration bit of configurable event input 16
pub use RT0_W as RT16_W;
///Field `RT21` writer - rising trigger event configuration bit of configurable event input 21
pub use RT0_W as RT21_W;
///Field `RT22` writer - rising trigger event configuration bit of configurable event input 22 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
pub use RT0_W as RT22_W;
impl R {
    ///Bit 0 - rising trigger event configuration bit of configurable event input 0
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rising trigger event configuration bit of configurable event input 1
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - rising trigger event configuration bit of configurable event input 2
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - rising trigger event configuration bit of configurable event input 3
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - rising trigger event configuration bit of configurable event input 4
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - rising trigger event configuration bit of configurable event input 5
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - rising trigger event configuration bit of configurable event input 6
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - rising trigger event configuration bit of configurable event input 7
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - rising trigger event configuration bit of configurable event input 8
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - rising trigger event configuration bit of configurable event input 9
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - rising trigger event configuration bit of configurable event input 10
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - rising trigger event configuration bit of configurable event input 11
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - rising trigger event configuration bit of configurable event input 12
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - rising trigger event configuration bit of configurable event input 13
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - rising trigger event configuration bit of configurable event input 14
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - rising trigger event configuration bit of configurable event input 15
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - rising trigger event configuration bit of configurable event input 16
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - rising trigger event configuration bit of configurable event input 21
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - rising trigger event configuration bit of configurable event input 22 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 1) != 0)
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
            .field("rt16", &self.rt16())
            .field("rt21", &self.rt21())
            .field("rt22", &self.rt22())
            .finish()
    }
}
impl W {
    ///Bit 0 - rising trigger event configuration bit of configurable event input 0
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W<'_, RTSR1rs> {
        RT0_W::new(self, 0)
    }
    ///Bit 1 - rising trigger event configuration bit of configurable event input 1
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W<'_, RTSR1rs> {
        RT1_W::new(self, 1)
    }
    ///Bit 2 - rising trigger event configuration bit of configurable event input 2
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W<'_, RTSR1rs> {
        RT2_W::new(self, 2)
    }
    ///Bit 3 - rising trigger event configuration bit of configurable event input 3
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W<'_, RTSR1rs> {
        RT3_W::new(self, 3)
    }
    ///Bit 4 - rising trigger event configuration bit of configurable event input 4
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W<'_, RTSR1rs> {
        RT4_W::new(self, 4)
    }
    ///Bit 5 - rising trigger event configuration bit of configurable event input 5
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W<'_, RTSR1rs> {
        RT5_W::new(self, 5)
    }
    ///Bit 6 - rising trigger event configuration bit of configurable event input 6
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W<'_, RTSR1rs> {
        RT6_W::new(self, 6)
    }
    ///Bit 7 - rising trigger event configuration bit of configurable event input 7
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W<'_, RTSR1rs> {
        RT7_W::new(self, 7)
    }
    ///Bit 8 - rising trigger event configuration bit of configurable event input 8
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W<'_, RTSR1rs> {
        RT8_W::new(self, 8)
    }
    ///Bit 9 - rising trigger event configuration bit of configurable event input 9
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W<'_, RTSR1rs> {
        RT9_W::new(self, 9)
    }
    ///Bit 10 - rising trigger event configuration bit of configurable event input 10
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W<'_, RTSR1rs> {
        RT10_W::new(self, 10)
    }
    ///Bit 11 - rising trigger event configuration bit of configurable event input 11
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W<'_, RTSR1rs> {
        RT11_W::new(self, 11)
    }
    ///Bit 12 - rising trigger event configuration bit of configurable event input 12
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W<'_, RTSR1rs> {
        RT12_W::new(self, 12)
    }
    ///Bit 13 - rising trigger event configuration bit of configurable event input 13
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W<'_, RTSR1rs> {
        RT13_W::new(self, 13)
    }
    ///Bit 14 - rising trigger event configuration bit of configurable event input 14
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W<'_, RTSR1rs> {
        RT14_W::new(self, 14)
    }
    ///Bit 15 - rising trigger event configuration bit of configurable event input 15
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W<'_, RTSR1rs> {
        RT15_W::new(self, 15)
    }
    ///Bit 16 - rising trigger event configuration bit of configurable event input 16
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W<'_, RTSR1rs> {
        RT16_W::new(self, 16)
    }
    ///Bit 21 - rising trigger event configuration bit of configurable event input 21
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W<'_, RTSR1rs> {
        RT21_W::new(self, 21)
    }
    ///Bit 22 - rising trigger event configuration bit of configurable event input 22 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt22(&mut self) -> RT22_W<'_, RTSR1rs> {
        RT22_W::new(self, 22)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:RTSR1)*/
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
