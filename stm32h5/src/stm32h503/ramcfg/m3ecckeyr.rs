///Register `M3ECCKEYR` writer
pub type W = crate::W<M3ECCKEYRrs>;
///Field `ECCKEY` writer - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\[7:0\]. 2) Write 0x75 into ECCKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M3ECCKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\[7:0\]. 2) Write 0x75 into ECCKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<M3ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG memory 3 ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RAMCFG:M3ECCKEYR)*/
pub struct M3ECCKEYRrs;
impl crate::RegisterSpec for M3ECCKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m3ecckeyr::W`](W) writer structure
impl crate::Writable for M3ECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3ECCKEYR to value 0
impl crate::Resettable for M3ECCKEYRrs {}
