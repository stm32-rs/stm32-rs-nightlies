///Register `KEYR` writer
pub type W = crate::W<KEYRrs>;
///Field `KEYR` writer - KEYR
pub type KEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - KEYR
    #[inline(always)]
    pub fn keyr(&mut self) -> KEYR_W<'_, KEYRrs> {
        KEYR_W::new(self, 0)
    }
}
/**Flash key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:KEYR)*/
pub struct KEYRrs;
impl crate::RegisterSpec for KEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr::W`](W) writer structure
impl crate::Writable for KEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR to value 0
impl crate::Resettable for KEYRrs {}
