///Register `BOF_TUNE` reader
pub type R = crate::R<BOF_TUNErs>;
///Register `BOF_TUNE` writer
pub type W = crate::W<BOF_TUNErs>;
///Field `BOF_TUNE` reader - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V
pub type BOF_TUNE_R = crate::FieldReader;
///Field `BOF_TUNE` writer - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V
pub type BOF_TUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V
    #[inline(always)]
    pub fn bof_tune(&self) -> BOF_TUNE_R {
        BOF_TUNE_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOF_TUNE")
            .field("bof_tune", &self.bof_tune())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V
    #[inline(always)]
    pub fn bof_tune(&mut self) -> BOF_TUNE_W<'_, BOF_TUNErs> {
        BOF_TUNE_W::new(self, 0)
    }
}
/**BOF_TUNE register

You can [`read`](crate::Reg::read) this register and get [`bof_tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bof_tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:BOF_TUNE)*/
pub struct BOF_TUNErs;
impl crate::RegisterSpec for BOF_TUNErs {
    type Ux = u32;
}
///`read()` method returns [`bof_tune::R`](R) reader structure
impl crate::Readable for BOF_TUNErs {}
///`write(|w| ..)` method takes [`bof_tune::W`](W) writer structure
impl crate::Writable for BOF_TUNErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOF_TUNE to value 0x04
impl crate::Resettable for BOF_TUNErs {
    const RESET_VALUE: u32 = 0x04;
}
