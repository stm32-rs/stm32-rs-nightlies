///Register `M5ECCKEYR` writer
pub type W = crate::W<M5ECCKEYRrs>;
///Field `ECCKEY` writer - ECC write protection key
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M5ECCKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ECC write protection key
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<'_, M5ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG memory 5 ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RAMCFG:M5ECCKEYR)*/
pub struct M5ECCKEYRrs;
impl crate::RegisterSpec for M5ECCKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m5ecckeyr::W`](W) writer structure
impl crate::Writable for M5ECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5ECCKEYR to value 0
impl crate::Resettable for M5ECCKEYRrs {}
