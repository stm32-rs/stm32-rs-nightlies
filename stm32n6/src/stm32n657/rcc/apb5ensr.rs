///Register `APB5ENSR` writer
pub type W = crate::W<APB5ENSRrs>;
///Field `LTDCENS` writer - LTDC enable
pub type LTDCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPENS` writer - DCMIPP enable
pub type DCMIPPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMENS` writer - GFXTIM enable
pub type GFXTIMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCENS` writer - VENC enable
pub type VENCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIENS` writer - CSI enable
pub type CSIENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC enable
    #[inline(always)]
    pub fn ltdcens(&mut self) -> LTDCENS_W<'_, APB5ENSRrs> {
        LTDCENS_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP enable
    #[inline(always)]
    pub fn dcmippens(&mut self) -> DCMIPPENS_W<'_, APB5ENSRrs> {
        DCMIPPENS_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM enable
    #[inline(always)]
    pub fn gfxtimens(&mut self) -> GFXTIMENS_W<'_, APB5ENSRrs> {
        GFXTIMENS_W::new(self, 4)
    }
    ///Bit 5 - VENC enable
    #[inline(always)]
    pub fn vencens(&mut self) -> VENCENS_W<'_, APB5ENSRrs> {
        VENCENS_W::new(self, 5)
    }
    ///Bit 6 - CSI enable
    #[inline(always)]
    pub fn csiens(&mut self) -> CSIENS_W<'_, APB5ENSRrs> {
        CSIENS_W::new(self, 6)
    }
}
/**RCC APB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB5ENSR)*/
pub struct APB5ENSRrs;
impl crate::RegisterSpec for APB5ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5ensr::W`](W) writer structure
impl crate::Writable for APB5ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5ENSR to value 0
impl crate::Resettable for APB5ENSRrs {}
