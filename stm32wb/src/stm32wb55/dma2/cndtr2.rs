///Register `CNDTR2` reader
pub type R = crate::R<CNDTR2rs>;
///Register `CNDTR2` writer
pub type W = crate::W<CNDTR2rs>;
///Field `NDT` reader - Number of data to transfer
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - Number of data to transfer
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR2").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<CNDTR2rs> {
        NDT_W::new(self, 0)
    }
}
/**channel x number of data register

You can [`read`](crate::Reg::read) this register and get [`cndtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA2:CNDTR2)*/
pub struct CNDTR2rs;
impl crate::RegisterSpec for CNDTR2rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr2::R`](R) reader structure
impl crate::Readable for CNDTR2rs {}
///`write(|w| ..)` method takes [`cndtr2::W`](W) writer structure
impl crate::Writable for CNDTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNDTR2 to value 0
impl crate::Resettable for CNDTR2rs {
    const RESET_VALUE: u32 = 0;
}
