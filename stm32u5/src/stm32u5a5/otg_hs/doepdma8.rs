///Register `DOEPDMA8` reader
pub type R = crate::R<DOEPDMA8rs>;
///Register `DOEPDMA8` writer
pub type W = crate::W<DOEPDMA8rs>;
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
        f.debug_struct("DOEPDMA8")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAADDR
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DOEPDMA8rs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG device OUT endpoint 8 DMA address register

You can [`read`](crate::Reg::read) this register and get [`doepdma8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:DOEPDMA8)*/
pub struct DOEPDMA8rs;
impl crate::RegisterSpec for DOEPDMA8rs {
    type Ux = u32;
}
///`read()` method returns [`doepdma8::R`](R) reader structure
impl crate::Readable for DOEPDMA8rs {}
///`write(|w| ..)` method takes [`doepdma8::W`](W) writer structure
impl crate::Writable for DOEPDMA8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOEPDMA8 to value 0
impl crate::Resettable for DOEPDMA8rs {
    const RESET_VALUE: u32 = 0;
}