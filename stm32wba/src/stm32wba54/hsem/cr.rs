///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LOCKID` writer - LOCKID of semaphores to be cleared This field can be written by software and is always read 0. This field indicates the LOCKID for which the semaphores are cleared when writing the HSEM_CR.
pub type LOCKID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC` writer - SEC value of semaphores to be cleared. This field can be written by software, is always read 0. Indicates the SEC for which the CID semaphores are cleared when writing the HSEM_CR
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` writer - PRIV value of semaphores to be cleared. This field can be written by software, is always read 0. Indicates the PRIV for which the CID semaphores are cleared when writing the HSEM_CR.
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEY` writer - Semaphore clear key This field can be written by software and is always read 0. If this key value does not match HSEM_KEYR.KEY, semaphores are not affected. If this key value matches HSEM_KEYR.KEY, all semaphores matching the LOCKID are cleared to the free state.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<CRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 8:11 - LOCKID of semaphores to be cleared This field can be written by software and is always read 0. This field indicates the LOCKID for which the semaphores are cleared when writing the HSEM_CR.
    #[inline(always)]
    pub fn lockid(&mut self) -> LOCKID_W<'_, CRrs> {
        LOCKID_W::new(self, 8)
    }
    ///Bit 12 - SEC value of semaphores to be cleared. This field can be written by software, is always read 0. Indicates the SEC for which the CID semaphores are cleared when writing the HSEM_CR
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, CRrs> {
        SEC_W::new(self, 12)
    }
    ///Bit 13 - PRIV value of semaphores to be cleared. This field can be written by software, is always read 0. Indicates the PRIV for which the CID semaphores are cleared when writing the HSEM_CR.
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, CRrs> {
        PRIV_W::new(self, 13)
    }
    ///Bits 16:31 - Semaphore clear key This field can be written by software and is always read 0. If this key value does not match HSEM_KEYR.KEY, semaphores are not affected. If this key value matches HSEM_KEYR.KEY, all semaphores matching the LOCKID are cleared to the free state.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, CRrs> {
        KEY_W::new(self, 16)
    }
}
/**HSEM clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#HSEM:CR)*/
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
