///Register `FIFOR13` reader
pub type R = crate::R<FIFOR13rs>;
///Register `FIFOR13` writer
pub type W = crate::W<FIFOR13rs>;
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
        f.debug_struct("FIFOR13")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<FIFOR13rs> {
        FIFODATA_W::new(self, 0)
    }
}
/**SDMMC data FIFO registers 13

You can [`read`](crate::Reg::read) this register and get [`fifor13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SDMMC1:FIFOR13)*/
pub struct FIFOR13rs;
impl crate::RegisterSpec for FIFOR13rs {
    type Ux = u32;
}
///`read()` method returns [`fifor13::R`](R) reader structure
impl crate::Readable for FIFOR13rs {}
///`write(|w| ..)` method takes [`fifor13::W`](W) writer structure
impl crate::Writable for FIFOR13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOR13 to value 0
impl crate::Resettable for FIFOR13rs {}
