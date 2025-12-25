///Register `FIFO` reader
pub type R = crate::R<FIFOrs>;
///Register `FIFO` writer
pub type W = crate::W<FIFOrs>;
///Field `FIFOData` reader - Receive and transmit FIFO data
pub type FIFODATA_R = crate::FieldReader<u32>;
///Field `FIFOData` writer - Receive and transmit FIFO data
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<'_, FIFOrs> {
        FIFODATA_W::new(self, 0)
    }
}
/**data FIFO register

You can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#SDMMC1:FIFO)*/
pub struct FIFOrs;
impl crate::RegisterSpec for FIFOrs {
    type Ux = u32;
}
///`read()` method returns [`fifo::R`](R) reader structure
impl crate::Readable for FIFOrs {}
///`write(|w| ..)` method takes [`fifo::W`](W) writer structure
impl crate::Writable for FIFOrs {
    type Safety = crate::Safe;
}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFOrs {}
