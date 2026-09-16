///Register `PR1` reader
pub type R = crate::R<PR1rs>;
///Register `PR1` writer
pub type W = crate::W<PR1rs>;
///Field `PIF0` reader - Configurable event inputs Pending bit
pub type PIF0_R = crate::BitReader;
///Field `PIF0` writer - Configurable event inputs Pending bit
pub type PIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF1` reader - Configurable event inputs Pending bit
pub type PIF1_R = crate::BitReader;
///Field `PIF1` writer - Configurable event inputs Pending bit
pub type PIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF2` reader - Configurable event inputs Pending bit
pub type PIF2_R = crate::BitReader;
///Field `PIF2` writer - Configurable event inputs Pending bit
pub type PIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF3` reader - Configurable event inputs Pending bit
pub type PIF3_R = crate::BitReader;
///Field `PIF3` writer - Configurable event inputs Pending bit
pub type PIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF4` reader - Configurable event inputs Pending bit
pub type PIF4_R = crate::BitReader;
///Field `PIF4` writer - Configurable event inputs Pending bit
pub type PIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF5` reader - Configurable event inputs Pending bit
pub type PIF5_R = crate::BitReader;
///Field `PIF5` writer - Configurable event inputs Pending bit
pub type PIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF6` reader - Configurable event inputs Pending bit
pub type PIF6_R = crate::BitReader;
///Field `PIF6` writer - Configurable event inputs Pending bit
pub type PIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF7` reader - Configurable event inputs Pending bit
pub type PIF7_R = crate::BitReader;
///Field `PIF7` writer - Configurable event inputs Pending bit
pub type PIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF8` reader - Configurable event inputs Pending bit
pub type PIF8_R = crate::BitReader;
///Field `PIF8` writer - Configurable event inputs Pending bit
pub type PIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF9` reader - Configurable event inputs Pending bit
pub type PIF9_R = crate::BitReader;
///Field `PIF9` writer - Configurable event inputs Pending bit
pub type PIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF10` reader - Configurable event inputs Pending bit
pub type PIF10_R = crate::BitReader;
///Field `PIF10` writer - Configurable event inputs Pending bit
pub type PIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF11` reader - Configurable event inputs Pending bit
pub type PIF11_R = crate::BitReader;
///Field `PIF11` writer - Configurable event inputs Pending bit
pub type PIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF12` reader - Configurable event inputs Pending bit
pub type PIF12_R = crate::BitReader;
///Field `PIF12` writer - Configurable event inputs Pending bit
pub type PIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF13` reader - Configurable event inputs Pending bit
pub type PIF13_R = crate::BitReader;
///Field `PIF13` writer - Configurable event inputs Pending bit
pub type PIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF14` reader - Configurable event inputs Pending bit
pub type PIF14_R = crate::BitReader;
///Field `PIF14` writer - Configurable event inputs Pending bit
pub type PIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF15` reader - Configurable event inputs Pending bit
pub type PIF15_R = crate::BitReader;
///Field `PIF15` writer - Configurable event inputs Pending bit
pub type PIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF16` reader - Configurable event inputs Pending bit
pub type PIF16_R = crate::BitReader;
///Field `PIF16` writer - Configurable event inputs Pending bit
pub type PIF16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF17` reader - Configurable event inputs Pending bit
pub type PIF17_R = crate::BitReader;
///Field `PIF17` writer - Configurable event inputs Pending bit
pub type PIF17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF18` reader - Configurable event inputs Pending bit
pub type PIF18_R = crate::BitReader;
///Field `PIF18` writer - Configurable event inputs Pending bit
pub type PIF18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF19` reader - Configurable event inputs Pending bit
pub type PIF19_R = crate::BitReader;
///Field `PIF19` writer - Configurable event inputs Pending bit
pub type PIF19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF20` reader - Configurable event inputs Pending bit
pub type PIF20_R = crate::BitReader;
///Field `PIF20` writer - Configurable event inputs Pending bit
pub type PIF20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF21` reader - Configurable event inputs Pending bit
pub type PIF21_R = crate::BitReader;
///Field `PIF21` writer - Configurable event inputs Pending bit
pub type PIF21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF31` reader - Configurable event inputs Pending bit
pub type PIF31_R = crate::BitReader;
///Field `PIF31` writer - Configurable event inputs Pending bit
pub type PIF31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif18(&self) -> PIF18_R {
        PIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif31(&self) -> PIF31_R {
        PIF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR1")
            .field("pif31", &self.pif31())
            .field("pif0", &self.pif0())
            .field("pif1", &self.pif1())
            .field("pif2", &self.pif2())
            .field("pif3", &self.pif3())
            .field("pif4", &self.pif4())
            .field("pif5", &self.pif5())
            .field("pif6", &self.pif6())
            .field("pif7", &self.pif7())
            .field("pif8", &self.pif8())
            .field("pif9", &self.pif9())
            .field("pif10", &self.pif10())
            .field("pif11", &self.pif11())
            .field("pif12", &self.pif12())
            .field("pif13", &self.pif13())
            .field("pif14", &self.pif14())
            .field("pif15", &self.pif15())
            .field("pif16", &self.pif16())
            .field("pif17", &self.pif17())
            .field("pif18", &self.pif18())
            .field("pif19", &self.pif19())
            .field("pif20", &self.pif20())
            .field("pif21", &self.pif21())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W<'_, PR1rs> {
        PIF0_W::new(self, 0)
    }
    ///Bit 1 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif1(&mut self) -> PIF1_W<'_, PR1rs> {
        PIF1_W::new(self, 1)
    }
    ///Bit 2 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W<'_, PR1rs> {
        PIF2_W::new(self, 2)
    }
    ///Bit 3 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif3(&mut self) -> PIF3_W<'_, PR1rs> {
        PIF3_W::new(self, 3)
    }
    ///Bit 4 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W<'_, PR1rs> {
        PIF4_W::new(self, 4)
    }
    ///Bit 5 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif5(&mut self) -> PIF5_W<'_, PR1rs> {
        PIF5_W::new(self, 5)
    }
    ///Bit 6 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif6(&mut self) -> PIF6_W<'_, PR1rs> {
        PIF6_W::new(self, 6)
    }
    ///Bit 7 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif7(&mut self) -> PIF7_W<'_, PR1rs> {
        PIF7_W::new(self, 7)
    }
    ///Bit 8 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif8(&mut self) -> PIF8_W<'_, PR1rs> {
        PIF8_W::new(self, 8)
    }
    ///Bit 9 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif9(&mut self) -> PIF9_W<'_, PR1rs> {
        PIF9_W::new(self, 9)
    }
    ///Bit 10 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif10(&mut self) -> PIF10_W<'_, PR1rs> {
        PIF10_W::new(self, 10)
    }
    ///Bit 11 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif11(&mut self) -> PIF11_W<'_, PR1rs> {
        PIF11_W::new(self, 11)
    }
    ///Bit 12 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif12(&mut self) -> PIF12_W<'_, PR1rs> {
        PIF12_W::new(self, 12)
    }
    ///Bit 13 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif13(&mut self) -> PIF13_W<'_, PR1rs> {
        PIF13_W::new(self, 13)
    }
    ///Bit 14 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif14(&mut self) -> PIF14_W<'_, PR1rs> {
        PIF14_W::new(self, 14)
    }
    ///Bit 15 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif15(&mut self) -> PIF15_W<'_, PR1rs> {
        PIF15_W::new(self, 15)
    }
    ///Bit 16 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif16(&mut self) -> PIF16_W<'_, PR1rs> {
        PIF16_W::new(self, 16)
    }
    ///Bit 17 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif17(&mut self) -> PIF17_W<'_, PR1rs> {
        PIF17_W::new(self, 17)
    }
    ///Bit 18 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif18(&mut self) -> PIF18_W<'_, PR1rs> {
        PIF18_W::new(self, 18)
    }
    ///Bit 19 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif19(&mut self) -> PIF19_W<'_, PR1rs> {
        PIF19_W::new(self, 19)
    }
    ///Bit 20 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif20(&mut self) -> PIF20_W<'_, PR1rs> {
        PIF20_W::new(self, 20)
    }
    ///Bit 21 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W<'_, PR1rs> {
        PIF21_W::new(self, 21)
    }
    ///Bit 31 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif31(&mut self) -> PIF31_W<'_, PR1rs> {
        PIF31_W::new(self, 31)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:PR1)*/
pub struct PR1rs;
impl crate::RegisterSpec for PR1rs {
    type Ux = u32;
}
///`read()` method returns [`pr1::R`](R) reader structure
impl crate::Readable for PR1rs {}
///`write(|w| ..)` method takes [`pr1::W`](W) writer structure
impl crate::Writable for PR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PR1 to value 0
impl crate::Resettable for PR1rs {}
