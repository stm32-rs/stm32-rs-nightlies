#[doc = "Register `APB1SMENR2` reader"]
pub type R = crate::R<APB1SMENR2rs>;
#[doc = "Register `APB1SMENR2` writer"]
pub type W = crate::W<APB1SMENR2rs>;
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_R = crate::BitReader;
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_R = crate::BitReader;
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes"]
pub type UCPD1SMEN_R = crate::BitReader;
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes"]
pub type UCPD1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<APB1SMENR2rs> {
        UCPD1SMEN_W::new(self, 8)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr2::R`](R) reader structure"]
impl crate::Readable for APB1SMENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure"]
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x0103"]
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x0103;
}
