///Register `HCDMA15` reader
pub type R = crate::R<HCDMA15rs>;
///Register `HCDMA15` writer
pub type W = crate::W<HCDMA15rs>;
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
        f.debug_struct("HCDMA15")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, HCDMA15rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 15 DMA address register

You can [`read`](crate::Reg::read) this register and get [`hcdma15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:HCDMA15)*/
pub struct HCDMA15rs;
impl crate::RegisterSpec for HCDMA15rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma15::R`](R) reader structure
impl crate::Readable for HCDMA15rs {}
///`write(|w| ..)` method takes [`hcdma15::W`](W) writer structure
impl crate::Writable for HCDMA15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCDMA15 to value 0
impl crate::Resettable for HCDMA15rs {}
