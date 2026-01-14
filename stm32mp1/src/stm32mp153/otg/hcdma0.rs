///Register `HCDMA0` reader
pub type R = crate::R<HCDMA0rs>;
///Register `HCDMA0` writer
pub type W = crate::W<HCDMA0rs>;
///Field `DMAADDR` reader - DMAADDR
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMAADDR
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA0")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, HCDMA0rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 0 DMA address register in buffer DMA \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`hcdma0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HCDMA0)*/
pub struct HCDMA0rs;
impl crate::RegisterSpec for HCDMA0rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma0::R`](R) reader structure
impl crate::Readable for HCDMA0rs {}
///`write(|w| ..)` method takes [`hcdma0::W`](W) writer structure
impl crate::Writable for HCDMA0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA0 to value 0
impl crate::Resettable for HCDMA0rs {}
