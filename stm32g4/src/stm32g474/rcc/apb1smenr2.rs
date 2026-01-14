///Register `APB1SMENR2` reader
pub type R = crate::R<APB1SMENR2rs>;
///Register `APB1SMENR2` writer
pub type W = crate::W<APB1SMENR2rs>;
///Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART1SMEN_R = crate::BitReader;
///Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software
pub type I2C4SMEN_R = crate::BitReader;
///Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type UCPD1SMEN_R = crate::BitReader;
///Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type UCPD1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR2")
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c4smen", &self.i2c4smen())
            .field("ucpd1smen", &self.ucpd1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<'_, APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    ///Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<'_, APB1SMENR2rs> {
        UCPD1SMEN_W::new(self, 8)
    }
}
/**APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#RCC:APB1SMENR2)*/
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr2::R`](R) reader structure
impl crate::Readable for APB1SMENR2rs {}
///`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR2 to value 0x0103
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x0103;
}
