///Register `ICACTIVER7` reader
pub type R = crate::R<ICACTIVER7rs>;
///Register `ICACTIVER7` writer
pub type W = crate::W<ICACTIVER7rs>;
///Field `ICACTIVER7` reader - ICACTIVER7
pub type ICACTIVER7_R = crate::FieldReader<u32>;
///Field `ICACTIVER7` writer - ICACTIVER7
pub type ICACTIVER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER7
    #[inline(always)]
    pub fn icactiver7(&self) -> ICACTIVER7_R {
        ICACTIVER7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER7")
            .field("icactiver7", &self.icactiver7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER7
    #[inline(always)]
    pub fn icactiver7(&mut self) -> ICACTIVER7_W<'_, ICACTIVER7rs> {
        ICACTIVER7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICACTIVER7)*/
pub struct ICACTIVER7rs;
impl crate::RegisterSpec for ICACTIVER7rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver7::R`](R) reader structure
impl crate::Readable for ICACTIVER7rs {}
///`write(|w| ..)` method takes [`icactiver7::W`](W) writer structure
impl crate::Writable for ICACTIVER7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER7 to value 0
impl crate::Resettable for ICACTIVER7rs {}
