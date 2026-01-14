///Register `ICACTIVER8` reader
pub type R = crate::R<ICACTIVER8rs>;
///Register `ICACTIVER8` writer
pub type W = crate::W<ICACTIVER8rs>;
///Field `ICACTIVER8` reader - ICACTIVER8
pub type ICACTIVER8_R = crate::FieldReader<u32>;
///Field `ICACTIVER8` writer - ICACTIVER8
pub type ICACTIVER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER8
    #[inline(always)]
    pub fn icactiver8(&self) -> ICACTIVER8_R {
        ICACTIVER8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER8")
            .field("icactiver8", &self.icactiver8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER8
    #[inline(always)]
    pub fn icactiver8(&mut self) -> ICACTIVER8_W<'_, ICACTIVER8rs> {
        ICACTIVER8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER8)*/
pub struct ICACTIVER8rs;
impl crate::RegisterSpec for ICACTIVER8rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver8::R`](R) reader structure
impl crate::Readable for ICACTIVER8rs {}
///`write(|w| ..)` method takes [`icactiver8::W`](W) writer structure
impl crate::Writable for ICACTIVER8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER8 to value 0
impl crate::Resettable for ICACTIVER8rs {}
