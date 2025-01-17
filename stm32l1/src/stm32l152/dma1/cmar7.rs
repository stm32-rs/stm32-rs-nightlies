///Register `CMAR7` reader
pub type R = crate::R<CMAR7rs>;
///Register `CMAR7` writer
pub type W = crate::W<CMAR7rs>;
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
        f.debug_struct("CMAR7").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<CMAR7rs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`cmar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#DMA1:CMAR7)*/
pub struct CMAR7rs;
impl crate::RegisterSpec for CMAR7rs {
    type Ux = u32;
}
///`read()` method returns [`cmar7::R`](R) reader structure
impl crate::Readable for CMAR7rs {}
///`write(|w| ..)` method takes [`cmar7::W`](W) writer structure
impl crate::Writable for CMAR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMAR7 to value 0
impl crate::Resettable for CMAR7rs {
    const RESET_VALUE: u32 = 0;
}
