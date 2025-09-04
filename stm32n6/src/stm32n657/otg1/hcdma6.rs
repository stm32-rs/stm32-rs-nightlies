///Register `HCDMA6` reader
pub type R = crate::R<HCDMA6rs>;
///Register `HCDMA6` writer
pub type W = crate::W<HCDMA6rs>;
///Field `DMAADDR` reader - DMA address
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMA address
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA6")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA6rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 6 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:HCDMA6)*/
pub struct HCDMA6rs;
impl crate::RegisterSpec for HCDMA6rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma6::R`](R) reader structure
impl crate::Readable for HCDMA6rs {}
///`write(|w| ..)` method takes [`hcdma6::W`](W) writer structure
impl crate::Writable for HCDMA6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA6 to value 0
impl crate::Resettable for HCDMA6rs {}
