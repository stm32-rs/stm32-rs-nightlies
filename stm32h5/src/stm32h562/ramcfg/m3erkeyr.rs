///Register `M3ERKEYR` writer
pub type W = crate::W<M3ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M3ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\[7:0\]. 2) Write 0x53 into ERASEKEY\[7:0\]. Note: Writing a wrong key reactivates the write protection.
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, M3ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG memory 3 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RAMCFG:M3ERKEYR)*/
pub struct M3ERKEYRrs;
impl crate::RegisterSpec for M3ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m3erkeyr::W`](W) writer structure
impl crate::Writable for M3ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3ERKEYR to value 0
impl crate::Resettable for M3ERKEYRrs {}
