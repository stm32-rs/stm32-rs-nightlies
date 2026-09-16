///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MASTERID` writer - MASTERID
pub type MASTERID_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `KEY` writer - Semaphore clear Key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 8:11 - MASTERID
    #[inline(always)]
    pub fn masterid(&mut self) -> MASTERID_W<'_, CRrs> {
        MASTERID_W::new(self, 8)
    }
    ///Bits 16:31 - Semaphore clear Key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CRrs> {
        KEY_W::new(self, 16)
    }
}
/**HSEM Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#HSEM:CR)*/
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
