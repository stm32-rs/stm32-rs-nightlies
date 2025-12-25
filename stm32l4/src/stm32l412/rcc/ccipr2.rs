///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `I2C4SEL_0` reader - I2C4 clock source selection
pub type I2C4SEL_0_R = crate::BitReader;
///Field `I2C4SEL_0` writer - I2C4 clock source selection
pub type I2C4SEL_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4SEL_1` reader - I2C4 clock source selection
pub type I2C4SEL_1_R = crate::BitReader;
impl R {
    ///Bit 0 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel_0(&self) -> I2C4SEL_0_R {
        I2C4SEL_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel_1(&self) -> I2C4SEL_1_R {
        I2C4SEL_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("i2c4sel_1", &self.i2c4sel_1())
            .field("i2c4sel_0", &self.i2c4sel_0())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel_0(&mut self) -> I2C4SEL_0_W<'_, CCIPR2rs> {
        I2C4SEL_0_W::new(self, 0)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
