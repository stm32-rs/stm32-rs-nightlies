///Register `APB3ENSR` writer
pub type W = crate::W<APB3ENSRrs>;
///Field `DFTENS` writer - DFT enable
pub type DFTENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB3ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - DFT enable
    #[inline(always)]
    pub fn dftens(&mut self) -> DFTENS_W<'_, APB3ENSRrs> {
        DFTENS_W::new(self, 2)
    }
}
/**RCC APB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB3ENSR)*/
pub struct APB3ENSRrs;
impl crate::RegisterSpec for APB3ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb3ensr::W`](W) writer structure
impl crate::Writable for APB3ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENSR to value 0
impl crate::Resettable for APB3ENSRrs {}
