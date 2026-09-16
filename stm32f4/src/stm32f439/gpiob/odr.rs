///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
///Field `ODR0` reader - Port output data (y = 0..15)
pub type ODR0_R = crate::BitReader;
///Field `ODR0` writer - Port output data (y = 0..15)
pub type ODR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR1` reader - Port output data (y = 0..15)
pub type ODR1_R = crate::BitReader;
///Field `ODR1` writer - Port output data (y = 0..15)
pub type ODR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR2` reader - Port output data (y = 0..15)
pub type ODR2_R = crate::BitReader;
///Field `ODR2` writer - Port output data (y = 0..15)
pub type ODR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR3` reader - Port output data (y = 0..15)
pub type ODR3_R = crate::BitReader;
///Field `ODR3` writer - Port output data (y = 0..15)
pub type ODR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR4` reader - Port output data (y = 0..15)
pub type ODR4_R = crate::BitReader;
///Field `ODR4` writer - Port output data (y = 0..15)
pub type ODR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR5` reader - Port output data (y = 0..15)
pub type ODR5_R = crate::BitReader;
///Field `ODR5` writer - Port output data (y = 0..15)
pub type ODR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR6` reader - Port output data (y = 0..15)
pub type ODR6_R = crate::BitReader;
///Field `ODR6` writer - Port output data (y = 0..15)
pub type ODR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR7` reader - Port output data (y = 0..15)
pub type ODR7_R = crate::BitReader;
///Field `ODR7` writer - Port output data (y = 0..15)
pub type ODR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR8` reader - Port output data (y = 0..15)
pub type ODR8_R = crate::BitReader;
///Field `ODR8` writer - Port output data (y = 0..15)
pub type ODR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR9` reader - Port output data (y = 0..15)
pub type ODR9_R = crate::BitReader;
///Field `ODR9` writer - Port output data (y = 0..15)
pub type ODR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR10` reader - Port output data (y = 0..15)
pub type ODR10_R = crate::BitReader;
///Field `ODR10` writer - Port output data (y = 0..15)
pub type ODR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR11` reader - Port output data (y = 0..15)
pub type ODR11_R = crate::BitReader;
///Field `ODR11` writer - Port output data (y = 0..15)
pub type ODR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR12` reader - Port output data (y = 0..15)
pub type ODR12_R = crate::BitReader;
///Field `ODR12` writer - Port output data (y = 0..15)
pub type ODR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR13` reader - Port output data (y = 0..15)
pub type ODR13_R = crate::BitReader;
///Field `ODR13` writer - Port output data (y = 0..15)
pub type ODR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR14` reader - Port output data (y = 0..15)
pub type ODR14_R = crate::BitReader;
///Field `ODR14` writer - Port output data (y = 0..15)
pub type ODR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODR15` reader - Port output data (y = 0..15)
pub type ODR15_R = crate::BitReader;
///Field `ODR15` writer - Port output data (y = 0..15)
pub type ODR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("odr15", &self.odr15())
            .field("odr14", &self.odr14())
            .field("odr13", &self.odr13())
            .field("odr12", &self.odr12())
            .field("odr11", &self.odr11())
            .field("odr10", &self.odr10())
            .field("odr9", &self.odr9())
            .field("odr8", &self.odr8())
            .field("odr7", &self.odr7())
            .field("odr6", &self.odr6())
            .field("odr5", &self.odr5())
            .field("odr4", &self.odr4())
            .field("odr3", &self.odr3())
            .field("odr2", &self.odr2())
            .field("odr1", &self.odr1())
            .field("odr0", &self.odr0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR0_W<'_, ODRrs> {
        ODR0_W::new(self, 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W<'_, ODRrs> {
        ODR1_W::new(self, 1)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W<'_, ODRrs> {
        ODR2_W::new(self, 2)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W<'_, ODRrs> {
        ODR3_W::new(self, 3)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W<'_, ODRrs> {
        ODR4_W::new(self, 4)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W<'_, ODRrs> {
        ODR5_W::new(self, 5)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W<'_, ODRrs> {
        ODR6_W::new(self, 6)
    }
    ///Bit 7 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr7(&mut self) -> ODR7_W<'_, ODRrs> {
        ODR7_W::new(self, 7)
    }
    ///Bit 8 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr8(&mut self) -> ODR8_W<'_, ODRrs> {
        ODR8_W::new(self, 8)
    }
    ///Bit 9 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr9(&mut self) -> ODR9_W<'_, ODRrs> {
        ODR9_W::new(self, 9)
    }
    ///Bit 10 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr10(&mut self) -> ODR10_W<'_, ODRrs> {
        ODR10_W::new(self, 10)
    }
    ///Bit 11 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr11(&mut self) -> ODR11_W<'_, ODRrs> {
        ODR11_W::new(self, 11)
    }
    ///Bit 12 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr12(&mut self) -> ODR12_W<'_, ODRrs> {
        ODR12_W::new(self, 12)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W<'_, ODRrs> {
        ODR13_W::new(self, 13)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W<'_, ODRrs> {
        ODR14_W::new(self, 14)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W<'_, ODRrs> {
        ODR15_W::new(self, 15)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#GPIOB:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}
