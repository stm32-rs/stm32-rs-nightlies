///Register `HSEM_CR` writer
pub type W = crate::W<HSEM_CRrs>;
///Field `COREID` writer - COREID
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KEY` writer - KEY
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<HSEM_CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 8:11 - COREID
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<HSEM_CRrs> {
        COREID_W::new(self, 8)
    }
    ///Bits 16:31 - KEY
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<HSEM_CRrs> {
        KEY_W::new(self, 16)
    }
}
/**Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_CR)*/
pub struct HSEM_CRrs;
impl crate::RegisterSpec for HSEM_CRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hsem_cr::W`](W) writer structure
impl crate::Writable for HSEM_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSEM_CR to value 0
impl crate::Resettable for HSEM_CRrs {
    const RESET_VALUE: u32 = 0;
}
