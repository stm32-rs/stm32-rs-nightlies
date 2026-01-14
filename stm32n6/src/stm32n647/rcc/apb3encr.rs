///Register `APB3ENCR` writer
pub type W = crate::W<APB3ENCRrs>;
///Field `DFTENC` writer - DFT enable
pub type DFTENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB3ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - DFT enable
    #[inline(always)]
    pub fn dftenc(&mut self) -> DFTENC_W<'_, APB3ENCRrs> {
        DFTENC_W::new(self, 2)
    }
}
/**RCC APB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3ENCR)*/
pub struct APB3ENCRrs;
impl crate::RegisterSpec for APB3ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb3encr::W`](W) writer structure
impl crate::Writable for APB3ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENCR to value 0
impl crate::Resettable for APB3ENCRrs {}
