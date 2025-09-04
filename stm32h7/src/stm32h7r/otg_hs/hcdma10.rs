///Register `HCDMA10` reader
pub type R = crate::R<HCDMA10rs>;
///Register `HCDMA10` writer
pub type W = crate::W<HCDMA10rs>;
///Field `DMAADDR` reader - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA10")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA10rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 10 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:HCDMA10)*/
pub struct HCDMA10rs;
impl crate::RegisterSpec for HCDMA10rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma10::R`](R) reader structure
impl crate::Readable for HCDMA10rs {}
///`write(|w| ..)` method takes [`hcdma10::W`](W) writer structure
impl crate::Writable for HCDMA10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA10 to value 0
impl crate::Resettable for HCDMA10rs {}
