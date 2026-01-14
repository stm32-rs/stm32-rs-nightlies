///Register `FIFOR8` reader
pub type R = crate::R<FIFOR8rs>;
///Register `FIFOR8` writer
pub type W = crate::W<FIFOR8rs>;
///Field `FIFODATA` reader - Receive and transmit FIFO data
pub type FIFODATA_R = crate::FieldReader<u32>;
///Field `FIFODATA` writer - Receive and transmit FIFO data
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOR8")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<'_, FIFOR8rs> {
        FIFODATA_W::new(self, 0)
    }
}
/**SDMMC data FIFO registers 8

You can [`read`](crate::Reg::read) this register and get [`fifor8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SDMMC1:FIFOR8)*/
pub struct FIFOR8rs;
impl crate::RegisterSpec for FIFOR8rs {
    type Ux = u32;
}
///`read()` method returns [`fifor8::R`](R) reader structure
impl crate::Readable for FIFOR8rs {}
///`write(|w| ..)` method takes [`fifor8::W`](W) writer structure
impl crate::Writable for FIFOR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOR8 to value 0
impl crate::Resettable for FIFOR8rs {}
