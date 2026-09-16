///Register `APB5RSTCR` writer
pub type W = crate::W<APB5RSTCRrs>;
///Field `LTDCRSTC` writer - LTDC reset
pub type LTDCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPRSTC` writer - DCMIPP reset
pub type DCMIPPRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMRSTC` writer - GFXTIM reset
pub type GFXTIMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRSTC` writer - VENC reset
pub type VENCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIRSTC` writer - CSI reset
pub type CSIRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC reset
    #[inline(always)]
    pub fn ltdcrstc(&mut self) -> LTDCRSTC_W<'_, APB5RSTCRrs> {
        LTDCRSTC_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP reset
    #[inline(always)]
    pub fn dcmipprstc(&mut self) -> DCMIPPRSTC_W<'_, APB5RSTCRrs> {
        DCMIPPRSTC_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM reset
    #[inline(always)]
    pub fn gfxtimrstc(&mut self) -> GFXTIMRSTC_W<'_, APB5RSTCRrs> {
        GFXTIMRSTC_W::new(self, 4)
    }
    ///Bit 5 - VENC reset
    #[inline(always)]
    pub fn vencrstc(&mut self) -> VENCRSTC_W<'_, APB5RSTCRrs> {
        VENCRSTC_W::new(self, 5)
    }
    ///Bit 6 - CSI reset
    #[inline(always)]
    pub fn csirstc(&mut self) -> CSIRSTC_W<'_, APB5RSTCRrs> {
        CSIRSTC_W::new(self, 6)
    }
}
/**RCC APB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB5RSTCR)*/
pub struct APB5RSTCRrs;
impl crate::RegisterSpec for APB5RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5rstcr::W`](W) writer structure
impl crate::Writable for APB5RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5RSTCR to value 0
impl crate::Resettable for APB5RSTCRrs {}
