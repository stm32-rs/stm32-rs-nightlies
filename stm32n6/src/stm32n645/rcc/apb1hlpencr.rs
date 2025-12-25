///Register `APB1HLPENCR` writer
pub type W = crate::W<APB1HLPENCRrs>;
///Field `MDIOSLPENC` writer - MDIOS sleep enable
pub type MDIOSLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANLPENC` writer - FDCAN sleep enable
pub type FDCANLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1LPENC` writer - UCPD1 sleep enable
pub type UCPD1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS sleep enable
    #[inline(always)]
    pub fn mdioslpenc(&mut self) -> MDIOSLPENC_W<'_, APB1HLPENCRrs> {
        MDIOSLPENC_W::new(self, 5)
    }
    ///Bit 8 - FDCAN sleep enable
    #[inline(always)]
    pub fn fdcanlpenc(&mut self) -> FDCANLPENC_W<'_, APB1HLPENCRrs> {
        FDCANLPENC_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 sleep enable
    #[inline(always)]
    pub fn ucpd1lpenc(&mut self) -> UCPD1LPENC_W<'_, APB1HLPENCRrs> {
        UCPD1LPENC_W::new(self, 18)
    }
}
/**RCC APB1H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB1HLPENCR)*/
pub struct APB1HLPENCRrs;
impl crate::RegisterSpec for APB1HLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hlpencr::W`](W) writer structure
impl crate::Writable for APB1HLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HLPENCR to value 0
impl crate::Resettable for APB1HLPENCRrs {}
