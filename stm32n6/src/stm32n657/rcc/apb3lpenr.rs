///Register `APB3LPENR` reader
pub type R = crate::R<APB3LPENRrs>;
///Register `APB3LPENR` writer
pub type W = crate::W<APB3LPENRrs>;
///Field `DFTLPEN` reader - DFT sleep enable
pub type DFTLPEN_R = crate::BitReader;
///Field `DFTLPEN` writer - DFT sleep enable
pub type DFTLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - DFT sleep enable
    #[inline(always)]
    pub fn dftlpen(&self) -> DFTLPEN_R {
        DFTLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3LPENR")
            .field("dftlpen", &self.dftlpen())
            .finish()
    }
}
impl W {
    ///Bit 2 - DFT sleep enable
    #[inline(always)]
    pub fn dftlpen(&mut self) -> DFTLPEN_W<'_, APB3LPENRrs> {
        DFTLPEN_W::new(self, 2)
    }
}
/**RCC APB3 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB3LPENR)*/
pub struct APB3LPENRrs;
impl crate::RegisterSpec for APB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3lpenr::R`](R) reader structure
impl crate::Readable for APB3LPENRrs {}
///`write(|w| ..)` method takes [`apb3lpenr::W`](W) writer structure
impl crate::Writable for APB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3LPENR to value 0
impl crate::Resettable for APB3LPENRrs {}
