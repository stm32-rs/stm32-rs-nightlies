///Register `MACTSICNR` reader
pub type R = crate::R<MACTSICNRrs>;
///Register `MACTSICNR` writer
pub type W = crate::W<MACTSICNRrs>;
///Field `TSIC` reader - Timestamp Ingress Correction
pub type TSIC_R = crate::FieldReader<u32>;
///Field `TSIC` writer - Timestamp Ingress Correction
pub type TSIC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Timestamp Ingress Correction
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSICNR")
            .field("tsic", &self.tsic())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Timestamp Ingress Correction
    #[inline(always)]
    pub fn tsic(&mut self) -> TSIC_W<'_, MACTSICNRrs> {
        TSIC_W::new(self, 0)
    }
}
/**Timestamp Ingress correction nanosecond register

You can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACTSICNR)*/
pub struct MACTSICNRrs;
impl crate::RegisterSpec for MACTSICNRrs {
    type Ux = u32;
}
///`read()` method returns [`mactsicnr::R`](R) reader structure
impl crate::Readable for MACTSICNRrs {}
///`write(|w| ..)` method takes [`mactsicnr::W`](W) writer structure
impl crate::Writable for MACTSICNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSICNR to value 0
impl crate::Resettable for MACTSICNRrs {}
