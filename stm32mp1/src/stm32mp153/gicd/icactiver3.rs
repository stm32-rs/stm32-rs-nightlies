///Register `ICACTIVER3` reader
pub type R = crate::R<ICACTIVER3rs>;
///Register `ICACTIVER3` writer
pub type W = crate::W<ICACTIVER3rs>;
///Field `ICACTIVER3` reader - ICACTIVER3
pub type ICACTIVER3_R = crate::FieldReader<u32>;
///Field `ICACTIVER3` writer - ICACTIVER3
pub type ICACTIVER3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER3
    #[inline(always)]
    pub fn icactiver3(&self) -> ICACTIVER3_R {
        ICACTIVER3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER3")
            .field("icactiver3", &self.icactiver3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER3
    #[inline(always)]
    pub fn icactiver3(&mut self) -> ICACTIVER3_W<'_, ICACTIVER3rs> {
        ICACTIVER3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICACTIVER3)*/
pub struct ICACTIVER3rs;
impl crate::RegisterSpec for ICACTIVER3rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver3::R`](R) reader structure
impl crate::Readable for ICACTIVER3rs {}
///`write(|w| ..)` method takes [`icactiver3::W`](W) writer structure
impl crate::Writable for ICACTIVER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER3 to value 0
impl crate::Resettable for ICACTIVER3rs {}
