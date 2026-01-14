///Register `APB3RSTR` reader
pub type R = crate::R<APB3RSTRrs>;
///Register `APB3RSTR` writer
pub type W = crate::W<APB3RSTRrs>;
///Field `SUBGHZSPIRST` reader - Sub-GHz radio SPI reset
pub type SUBGHZSPIRST_R = crate::BitReader;
///Field `SUBGHZSPIRST` writer - Sub-GHz radio SPI reset
pub type SUBGHZSPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sub-GHz radio SPI reset
    #[inline(always)]
    pub fn subghzspirst(&self) -> SUBGHZSPIRST_R {
        SUBGHZSPIRST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTR")
            .field("subghzspirst", &self.subghzspirst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sub-GHz radio SPI reset
    #[inline(always)]
    pub fn subghzspirst(&mut self) -> SUBGHZSPIRST_W<'_, APB3RSTRrs> {
        SUBGHZSPIRST_W::new(self, 0)
    }
}
/**APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB3RSTR)*/
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstr::R`](R) reader structure
impl crate::Readable for APB3RSTRrs {}
///`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTR to value 0
impl crate::Resettable for APB3RSTRrs {}
