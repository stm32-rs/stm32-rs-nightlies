///Register `GICD_ICACTIVER0` reader
pub type R = crate::R<GICD_ICACTIVER0rs>;
///Register `GICD_ICACTIVER0` writer
pub type W = crate::W<GICD_ICACTIVER0rs>;
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
        f.debug_struct("GICD_ICACTIVER0")
            .field("icactiver0", &self.icactiver0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER0
    #[inline(always)]
    #[must_use]
    pub fn icactiver0(&mut self) -> ICACTIVER0_W<GICD_ICACTIVER0rs> {
        ICACTIVER0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icactiver0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ICACTIVER0)*/
pub struct GICD_ICACTIVER0rs;
impl crate::RegisterSpec for GICD_ICACTIVER0rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icactiver0::R`](R) reader structure
impl crate::Readable for GICD_ICACTIVER0rs {}
///`write(|w| ..)` method takes [`gicd_icactiver0::W`](W) writer structure
impl crate::Writable for GICD_ICACTIVER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICACTIVER0 to value 0
impl crate::Resettable for GICD_ICACTIVER0rs {
    const RESET_VALUE: u32 = 0;
}
