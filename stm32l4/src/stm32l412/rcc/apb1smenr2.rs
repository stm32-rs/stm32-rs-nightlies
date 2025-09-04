///Register `APB1SMENR2` reader
pub type R = crate::R<APB1SMENR2rs>;
///Register `APB1SMENR2` writer
pub type W = crate::W<APB1SMENR2rs>;
///Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_R = crate::BitReader;
///Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SMEN` reader - LPTIM2SMEN
pub type LPTIM2SMEN_R = crate::BitReader;
///Field `LPTIM2SMEN` writer - LPTIM2SMEN
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR2")
            .field("lptim2smen", &self.lptim2smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
}
/**APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:APB1SMENR2)*/
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
///`reset()` method sets APB1SMENR2 to value 0x21
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x21;
}
