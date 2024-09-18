///Register `GICD_ICACTIVER2` reader
pub type R = crate::R<GICD_ICACTIVER2rs>;
///Register `GICD_ICACTIVER2` writer
pub type W = crate::W<GICD_ICACTIVER2rs>;
///Field `ICACTIVER2` reader - ICACTIVER2
pub type ICACTIVER2_R = crate::FieldReader<u32>;
///Field `ICACTIVER2` writer - ICACTIVER2
pub type ICACTIVER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICACTIVER2
    #[inline(always)]
    pub fn icactiver2(&self) -> ICACTIVER2_R {
        ICACTIVER2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICACTIVER2")
            .field("icactiver2", &self.icactiver2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER2
    #[inline(always)]
    #[must_use]
    pub fn icactiver2(&mut self) -> ICACTIVER2_W<GICD_ICACTIVER2rs> {
        ICACTIVER2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icactiver2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ICACTIVER2)*/
pub struct GICD_ICACTIVER2rs;
impl crate::RegisterSpec for GICD_ICACTIVER2rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icactiver2::R`](R) reader structure
impl crate::Readable for GICD_ICACTIVER2rs {}
///`write(|w| ..)` method takes [`gicd_icactiver2::W`](W) writer structure
impl crate::Writable for GICD_ICACTIVER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICACTIVER2 to value 0
impl crate::Resettable for GICD_ICACTIVER2rs {
    const RESET_VALUE: u32 = 0;
}
