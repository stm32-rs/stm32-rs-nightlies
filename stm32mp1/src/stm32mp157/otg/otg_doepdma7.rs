///Register `OTG_DOEPDMA7` reader
pub type R = crate::R<OTG_DOEPDMA7rs>;
///Register `OTG_DOEPDMA7` writer
pub type W = crate::W<OTG_DOEPDMA7rs>;
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
        f.debug_struct("OTG_DOEPDMA7")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<OTG_DOEPDMA7rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device OUT endpoint 7 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_doepdma7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_doepdma7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_DOEPDMA7)*/
pub struct OTG_DOEPDMA7rs;
impl crate::RegisterSpec for OTG_DOEPDMA7rs {
    type Ux = u32;
}
///`read()` method returns [`otg_doepdma7::R`](R) reader structure
impl crate::Readable for OTG_DOEPDMA7rs {}
///`write(|w| ..)` method takes [`otg_doepdma7::W`](W) writer structure
impl crate::Writable for OTG_DOEPDMA7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DOEPDMA7 to value 0
impl crate::Resettable for OTG_DOEPDMA7rs {
    const RESET_VALUE: u32 = 0;
}
