///Register `RTSR` reader
pub type R = crate::R<RTSRrs>;
///Register `RTSR` writer
pub type W = crate::W<RTSRrs>;
/**Rising trigger event configuration of line 0

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
///Field `RT0` reader - Rising trigger event configuration of line 0
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
///Field `RT0` writer - Rising trigger event configuration of line 0
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
///Field `RT1` reader - Rising trigger event configuration of line 1
pub use RT0_R as RT1_R;
///Field `RT2` reader - Rising trigger event configuration of line 2
pub use RT0_R as RT2_R;
///Field `RT3` reader - Rising trigger event configuration of line 3
pub use RT0_R as RT3_R;
///Field `RT4` reader - Rising trigger event configuration of line 4
pub use RT0_R as RT4_R;
///Field `RT5` reader - Rising trigger event configuration of line 5
pub use RT0_R as RT5_R;
///Field `RT6` reader - Rising trigger event configuration of line 6
pub use RT0_R as RT6_R;
///Field `RT7` reader - Rising trigger event configuration of line 7
pub use RT0_R as RT7_R;
///Field `RT8` reader - Rising trigger event configuration of line 8
pub use RT0_R as RT8_R;
///Field `RT9` reader - Rising trigger event configuration of line 9
pub use RT0_R as RT9_R;
///Field `RT10` reader - Rising trigger event configuration of line 10
pub use RT0_R as RT10_R;
///Field `RT11` reader - Rising trigger event configuration of line 11
pub use RT0_R as RT11_R;
///Field `RT12` reader - Rising trigger event configuration of line 12
pub use RT0_R as RT12_R;
///Field `RT13` reader - Rising trigger event configuration of line 13
pub use RT0_R as RT13_R;
///Field `RT14` reader - Rising trigger event configuration of line 14
pub use RT0_R as RT14_R;
///Field `RT15` reader - Rising trigger event configuration of line 15
pub use RT0_R as RT15_R;
///Field `RT16` reader - Rising trigger event configuration of line 16
pub use RT0_R as RT16_R;
///Field `RT17` reader - Rising trigger event configuration of line 17
pub use RT0_R as RT17_R;
///Field `RT19` reader - Rising trigger event configuration of line 19
pub use RT0_R as RT19_R;
///Field `RT20` reader - Rising trigger event configuration of line 20
pub use RT0_R as RT20_R;
///Field `RT21` reader - Rising trigger event configuration of line 21
pub use RT0_R as RT21_R;
///Field `RT22` reader - Rising trigger event configuration of line 22
pub use RT0_R as RT22_R;
///Field `RT1` writer - Rising trigger event configuration of line 1
pub use RT0_W as RT1_W;
///Field `RT2` writer - Rising trigger event configuration of line 2
pub use RT0_W as RT2_W;
///Field `RT3` writer - Rising trigger event configuration of line 3
pub use RT0_W as RT3_W;
///Field `RT4` writer - Rising trigger event configuration of line 4
pub use RT0_W as RT4_W;
///Field `RT5` writer - Rising trigger event configuration of line 5
pub use RT0_W as RT5_W;
///Field `RT6` writer - Rising trigger event configuration of line 6
pub use RT0_W as RT6_W;
///Field `RT7` writer - Rising trigger event configuration of line 7
pub use RT0_W as RT7_W;
///Field `RT8` writer - Rising trigger event configuration of line 8
pub use RT0_W as RT8_W;
///Field `RT9` writer - Rising trigger event configuration of line 9
pub use RT0_W as RT9_W;
///Field `RT10` writer - Rising trigger event configuration of line 10
pub use RT0_W as RT10_W;
///Field `RT11` writer - Rising trigger event configuration of line 11
pub use RT0_W as RT11_W;
///Field `RT12` writer - Rising trigger event configuration of line 12
pub use RT0_W as RT12_W;
///Field `RT13` writer - Rising trigger event configuration of line 13
pub use RT0_W as RT13_W;
///Field `RT14` writer - Rising trigger event configuration of line 14
pub use RT0_W as RT14_W;
///Field `RT15` writer - Rising trigger event configuration of line 15
pub use RT0_W as RT15_W;
///Field `RT16` writer - Rising trigger event configuration of line 16
pub use RT0_W as RT16_W;
///Field `RT17` writer - Rising trigger event configuration of line 17
pub use RT0_W as RT17_W;
///Field `RT19` writer - Rising trigger event configuration of line 19
pub use RT0_W as RT19_W;
///Field `RT20` writer - Rising trigger event configuration of line 20
pub use RT0_W as RT20_W;
///Field `RT21` writer - Rising trigger event configuration of line 21
pub use RT0_W as RT21_W;
///Field `RT22` writer - Rising trigger event configuration of line 22
pub use RT0_W as RT22_W;
impl R {
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    pub fn rt17(&self) -> RT17_R {
        RT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    pub fn rt19(&self) -> RT19_R {
        RT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    pub fn rt20(&self) -> RT20_R {
        RT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR")
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
            .field("rt17", &self.rt17())
            .field("rt19", &self.rt19())
            .field("rt20", &self.rt20())
            .field("rt21", &self.rt21())
            .field("rt22", &self.rt22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W<RTSRrs> {
        RT0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W<RTSRrs> {
        RT1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W<RTSRrs> {
        RT2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W<RTSRrs> {
        RT3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W<RTSRrs> {
        RT4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W<RTSRrs> {
        RT5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W<RTSRrs> {
        RT6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W<RTSRrs> {
        RT7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W<RTSRrs> {
        RT8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W<RTSRrs> {
        RT9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W<RTSRrs> {
        RT10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W<RTSRrs> {
        RT11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W<RTSRrs> {
        RT12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W<RTSRrs> {
        RT13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W<RTSRrs> {
        RT14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W<RTSRrs> {
        RT15_W::new(self, 15)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W<RTSRrs> {
        RT16_W::new(self, 16)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    pub fn rt17(&mut self) -> RT17_W<RTSRrs> {
        RT17_W::new(self, 17)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    pub fn rt19(&mut self) -> RT19_W<RTSRrs> {
        RT19_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    pub fn rt20(&mut self) -> RT20_W<RTSRrs> {
        RT20_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W<RTSRrs> {
        RT21_W::new(self, 21)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    pub fn rt22(&mut self) -> RT22_W<RTSRrs> {
        RT22_W::new(self, 22)
    }
}
/**Rising Trigger selection register (EXTI_RTSR)

You can [`read`](crate::Reg::read) this register and get [`rtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#EXTI:RTSR)*/
pub struct RTSRrs;
impl crate::RegisterSpec for RTSRrs {
    type Ux = u32;
}
///`read()` method returns [`rtsr::R`](R) reader structure
impl crate::Readable for RTSRrs {}
///`write(|w| ..)` method takes [`rtsr::W`](W) writer structure
impl crate::Writable for RTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR to value 0
impl crate::Resettable for RTSRrs {}
