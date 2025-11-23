///Register `ICACTIVER6` reader
pub type R = crate::R<ICACTIVER6rs>;
///Register `ICACTIVER6` writer
pub type W = crate::W<ICACTIVER6rs>;
///Field `ICACTIVER6` reader - ICACTIVER6
pub type ICACTIVER6_R = crate::FieldReader<u32>;
///Field `ICACTIVER6` writer - ICACTIVER6
pub type ICACTIVER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER6
    #[inline(always)]
    pub fn icactiver6(&self) -> ICACTIVER6_R {
        ICACTIVER6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER6")
            .field("icactiver6", &self.icactiver6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER6
    #[inline(always)]
    pub fn icactiver6(&mut self) -> ICACTIVER6_W<'_, ICACTIVER6rs> {
        ICACTIVER6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICACTIVER6)*/
pub struct ICACTIVER6rs;
impl crate::RegisterSpec for ICACTIVER6rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver6::R`](R) reader structure
impl crate::Readable for ICACTIVER6rs {}
///`write(|w| ..)` method takes [`icactiver6::W`](W) writer structure
impl crate::Writable for ICACTIVER6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER6 to value 0
impl crate::Resettable for ICACTIVER6rs {}
