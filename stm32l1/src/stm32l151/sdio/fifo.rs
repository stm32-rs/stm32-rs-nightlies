///Register `FIFO` reader
pub type R = crate::R<FIFOrs>;
///Register `FIFO` writer
pub type W = crate::W<FIFOrs>;
///Field `FIF0Data` reader - FIF0Data
pub type FIF0DATA_R = crate::FieldReader<u32>;
///Field `FIF0Data` writer - FIF0Data
pub type FIF0DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - FIF0Data
    #[inline(always)]
    pub fn fif0data(&self) -> FIF0DATA_R {
        FIF0DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO")
            .field("fif0data", &self.fif0data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - FIF0Data
    #[inline(always)]
    pub fn fif0data(&mut self) -> FIF0DATA_W<FIFOrs> {
        FIF0DATA_W::new(self, 0)
    }
}
/**data FIFO register

You can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#SDIO:FIFO)*/
pub struct FIFOrs;
impl crate::RegisterSpec for FIFOrs {
    type Ux = u32;
}
///`read()` method returns [`fifo::R`](R) reader structure
impl crate::Readable for FIFOrs {}
///`write(|w| ..)` method takes [`fifo::W`](W) writer structure
impl crate::Writable for FIFOrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFOrs {}
