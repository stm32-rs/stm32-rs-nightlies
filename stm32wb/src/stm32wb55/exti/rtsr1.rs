///Register `RTSR1` reader
pub type R = crate::R<RTSR1rs>;
///Register `RTSR1` writer
pub type W = crate::W<RTSR1rs>;
///Field `RT0` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT0_R = crate::BitReader;
///Field `RT0` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT1` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT1_R = crate::BitReader;
///Field `RT1` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT2` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT2_R = crate::BitReader;
///Field `RT2` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT3` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT3_R = crate::BitReader;
///Field `RT3` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT4` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT4_R = crate::BitReader;
///Field `RT4` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT5` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT5_R = crate::BitReader;
///Field `RT5` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT6` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT6_R = crate::BitReader;
///Field `RT6` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT7` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT7_R = crate::BitReader;
///Field `RT7` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT8` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT8_R = crate::BitReader;
///Field `RT8` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT9` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT9_R = crate::BitReader;
///Field `RT9` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT10` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT10_R = crate::BitReader;
///Field `RT10` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT11` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT11_R = crate::BitReader;
///Field `RT11` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT12` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT12_R = crate::BitReader;
///Field `RT12` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT13` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT13_R = crate::BitReader;
///Field `RT13` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT14` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT14_R = crate::BitReader;
///Field `RT14` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT15` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT15_R = crate::BitReader;
///Field `RT15` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT16` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT16_R = crate::BitReader;
///Field `RT16` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT17` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT17_R = crate::BitReader;
///Field `RT17` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT18` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT18_R = crate::BitReader;
///Field `RT18` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT19` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT19_R = crate::BitReader;
///Field `RT19` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT20` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT20_R = crate::BitReader;
///Field `RT20` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT21` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT21_R = crate::BitReader;
///Field `RT21` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT31` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT31_R = crate::BitReader;
///Field `RT31` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt17(&self) -> RT17_R {
        RT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt18(&self) -> RT18_R {
        RT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt19(&self) -> RT19_R {
        RT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt20(&self) -> RT20_R {
        RT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt31(&self) -> RT31_R {
        RT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR1")
            .field("rt31", &self.rt31())
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
            .field("rt18", &self.rt18())
            .field("rt19", &self.rt19())
            .field("rt20", &self.rt20())
            .field("rt21", &self.rt21())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W<RTSR1rs> {
        RT0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W<RTSR1rs> {
        RT1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W<RTSR1rs> {
        RT2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W<RTSR1rs> {
        RT3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W<RTSR1rs> {
        RT4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W<RTSR1rs> {
        RT5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W<RTSR1rs> {
        RT6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W<RTSR1rs> {
        RT7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W<RTSR1rs> {
        RT8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W<RTSR1rs> {
        RT9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W<RTSR1rs> {
        RT10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W<RTSR1rs> {
        RT11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W<RTSR1rs> {
        RT12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W<RTSR1rs> {
        RT13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W<RTSR1rs> {
        RT14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W<RTSR1rs> {
        RT15_W::new(self, 15)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W<RTSR1rs> {
        RT16_W::new(self, 16)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt17(&mut self) -> RT17_W<RTSR1rs> {
        RT17_W::new(self, 17)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt18(&mut self) -> RT18_W<RTSR1rs> {
        RT18_W::new(self, 18)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt19(&mut self) -> RT19_W<RTSR1rs> {
        RT19_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt20(&mut self) -> RT20_W<RTSR1rs> {
        RT20_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W<RTSR1rs> {
        RT21_W::new(self, 21)
    }
    ///Bit 31 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt31(&mut self) -> RT31_W<RTSR1rs> {
        RT31_W::new(self, 31)
    }
}
/**rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:RTSR1)*/
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
