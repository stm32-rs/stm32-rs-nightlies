///Register `CNDTR7` reader
pub type R = crate::R<CNDTR7rs>;
///Register `CNDTR7` writer
pub type W = crate::W<CNDTR7rs>;
///Field `NDT` reader - number of data to transfer (0 to 218 - 1)
pub type NDT_R = crate::FieldReader<u32>;
///Field `NDT` writer - number of data to transfer (0 to 218 - 1)
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32, crate::Safe>;
impl R {
    ///Bits 0:17 - number of data to transfer (0 to 218 - 1)
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR7").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:17 - number of data to transfer (0 to 218 - 1)
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<CNDTR7rs> {
        NDT_W::new(self, 0)
    }
}
/**channel x number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`cndtr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#DMA1:CNDTR7)*/
pub struct CNDTR7rs;
impl crate::RegisterSpec for CNDTR7rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr7::R`](R) reader structure
impl crate::Readable for CNDTR7rs {}
///`write(|w| ..)` method takes [`cndtr7::W`](W) writer structure
impl crate::Writable for CNDTR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR7 to value 0
impl crate::Resettable for CNDTR7rs {
    const RESET_VALUE: u32 = 0;
}
