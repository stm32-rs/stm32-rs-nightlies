///Register `TXEFA` reader
pub type R = crate::R<TXEFArs>;
///Register `TXEFA` writer
pub type W = crate::W<TXEFArs>;
///Field `EFAI` reader - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
pub type EFAI_R = crate::FieldReader;
///Field `EFAI` writer - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
pub type EFAI_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFA").field("efai", &self.efai()).finish()
    }
}
impl W {
    ///Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W<'_, TXEFArs> {
        EFAI_W::new(self, 0)
    }
}
/**FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#FDCAN1:TXEFA)*/
pub struct TXEFArs;
impl crate::RegisterSpec for TXEFArs {
    type Ux = u32;
}
///`read()` method returns [`txefa::R`](R) reader structure
impl crate::Readable for TXEFArs {}
///`write(|w| ..)` method takes [`txefa::W`](W) writer structure
impl crate::Writable for TXEFArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXEFA to value 0
impl crate::Resettable for TXEFArs {}
