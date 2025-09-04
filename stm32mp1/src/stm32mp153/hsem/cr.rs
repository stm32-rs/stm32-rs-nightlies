///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `COREID` writer - COREID
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KEY` writer - KEY
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W<CRrs> {
        COREID_W::new(self, 8)
    }
    ///Bits 16:31 - KEY
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<CRrs> {
        KEY_W::new(self, 16)
    }
}
/**Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
