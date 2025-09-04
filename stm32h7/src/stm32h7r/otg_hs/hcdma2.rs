///Register `HCDMA2` reader
pub type R = crate::R<HCDMA2rs>;
///Register `HCDMA2` writer
pub type W = crate::W<HCDMA2rs>;
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
        f.debug_struct("HCDMA2")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction.
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA2rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 2 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:HCDMA2)*/
pub struct HCDMA2rs;
impl crate::RegisterSpec for HCDMA2rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma2::R`](R) reader structure
impl crate::Readable for HCDMA2rs {}
///`write(|w| ..)` method takes [`hcdma2::W`](W) writer structure
impl crate::Writable for HCDMA2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA2 to value 0
impl crate::Resettable for HCDMA2rs {}
