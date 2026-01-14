///Register `ITIP` reader
pub type R = crate::R<ITIPrs>;
///Register `ITIP` writer
pub type W = crate::W<ITIPrs>;
///Field `ITIP` reader - Integration-test input register
pub type ITIP_R = crate::BitReader;
///Field `ITIP` writer - Integration-test input register
pub type ITIP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Integration-test input register
    #[inline(always)]
    pub fn itip(&self) -> ITIP_R {
        ITIP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITIP").field("itip", &self.itip()).finish()
    }
}
impl W {
    ///Bit 0 - Integration-test input register
    #[inline(always)]
    pub fn itip(&mut self) -> ITIP_W<'_, ITIPrs> {
        ITIP_W::new(self, 0)
    }
}
/**RNG_ITIP register

You can [`read`](crate::Reg::read) this register and get [`itip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:ITIP)*/
pub struct ITIPrs;
impl crate::RegisterSpec for ITIPrs {
    type Ux = u32;
}
///`read()` method returns [`itip::R`](R) reader structure
impl crate::Readable for ITIPrs {}
///`write(|w| ..)` method takes [`itip::W`](W) writer structure
impl crate::Writable for ITIPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ITIP to value 0
impl crate::Resettable for ITIPrs {}
