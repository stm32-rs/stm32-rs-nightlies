///Register `HCDMA1` reader
pub type R = crate::R<HCDMA1rs>;
///Register `HCDMA1` writer
pub type W = crate::W<HCDMA1rs>;
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
        f.debug_struct("HCDMA1")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA1rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG host channel 1 DMA address register in buffer DMA \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`hcdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HCDMA1)*/
pub struct HCDMA1rs;
impl crate::RegisterSpec for HCDMA1rs {
    type Ux = u32;
}
///`read()` method returns [`hcdma1::R`](R) reader structure
impl crate::Readable for HCDMA1rs {}
///`write(|w| ..)` method takes [`hcdma1::W`](W) writer structure
impl crate::Writable for HCDMA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HCDMA1 to value 0
impl crate::Resettable for HCDMA1rs {
    const RESET_VALUE: u32 = 0;
}