///Register `VENCRAMERKEYR` writer
pub type W = crate::W<VENCRAMERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<VENCRAMERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, VENCRAMERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG VENCRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramerkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RAMCFG:VENCRAMERKEYR)*/
pub struct VENCRAMERKEYRrs;
impl crate::RegisterSpec for VENCRAMERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`vencramerkeyr::W`](W) writer structure
impl crate::Writable for VENCRAMERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VENCRAMERKEYR to value 0
impl crate::Resettable for VENCRAMERKEYRrs {}
