///Register `ICACTIVER4` reader
pub type R = crate::R<ICACTIVER4rs>;
///Register `ICACTIVER4` writer
pub type W = crate::W<ICACTIVER4rs>;
///Field `ICACTIVER4` reader - ICACTIVER4
pub type ICACTIVER4_R = crate::FieldReader<u32>;
///Field `ICACTIVER4` writer - ICACTIVER4
pub type ICACTIVER4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER4
    #[inline(always)]
    pub fn icactiver4(&self) -> ICACTIVER4_R {
        ICACTIVER4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER4")
            .field("icactiver4", &self.icactiver4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER4
    #[inline(always)]
    pub fn icactiver4(&mut self) -> ICACTIVER4_W<'_, ICACTIVER4rs> {
        ICACTIVER4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER4)*/
pub struct ICACTIVER4rs;
impl crate::RegisterSpec for ICACTIVER4rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver4::R`](R) reader structure
impl crate::Readable for ICACTIVER4rs {}
///`write(|w| ..)` method takes [`icactiver4::W`](W) writer structure
impl crate::Writable for ICACTIVER4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER4 to value 0
impl crate::Resettable for ICACTIVER4rs {}
