///Register `SECKEYR` writer
pub type W = crate::W<SECKEYRrs>;
///Field `SECKEYR` writer - SECKEYR
pub type SECKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SECKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - SECKEYR
    #[inline(always)]
    pub fn seckeyr(&mut self) -> SECKEYR_W<'_, SECKEYRrs> {
        SECKEYR_W::new(self, 0)
    }
}
/**Flash secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:SECKEYR)*/
pub struct SECKEYRrs;
impl crate::RegisterSpec for SECKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`seckeyr::W`](W) writer structure
impl crate::Writable for SECKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECKEYR to value 0
impl crate::Resettable for SECKEYRrs {}
