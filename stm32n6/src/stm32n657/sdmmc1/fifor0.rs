///Register `FIFOR0` reader
pub type R = crate::R<FIFOR0rs>;
///Register `FIFOR0` writer
pub type W = crate::W<FIFOR0rs>;
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
        f.debug_struct("FIFOR0")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<'_, FIFOR0rs> {
        FIFODATA_W::new(self, 0)
    }
}
/**SDMMC data FIFO registers 0

You can [`read`](crate::Reg::read) this register and get [`fifor0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SDMMC1:FIFOR0)*/
pub struct FIFOR0rs;
impl crate::RegisterSpec for FIFOR0rs {
    type Ux = u32;
}
///`read()` method returns [`fifor0::R`](R) reader structure
impl crate::Readable for FIFOR0rs {}
///`write(|w| ..)` method takes [`fifor0::W`](W) writer structure
impl crate::Writable for FIFOR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOR0 to value 0
impl crate::Resettable for FIFOR0rs {}
