///Register `FPR1` reader
pub type R = crate::R<FPR1rs>;
///Register `FPR1` writer
pub type W = crate::W<FPR1rs>;
///Field `FPIF0` reader - Configurable event input x rising edge pending bit
pub type FPIF0_R = crate::BitReader;
///Field `FPIF0` writer - Configurable event input x rising edge pending bit
pub type FPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF1` reader - Configurable event input x rising edge pending bit
pub type FPIF1_R = crate::BitReader;
///Field `FPIF1` writer - Configurable event input x rising edge pending bit
pub type FPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF2` reader - Configurable event input x rising edge pending bit
pub type FPIF2_R = crate::BitReader;
///Field `FPIF2` writer - Configurable event input x rising edge pending bit
pub type FPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF3` reader - Configurable event input x rising edge pending bit
pub type FPIF3_R = crate::BitReader;
///Field `FPIF3` writer - Configurable event input x rising edge pending bit
pub type FPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF4` reader - Configurable event input x rising edge pending bit
pub type FPIF4_R = crate::BitReader;
///Field `FPIF4` writer - Configurable event input x rising edge pending bit
pub type FPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF5` reader - Configurable event input x rising edge pending bit
pub type FPIF5_R = crate::BitReader;
///Field `FPIF5` writer - Configurable event input x rising edge pending bit
pub type FPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF6` reader - Configurable event input x rising edge pending bit
pub type FPIF6_R = crate::BitReader;
///Field `FPIF6` writer - Configurable event input x rising edge pending bit
pub type FPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF7` reader - Configurable event input x rising edge pending bit
pub type FPIF7_R = crate::BitReader;
///Field `FPIF7` writer - Configurable event input x rising edge pending bit
pub type FPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF8` reader - Configurable event input x rising edge pending bit
pub type FPIF8_R = crate::BitReader;
///Field `FPIF8` writer - Configurable event input x rising edge pending bit
pub type FPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF9` reader - Configurable event input x rising edge pending bit
pub type FPIF9_R = crate::BitReader;
///Field `FPIF9` writer - Configurable event input x rising edge pending bit
pub type FPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF10` reader - Configurable event input x rising edge pending bit
pub type FPIF10_R = crate::BitReader;
///Field `FPIF10` writer - Configurable event input x rising edge pending bit
pub type FPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF11` reader - Configurable event input x rising edge pending bit
pub type FPIF11_R = crate::BitReader;
///Field `FPIF11` writer - Configurable event input x rising edge pending bit
pub type FPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF12` reader - Configurable event input x rising edge pending bit
pub type FPIF12_R = crate::BitReader;
///Field `FPIF12` writer - Configurable event input x rising edge pending bit
pub type FPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF13` reader - Configurable event input x rising edge pending bit
pub type FPIF13_R = crate::BitReader;
///Field `FPIF13` writer - Configurable event input x rising edge pending bit
pub type FPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF14` reader - Configurable event input x rising edge pending bit
pub type FPIF14_R = crate::BitReader;
///Field `FPIF14` writer - Configurable event input x rising edge pending bit
pub type FPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF15` reader - Configurable event input x rising edge pending bit
pub type FPIF15_R = crate::BitReader;
///Field `FPIF15` writer - Configurable event input x rising edge pending bit
pub type FPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF20` reader - configurable event input x falling edge pending bit
pub type FPIF20_R = crate::BitReader;
///Field `FPIF20` writer - configurable event input x falling edge pending bit
pub type FPIF20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF21` reader - configurable event input x falling edge pending bit
pub type FPIF21_R = crate::BitReader;
///Field `FPIF21` writer - configurable event input x falling edge pending bit
pub type FPIF21_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif20(&self) -> FPIF20_R {
        FPIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif21(&self) -> FPIF21_R {
        FPIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR1")
            .field("fpif0", &self.fpif0())
            .field("fpif1", &self.fpif1())
            .field("fpif2", &self.fpif2())
            .field("fpif3", &self.fpif3())
            .field("fpif4", &self.fpif4())
            .field("fpif5", &self.fpif5())
            .field("fpif6", &self.fpif6())
            .field("fpif7", &self.fpif7())
            .field("fpif8", &self.fpif8())
            .field("fpif9", &self.fpif9())
            .field("fpif10", &self.fpif10())
            .field("fpif11", &self.fpif11())
            .field("fpif12", &self.fpif12())
            .field("fpif13", &self.fpif13())
            .field("fpif14", &self.fpif14())
            .field("fpif15", &self.fpif15())
            .field("fpif20", &self.fpif20())
            .field("fpif21", &self.fpif21())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W<FPR1rs> {
        FPIF0_W::new(self, 0)
    }
    ///Bit 1 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W<FPR1rs> {
        FPIF1_W::new(self, 1)
    }
    ///Bit 2 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W<FPR1rs> {
        FPIF2_W::new(self, 2)
    }
    ///Bit 3 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W<FPR1rs> {
        FPIF3_W::new(self, 3)
    }
    ///Bit 4 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W<FPR1rs> {
        FPIF4_W::new(self, 4)
    }
    ///Bit 5 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W<FPR1rs> {
        FPIF5_W::new(self, 5)
    }
    ///Bit 6 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W<FPR1rs> {
        FPIF6_W::new(self, 6)
    }
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W<FPR1rs> {
        FPIF7_W::new(self, 7)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W<FPR1rs> {
        FPIF8_W::new(self, 8)
    }
    ///Bit 9 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W<FPR1rs> {
        FPIF9_W::new(self, 9)
    }
    ///Bit 10 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W<FPR1rs> {
        FPIF10_W::new(self, 10)
    }
    ///Bit 11 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W<FPR1rs> {
        FPIF11_W::new(self, 11)
    }
    ///Bit 12 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W<FPR1rs> {
        FPIF12_W::new(self, 12)
    }
    ///Bit 13 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W<FPR1rs> {
        FPIF13_W::new(self, 13)
    }
    ///Bit 14 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W<FPR1rs> {
        FPIF14_W::new(self, 14)
    }
    ///Bit 15 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W<FPR1rs> {
        FPIF15_W::new(self, 15)
    }
    ///Bit 20 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif20(&mut self) -> FPIF20_W<FPR1rs> {
        FPIF20_W::new(self, 20)
    }
    ///Bit 21 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif21(&mut self) -> FPIF21_W<FPR1rs> {
        FPIF21_W::new(self, 21)
    }
}
/**EXTI falling edge pending register

You can [`read`](crate::Reg::read) this register and get [`fpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#EXTI:FPR1)*/
pub struct FPR1rs;
impl crate::RegisterSpec for FPR1rs {
    type Ux = u32;
}
///`read()` method returns [`fpr1::R`](R) reader structure
impl crate::Readable for FPR1rs {}
///`write(|w| ..)` method takes [`fpr1::W`](W) writer structure
impl crate::Writable for FPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPR1 to value 0
impl crate::Resettable for FPR1rs {}
