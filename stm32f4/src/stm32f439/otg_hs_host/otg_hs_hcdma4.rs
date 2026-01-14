///Register `OTG_HS_HCDMA4` reader
pub type R = crate::R<OTG_HS_HCDMA4rs>;
///Register `OTG_HS_HCDMA4` writer
pub type W = crate::W<OTG_HS_HCDMA4rs>;
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
        f.debug_struct("OTG_HS_HCDMA4")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, OTG_HS_HCDMA4rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG_HS host channel-4 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_HOST:OTG_HS_HCDMA4)*/
pub struct OTG_HS_HCDMA4rs;
impl crate::RegisterSpec for OTG_HS_HCDMA4rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_hcdma4::R`](R) reader structure
impl crate::Readable for OTG_HS_HCDMA4rs {}
///`write(|w| ..)` method takes [`otg_hs_hcdma4::W`](W) writer structure
impl crate::Writable for OTG_HS_HCDMA4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_HCDMA4 to value 0
impl crate::Resettable for OTG_HS_HCDMA4rs {}
