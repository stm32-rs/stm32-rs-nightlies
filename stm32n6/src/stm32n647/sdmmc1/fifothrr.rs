///Register `FIFOTHRR` reader
pub type R = crate::R<FIFOTHRRrs>;
///Register `FIFOTHRR` writer
pub type W = crate::W<FIFOTHRRrs>;
///Field `THR` reader - FIFO threshold
pub type THR_R = crate::FieldReader;
///Field `THR` writer - FIFO threshold
pub type THR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - FIFO threshold
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOTHRR")
            .field("thr", &self.thr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - FIFO threshold
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W<'_, FIFOTHRRrs> {
        THR_W::new(self, 0)
    }
}
/**SDMMC data FIFO threshold register

You can [`read`](crate::Reg::read) this register and get [`fifothrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SDMMC1:FIFOTHRR)*/
pub struct FIFOTHRRrs;
impl crate::RegisterSpec for FIFOTHRRrs {
    type Ux = u32;
}
///`read()` method returns [`fifothrr::R`](R) reader structure
impl crate::Readable for FIFOTHRRrs {}
///`write(|w| ..)` method takes [`fifothrr::W`](W) writer structure
impl crate::Writable for FIFOTHRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOTHRR to value 0
impl crate::Resettable for FIFOTHRRrs {}
