///Register `LVEKEYR` writer
pub type W = crate::W<LVEKEYRrs>;
///Field `LVEKEYR` writer - LVEKEYR
pub type LVEKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<LVEKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - LVEKEYR
    #[inline(always)]
    pub fn lvekeyr(&mut self) -> LVEKEYR_W<'_, LVEKEYRrs> {
        LVEKEYR_W::new(self, 0)
    }
}
/**Flash low voltage key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvekeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:LVEKEYR)*/
pub struct LVEKEYRrs;
impl crate::RegisterSpec for LVEKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvekeyr::W`](W) writer structure
impl crate::Writable for LVEKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVEKEYR to value 0
impl crate::Resettable for LVEKEYRrs {}
