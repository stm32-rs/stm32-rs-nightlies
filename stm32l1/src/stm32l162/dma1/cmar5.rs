///Register `CMAR5` reader
pub type R = crate::R<CMAR5rs>;
///Register `CMAR5` writer
pub type W = crate::W<CMAR5rs>;
///Field `MA` reader - Memory address
pub type MA_R = crate::FieldReader<u32>;
///Field `MA` writer - Memory address
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMAR5").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR5rs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`cmar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#DMA1:CMAR5)*/
pub struct CMAR5rs;
impl crate::RegisterSpec for CMAR5rs {
    type Ux = u32;
}
///`read()` method returns [`cmar5::R`](R) reader structure
impl crate::Readable for CMAR5rs {}
///`write(|w| ..)` method takes [`cmar5::W`](W) writer structure
impl crate::Writable for CMAR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMAR5 to value 0
impl crate::Resettable for CMAR5rs {
    const RESET_VALUE: u32 = 0;
}
