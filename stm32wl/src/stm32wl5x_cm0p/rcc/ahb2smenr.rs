///Register `AHB2SMENR` reader
pub type R = crate::R<AHB2SMENRrs>;
///Register `AHB2SMENR` writer
pub type W = crate::W<AHB2SMENRrs>;
///Field `GPIOASMEN` reader - IO port A clock enable during CPU1 CSleep mode.
pub type GPIOASMEN_R = crate::BitReader;
///Field `GPIOASMEN` writer - IO port A clock enable during CPU1 CSleep mode.
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBSMEN` reader - IO port B clock enable during CPU1 CSleep mode.
pub type GPIOBSMEN_R = crate::BitReader;
///Field `GPIOBSMEN` writer - IO port B clock enable during CPU1 CSleep mode.
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCSMEN` reader - IO port C clock enable during CPU1 CSleep mode.
pub type GPIOCSMEN_R = crate::BitReader;
///Field `GPIOCSMEN` writer - IO port C clock enable during CPU1 CSleep mode.
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHSMEN` reader - IO port H clock enable during CPU1 CSleep mode.
pub type GPIOHSMEN_R = crate::BitReader;
///Field `GPIOHSMEN` writer - IO port H clock enable during CPU1 CSleep mode.
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR")
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpioasmen", &self.gpioasmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 7 - IO port H clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<'_, AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
}
/**AHB2 peripheral clocks enable in Sleep modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB2SMENR)*/
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2smenr::R`](R) reader structure
impl crate::Readable for AHB2SMENRrs {}
///`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2SMENR to value 0x87
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x87;
}
