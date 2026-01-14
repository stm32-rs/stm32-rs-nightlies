///Register `CNT32` reader
pub type R = crate::R<CNT32rs>;
///Register `CNT32` writer
pub type W = crate::W<CNT32rs>;
///Field `CNT` reader - counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT32").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNT32rs> {
        CNT_W::new(self, 0)
    }
}
/**32-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#TIM5:CNT32)*/
pub struct CNT32rs;
impl crate::RegisterSpec for CNT32rs {
    type Ux = u32;
}
///`read()` method returns [`cnt32::R`](R) reader structure
impl crate::Readable for CNT32rs {}
///`write(|w| ..)` method takes [`cnt32::W`](W) writer structure
impl crate::Writable for CNT32rs {
    type Safety = crate::Safe;
}
///`reset()` method sets CNT32 to value 0
impl crate::Resettable for CNT32rs {}
