///Register `APB5LPENSR` writer
pub type W = crate::W<APB5LPENSRrs>;
///Field `LTDCLPENS` writer - LTDC sleep enable
pub type LTDCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPLPENS` writer - DCMIPP sleep enable
pub type DCMIPPLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMLPENS` writer - GFXTIM sleep enable
pub type GFXTIMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCLPENS` writer - VENC sleep enable
pub type VENCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSILPENS` writer - CSI sleep enable
pub type CSILPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC sleep enable
    #[inline(always)]
    pub fn ltdclpens(&mut self) -> LTDCLPENS_W<'_, APB5LPENSRrs> {
        LTDCLPENS_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP sleep enable
    #[inline(always)]
    pub fn dcmipplpens(&mut self) -> DCMIPPLPENS_W<'_, APB5LPENSRrs> {
        DCMIPPLPENS_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM sleep enable
    #[inline(always)]
    pub fn gfxtimlpens(&mut self) -> GFXTIMLPENS_W<'_, APB5LPENSRrs> {
        GFXTIMLPENS_W::new(self, 4)
    }
    ///Bit 5 - VENC sleep enable
    #[inline(always)]
    pub fn venclpens(&mut self) -> VENCLPENS_W<'_, APB5LPENSRrs> {
        VENCLPENS_W::new(self, 5)
    }
    ///Bit 6 - CSI sleep enable
    #[inline(always)]
    pub fn csilpens(&mut self) -> CSILPENS_W<'_, APB5LPENSRrs> {
        CSILPENS_W::new(self, 6)
    }
}
/**RCC APB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB5LPENSR)*/
pub struct APB5LPENSRrs;
impl crate::RegisterSpec for APB5LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5lpensr::W`](W) writer structure
impl crate::Writable for APB5LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5LPENSR to value 0
impl crate::Resettable for APB5LPENSRrs {}
