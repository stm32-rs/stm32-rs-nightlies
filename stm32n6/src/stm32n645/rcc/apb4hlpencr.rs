///Register `APB4HLPENCR` writer
pub type W = crate::W<APB4HLPENCRrs>;
///Field `SYSCFGLPENC` writer - SYSCFG sleep enable
pub type SYSCFGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECLPENC` writer - BSEC sleep enable
pub type BSECLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSLPENC` writer - DTS sleep enable
pub type DTSLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMLPENC` writer - BUSPERFM sleep enable
pub type BUSPERFMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG sleep enable
    #[inline(always)]
    pub fn syscfglpenc(&mut self) -> SYSCFGLPENC_W<'_, APB4HLPENCRrs> {
        SYSCFGLPENC_W::new(self, 0)
    }
    ///Bit 1 - BSEC sleep enable
    #[inline(always)]
    pub fn bseclpenc(&mut self) -> BSECLPENC_W<'_, APB4HLPENCRrs> {
        BSECLPENC_W::new(self, 1)
    }
    ///Bit 2 - DTS sleep enable
    #[inline(always)]
    pub fn dtslpenc(&mut self) -> DTSLPENC_W<'_, APB4HLPENCRrs> {
        DTSLPENC_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM sleep enable
    #[inline(always)]
    pub fn busperfmlpenc(&mut self) -> BUSPERFMLPENC_W<'_, APB4HLPENCRrs> {
        BUSPERFMLPENC_W::new(self, 4)
    }
}
/**RCC APB4H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB4HLPENCR)*/
pub struct APB4HLPENCRrs;
impl crate::RegisterSpec for APB4HLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hlpencr::W`](W) writer structure
impl crate::Writable for APB4HLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HLPENCR to value 0
impl crate::Resettable for APB4HLPENCRrs {}
