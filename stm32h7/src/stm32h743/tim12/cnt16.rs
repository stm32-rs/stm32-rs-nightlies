///Register `CNT16` reader
pub type R = crate::R<CNT16rs>;
///Register `CNT16` writer
pub type W = crate::W<CNT16rs>;
///Field `CNT` reader - counter value
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT16").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNT16rs> {
        CNT_W::new(self, 0)
    }
}
/**16-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#TIM12:CNT16)*/
pub struct CNT16rs;
impl crate::RegisterSpec for CNT16rs {
    type Ux = u16;
}
///`read()` method returns [`cnt16::R`](R) reader structure
impl crate::Readable for CNT16rs {}
///`write(|w| ..)` method takes [`cnt16::W`](W) writer structure
impl crate::Writable for CNT16rs {
    type Safety = crate::Safe;
}
///`reset()` method sets CNT16 to value 0
impl crate::Resettable for CNT16rs {}
