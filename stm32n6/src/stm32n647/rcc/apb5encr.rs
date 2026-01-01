///Register `APB5ENCR` writer
pub type W = crate::W<APB5ENCRrs>;
///Field `LTDCENC` writer - LTDC enable
pub type LTDCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPENC` writer - DCMIPP enable
pub type DCMIPPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMENC` writer - GFXTIM enable
pub type GFXTIMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCENC` writer - VENC enable
pub type VENCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIENC` writer - CSI enable
pub type CSIENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC enable
    #[inline(always)]
    pub fn ltdcenc(&mut self) -> LTDCENC_W<'_, APB5ENCRrs> {
        LTDCENC_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP enable
    #[inline(always)]
    pub fn dcmippenc(&mut self) -> DCMIPPENC_W<'_, APB5ENCRrs> {
        DCMIPPENC_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM enable
    #[inline(always)]
    pub fn gfxtimenc(&mut self) -> GFXTIMENC_W<'_, APB5ENCRrs> {
        GFXTIMENC_W::new(self, 4)
    }
    ///Bit 5 - VENC enable
    #[inline(always)]
    pub fn vencenc(&mut self) -> VENCENC_W<'_, APB5ENCRrs> {
        VENCENC_W::new(self, 5)
    }
    ///Bit 6 - CSI enable
    #[inline(always)]
    pub fn csienc(&mut self) -> CSIENC_W<'_, APB5ENCRrs> {
        CSIENC_W::new(self, 6)
    }
}
/**RCC APB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5ENCR)*/
pub struct APB5ENCRrs;
impl crate::RegisterSpec for APB5ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5encr::W`](W) writer structure
impl crate::Writable for APB5ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5ENCR to value 0
impl crate::Resettable for APB5ENCRrs {}
