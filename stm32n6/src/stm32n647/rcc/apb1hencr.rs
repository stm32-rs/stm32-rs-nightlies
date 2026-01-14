///Register `APB1HENCR` writer
pub type W = crate::W<APB1HENCRrs>;
///Field `MDIOSENC` writer - MDIOS enable
pub type MDIOSENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANENC` writer - FDCAN enable
pub type FDCANENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1ENC` writer - UCPD1 enable
pub type UCPD1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS enable
    #[inline(always)]
    pub fn mdiosenc(&mut self) -> MDIOSENC_W<'_, APB1HENCRrs> {
        MDIOSENC_W::new(self, 5)
    }
    ///Bit 8 - FDCAN enable
    #[inline(always)]
    pub fn fdcanenc(&mut self) -> FDCANENC_W<'_, APB1HENCRrs> {
        FDCANENC_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 enable
    #[inline(always)]
    pub fn ucpd1enc(&mut self) -> UCPD1ENC_W<'_, APB1HENCRrs> {
        UCPD1ENC_W::new(self, 18)
    }
}
/**RCC APB1H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HENCR)*/
pub struct APB1HENCRrs;
impl crate::RegisterSpec for APB1HENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hencr::W`](W) writer structure
impl crate::Writable for APB1HENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HENCR to value 0
impl crate::Resettable for APB1HENCRrs {}
