///Register `APB5LPENCR` writer
pub type W = crate::W<APB5LPENCRrs>;
///Field `LTDCLPENC` writer - LTDC sleep enable
pub type LTDCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPLPENC` writer - DCMIPP sleep enable
pub type DCMIPPLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMLPENC` writer - GFXTIM sleep enable
pub type GFXTIMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCLPENC` writer - VENC sleep enable
pub type VENCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSILPENC` writer - CSI sleep enable
pub type CSILPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC sleep enable
    #[inline(always)]
    pub fn ltdclpenc(&mut self) -> LTDCLPENC_W<'_, APB5LPENCRrs> {
        LTDCLPENC_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP sleep enable
    #[inline(always)]
    pub fn dcmipplpenc(&mut self) -> DCMIPPLPENC_W<'_, APB5LPENCRrs> {
        DCMIPPLPENC_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM sleep enable
    #[inline(always)]
    pub fn gfxtimlpenc(&mut self) -> GFXTIMLPENC_W<'_, APB5LPENCRrs> {
        GFXTIMLPENC_W::new(self, 4)
    }
    ///Bit 5 - VENC sleep enable
    #[inline(always)]
    pub fn venclpenc(&mut self) -> VENCLPENC_W<'_, APB5LPENCRrs> {
        VENCLPENC_W::new(self, 5)
    }
    ///Bit 6 - CSI sleep enable
    #[inline(always)]
    pub fn csilpenc(&mut self) -> CSILPENC_W<'_, APB5LPENCRrs> {
        CSILPENC_W::new(self, 6)
    }
}
/**RCC APB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB5LPENCR)*/
pub struct APB5LPENCRrs;
impl crate::RegisterSpec for APB5LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5lpencr::W`](W) writer structure
impl crate::Writable for APB5LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5LPENCR to value 0
impl crate::Resettable for APB5LPENCRrs {}
