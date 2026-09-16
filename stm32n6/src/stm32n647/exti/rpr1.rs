///Register `RPR1` reader
pub type R = crate::R<RPR1rs>;
///Register `RPR1` writer
pub type W = crate::W<RPR1rs>;
///Field `RPIF0` reader - Configurable event input x rising edge pending bit
pub type RPIF0_R = crate::BitReader;
///Field `RPIF0` writer - Configurable event input x rising edge pending bit
pub type RPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF1` reader - Configurable event input x rising edge pending bit
pub type RPIF1_R = crate::BitReader;
///Field `RPIF1` writer - Configurable event input x rising edge pending bit
pub type RPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF2` reader - Configurable event input x rising edge pending bit
pub type RPIF2_R = crate::BitReader;
///Field `RPIF2` writer - Configurable event input x rising edge pending bit
pub type RPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF3` reader - Configurable event input x rising edge pending bit
pub type RPIF3_R = crate::BitReader;
///Field `RPIF3` writer - Configurable event input x rising edge pending bit
pub type RPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF4` reader - Configurable event input x rising edge pending bit
pub type RPIF4_R = crate::BitReader;
///Field `RPIF4` writer - Configurable event input x rising edge pending bit
pub type RPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF5` reader - Configurable event input x rising edge pending bit
pub type RPIF5_R = crate::BitReader;
///Field `RPIF5` writer - Configurable event input x rising edge pending bit
pub type RPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF6` reader - Configurable event input x rising edge pending bit
pub type RPIF6_R = crate::BitReader;
///Field `RPIF6` writer - Configurable event input x rising edge pending bit
pub type RPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF7` reader - Configurable event input x rising edge pending bit
pub type RPIF7_R = crate::BitReader;
///Field `RPIF7` writer - Configurable event input x rising edge pending bit
pub type RPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF8` reader - Configurable event input x rising edge pending bit
pub type RPIF8_R = crate::BitReader;
///Field `RPIF8` writer - Configurable event input x rising edge pending bit
pub type RPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF9` reader - Configurable event input x rising edge pending bit
pub type RPIF9_R = crate::BitReader;
///Field `RPIF9` writer - Configurable event input x rising edge pending bit
pub type RPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF10` reader - Configurable event input x rising edge pending bit
pub type RPIF10_R = crate::BitReader;
///Field `RPIF10` writer - Configurable event input x rising edge pending bit
pub type RPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF11` reader - Configurable event input x rising edge pending bit
pub type RPIF11_R = crate::BitReader;
///Field `RPIF11` writer - Configurable event input x rising edge pending bit
pub type RPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF12` reader - Configurable event input x rising edge pending bit
pub type RPIF12_R = crate::BitReader;
///Field `RPIF12` writer - Configurable event input x rising edge pending bit
pub type RPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF13` reader - Configurable event input x rising edge pending bit
pub type RPIF13_R = crate::BitReader;
///Field `RPIF13` writer - Configurable event input x rising edge pending bit
pub type RPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF14` reader - Configurable event input x rising edge pending bit
pub type RPIF14_R = crate::BitReader;
///Field `RPIF14` writer - Configurable event input x rising edge pending bit
pub type RPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF15` reader - Configurable event input x rising edge pending bit
pub type RPIF15_R = crate::BitReader;
///Field `RPIF15` writer - Configurable event input x rising edge pending bit
pub type RPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF20` reader - Configurable event input x rising edge pending bit
pub type RPIF20_R = crate::BitReader;
///Field `RPIF20` writer - Configurable event input x rising edge pending bit
pub type RPIF20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF21` reader - Configurable event input x rising edge pending bit
pub type RPIF21_R = crate::BitReader;
///Field `RPIF21` writer - Configurable event input x rising edge pending bit
pub type RPIF21_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif20(&self) -> RPIF20_R {
        RPIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif21(&self) -> RPIF21_R {
        RPIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR1")
            .field("rpif0", &self.rpif0())
            .field("rpif1", &self.rpif1())
            .field("rpif2", &self.rpif2())
            .field("rpif3", &self.rpif3())
            .field("rpif4", &self.rpif4())
            .field("rpif5", &self.rpif5())
            .field("rpif6", &self.rpif6())
            .field("rpif7", &self.rpif7())
            .field("rpif8", &self.rpif8())
            .field("rpif9", &self.rpif9())
            .field("rpif10", &self.rpif10())
            .field("rpif11", &self.rpif11())
            .field("rpif12", &self.rpif12())
            .field("rpif13", &self.rpif13())
            .field("rpif14", &self.rpif14())
            .field("rpif15", &self.rpif15())
            .field("rpif20", &self.rpif20())
            .field("rpif21", &self.rpif21())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W<'_, RPR1rs> {
        RPIF0_W::new(self, 0)
    }
    ///Bit 1 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W<'_, RPR1rs> {
        RPIF1_W::new(self, 1)
    }
    ///Bit 2 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W<'_, RPR1rs> {
        RPIF2_W::new(self, 2)
    }
    ///Bit 3 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W<'_, RPR1rs> {
        RPIF3_W::new(self, 3)
    }
    ///Bit 4 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W<'_, RPR1rs> {
        RPIF4_W::new(self, 4)
    }
    ///Bit 5 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W<'_, RPR1rs> {
        RPIF5_W::new(self, 5)
    }
    ///Bit 6 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W<'_, RPR1rs> {
        RPIF6_W::new(self, 6)
    }
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W<'_, RPR1rs> {
        RPIF7_W::new(self, 7)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W<'_, RPR1rs> {
        RPIF8_W::new(self, 8)
    }
    ///Bit 9 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W<'_, RPR1rs> {
        RPIF9_W::new(self, 9)
    }
    ///Bit 10 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W<'_, RPR1rs> {
        RPIF10_W::new(self, 10)
    }
    ///Bit 11 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W<'_, RPR1rs> {
        RPIF11_W::new(self, 11)
    }
    ///Bit 12 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W<'_, RPR1rs> {
        RPIF12_W::new(self, 12)
    }
    ///Bit 13 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W<'_, RPR1rs> {
        RPIF13_W::new(self, 13)
    }
    ///Bit 14 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W<'_, RPR1rs> {
        RPIF14_W::new(self, 14)
    }
    ///Bit 15 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W<'_, RPR1rs> {
        RPIF15_W::new(self, 15)
    }
    ///Bit 20 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif20(&mut self) -> RPIF20_W<'_, RPR1rs> {
        RPIF20_W::new(self, 20)
    }
    ///Bit 21 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif21(&mut self) -> RPIF21_W<'_, RPR1rs> {
        RPIF21_W::new(self, 21)
    }
}
/**EXTI rising edge pending register

You can [`read`](crate::Reg::read) this register and get [`rpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#EXTI:RPR1)*/
pub struct RPR1rs;
impl crate::RegisterSpec for RPR1rs {
    type Ux = u32;
}
///`read()` method returns [`rpr1::R`](R) reader structure
impl crate::Readable for RPR1rs {}
///`write(|w| ..)` method takes [`rpr1::W`](W) writer structure
impl crate::Writable for RPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR1 to value 0
impl crate::Resettable for RPR1rs {}
