///Register `APB3LPENCR` writer
pub type W = crate::W<APB3LPENCRrs>;
///Field `DFTLPENC` writer - DFT sleep enable
pub type DFTLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB3LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - DFT sleep enable
    #[inline(always)]
    pub fn dftlpenc(&mut self) -> DFTLPENC_W<APB3LPENCRrs> {
        DFTLPENC_W::new(self, 2)
    }
}
/**RCC APB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB3LPENCR)*/
pub struct APB3LPENCRrs;
impl crate::RegisterSpec for APB3LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb3lpencr::W`](W) writer structure
impl crate::Writable for APB3LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3LPENCR to value 0
impl crate::Resettable for APB3LPENCRrs {}
