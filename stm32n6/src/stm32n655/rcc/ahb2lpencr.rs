///Register `AHB2LPENCR` writer
pub type W = crate::W<AHB2LPENCRrs>;
///Field `RAMCFGLPENC` writer - RAMCFG sleep enable
pub type RAMCFGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1LPENC` writer - MDF1 sleep enable
pub type MDF1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1LPENC` writer - ADF1 sleep enable
pub type ADF1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB2LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 12 - RAMCFG sleep enable
    #[inline(always)]
    pub fn ramcfglpenc(&mut self) -> RAMCFGLPENC_W<'_, AHB2LPENCRrs> {
        RAMCFGLPENC_W::new(self, 12)
    }
    ///Bit 16 - MDF1 sleep enable
    #[inline(always)]
    pub fn mdf1lpenc(&mut self) -> MDF1LPENC_W<'_, AHB2LPENCRrs> {
        MDF1LPENC_W::new(self, 16)
    }
    ///Bit 17 - ADF1 sleep enable
    #[inline(always)]
    pub fn adf1lpenc(&mut self) -> ADF1LPENC_W<'_, AHB2LPENCRrs> {
        ADF1LPENC_W::new(self, 17)
    }
}
/**RCC AHB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB2LPENCR)*/
pub struct AHB2LPENCRrs;
impl crate::RegisterSpec for AHB2LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb2lpencr::W`](W) writer structure
impl crate::Writable for AHB2LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENCR to value 0
impl crate::Resettable for AHB2LPENCRrs {}
