///Register `ICACTIVER5` reader
pub type R = crate::R<ICACTIVER5rs>;
///Register `ICACTIVER5` writer
pub type W = crate::W<ICACTIVER5rs>;
///Field `ICACTIVER5` reader - ICACTIVER5
pub type ICACTIVER5_R = crate::FieldReader<u32>;
///Field `ICACTIVER5` writer - ICACTIVER5
pub type ICACTIVER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER5
    #[inline(always)]
    pub fn icactiver5(&self) -> ICACTIVER5_R {
        ICACTIVER5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER5")
            .field("icactiver5", &self.icactiver5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER5
    #[inline(always)]
    pub fn icactiver5(&mut self) -> ICACTIVER5_W<'_, ICACTIVER5rs> {
        ICACTIVER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICACTIVER5)*/
pub struct ICACTIVER5rs;
impl crate::RegisterSpec for ICACTIVER5rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver5::R`](R) reader structure
impl crate::Readable for ICACTIVER5rs {}
///`write(|w| ..)` method takes [`icactiver5::W`](W) writer structure
impl crate::Writable for ICACTIVER5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER5 to value 0
impl crate::Resettable for ICACTIVER5rs {}
