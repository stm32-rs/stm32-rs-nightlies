///Register `ICACTIVER1` reader
pub type R = crate::R<ICACTIVER1rs>;
///Register `ICACTIVER1` writer
pub type W = crate::W<ICACTIVER1rs>;
///Field `ICACTIVER1` reader - ICACTIVER1
pub type ICACTIVER1_R = crate::FieldReader<u32>;
///Field `ICACTIVER1` writer - ICACTIVER1
pub type ICACTIVER1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER1
    #[inline(always)]
    pub fn icactiver1(&self) -> ICACTIVER1_R {
        ICACTIVER1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER1")
            .field("icactiver1", &self.icactiver1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER1
    #[inline(always)]
    pub fn icactiver1(&mut self) -> ICACTIVER1_W<'_, ICACTIVER1rs> {
        ICACTIVER1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICACTIVER1)*/
pub struct ICACTIVER1rs;
impl crate::RegisterSpec for ICACTIVER1rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver1::R`](R) reader structure
impl crate::Readable for ICACTIVER1rs {}
///`write(|w| ..)` method takes [`icactiver1::W`](W) writer structure
impl crate::Writable for ICACTIVER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER1 to value 0
impl crate::Resettable for ICACTIVER1rs {}
