///Register `AR` writer
pub type W = crate::W<ARrs>;
///Field `FAR` writer - Flash Address
pub type FAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<ARrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash Address
    #[inline(always)]
    pub fn far(&mut self) -> FAR_W<'_, ARrs> {
        FAR_W::new(self, 0)
    }
}
/**Flash address register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#FLASH:AR)*/
pub struct ARrs;
impl crate::RegisterSpec for ARrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ar::W`](W) writer structure
impl crate::Writable for ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AR to value 0
impl crate::Resettable for ARrs {}
