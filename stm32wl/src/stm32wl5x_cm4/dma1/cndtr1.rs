///Register `CNDTR1` reader
pub type R = crate::R<CNDTR1rs>;
///Register `CNDTR1` writer
pub type W = crate::W<CNDTR1rs>;
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
        f.debug_struct("CNDTR1").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:17 - number of data to transfer (0 to 218 - 1)
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<CNDTR1rs> {
        NDT_W::new(self, 0)
    }
}
/**channel x number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`cndtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DMA1:CNDTR1)*/
pub struct CNDTR1rs;
impl crate::RegisterSpec for CNDTR1rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr1::R`](R) reader structure
impl crate::Readable for CNDTR1rs {}
///`write(|w| ..)` method takes [`cndtr1::W`](W) writer structure
impl crate::Writable for CNDTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR1 to value 0
impl crate::Resettable for CNDTR1rs {
    const RESET_VALUE: u32 = 0;
}
