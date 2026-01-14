///Register `APB3SMENR` reader
pub type R = crate::R<APB3SMENRrs>;
///Register `APB3SMENR` writer
pub type W = crate::W<APB3SMENRrs>;
///Field `SUBGHZSPISMEN` reader - Sub-GHz radio SPI clock enable during Sleep and Stop modes
pub type SUBGHZSPISMEN_R = crate::BitReader;
///Field `SUBGHZSPISMEN` writer - Sub-GHz radio SPI clock enable during Sleep and Stop modes
pub type SUBGHZSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3SMENR")
            .field("subghzspismen", &self.subghzspismen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn subghzspismen(&mut self) -> SUBGHZSPISMEN_W<'_, APB3SMENRrs> {
        SUBGHZSPISMEN_W::new(self, 0)
    }
}
/**APB3 peripheral clock enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB3SMENR)*/
pub struct APB3SMENRrs;
impl crate::RegisterSpec for APB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3smenr::R`](R) reader structure
impl crate::Readable for APB3SMENRrs {}
///`write(|w| ..)` method takes [`apb3smenr::W`](W) writer structure
impl crate::Writable for APB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3SMENR to value 0x01
impl crate::Resettable for APB3SMENRrs {
    const RESET_VALUE: u32 = 0x01;
}
