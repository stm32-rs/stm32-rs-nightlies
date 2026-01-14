///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2").field("mms", &self.mms()).finish()
    }
}
impl W {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, CR2rs> {
        MMS_W::new(self, 4)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#TIM9:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
