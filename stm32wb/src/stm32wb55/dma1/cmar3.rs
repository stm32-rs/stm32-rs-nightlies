///Register `CMAR3` reader
pub type R = crate::R<CMAR3rs>;
///Register `CMAR3` writer
pub type W = crate::W<CMAR3rs>;
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
        f.debug_struct("CMAR3").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR3rs> {
        MA_W::new(self, 0)
    }
}
/**channel x memory address register

You can [`read`](crate::Reg::read) this register and get [`cmar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA1:CMAR3)*/
pub struct CMAR3rs;
impl crate::RegisterSpec for CMAR3rs {
    type Ux = u32;
}
///`read()` method returns [`cmar3::R`](R) reader structure
impl crate::Readable for CMAR3rs {}
///`write(|w| ..)` method takes [`cmar3::W`](W) writer structure
impl crate::Writable for CMAR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMAR3 to value 0
impl crate::Resettable for CMAR3rs {
    const RESET_VALUE: u32 = 0;
}
