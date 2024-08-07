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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
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
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<ODRrs> {
        ODR0_W::new(self, 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<ODRrs> {
        ODR1_W::new(self, 1)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR2_W<ODRrs> {
        ODR2_W::new(self, 2)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<ODRrs> {
        ODR3_W::new(self, 3)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR4_W<ODRrs> {
        ODR4_W::new(self, 4)
    }
}
/**GPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {
    const RESET_VALUE: u32 = 0;
}
