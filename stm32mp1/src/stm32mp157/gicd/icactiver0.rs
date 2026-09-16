///Register `ICACTIVER0` reader
pub type R = crate::R<ICACTIVER0rs>;
///Register `ICACTIVER0` writer
pub type W = crate::W<ICACTIVER0rs>;
///Field `ICACTIVER0` reader - ICACTIVER0
pub type ICACTIVER0_R = crate::FieldReader<u32>;
///Field `ICACTIVER0` writer - ICACTIVER0
pub type ICACTIVER0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER0
    #[inline(always)]
    pub fn icactiver0(&self) -> ICACTIVER0_R {
        ICACTIVER0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACTIVER0")
            .field("icactiver0", &self.icactiver0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER0
    #[inline(always)]
    pub fn icactiver0(&mut self) -> ICACTIVER0_W<'_, ICACTIVER0rs> {
        ICACTIVER0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER0)*/
pub struct ICACTIVER0rs;
impl crate::RegisterSpec for ICACTIVER0rs {
    type Ux = u32;
}
///`read()` method returns [`icactiver0::R`](R) reader structure
impl crate::Readable for ICACTIVER0rs {}
///`write(|w| ..)` method takes [`icactiver0::W`](W) writer structure
impl crate::Writable for ICACTIVER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICACTIVER0 to value 0
impl crate::Resettable for ICACTIVER0rs {}
