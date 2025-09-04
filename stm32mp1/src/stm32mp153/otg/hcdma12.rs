///Register `HCDMA12` reader
pub type R = crate::R<HCDMA12rs>;
///Register `HCDMA12` writer
pub type W = crate::W<HCDMA12rs>;
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
        f.debug_struct("HCDMA12")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA12rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 12 DMA address register in buffer DMA \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`hcdma12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HCDMA12)*/
pub struct HCDMA12rs;
impl crate::RegisterSpec for HCDMA12rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma12::R`](R) reader structure
impl crate::Readable for HCDMA12rs {}
///`write(|w| ..)` method takes [`hcdma12::W`](W) writer structure
impl crate::Writable for HCDMA12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA12 to value 0
impl crate::Resettable for HCDMA12rs {}
