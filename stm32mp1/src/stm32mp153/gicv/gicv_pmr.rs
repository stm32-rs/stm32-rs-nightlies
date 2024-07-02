///Register `GICV_PMR` reader
pub type R = crate::R<GICV_PMRrs>;
///Register `GICV_PMR` writer
pub type W = crate::W<GICV_PMRrs>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader;
///Field `PRIORITY` writer - PRIORITY
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICV_PMR")
            .field("priority", &self.priority())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICV_PMRrs> {
        PRIORITY_W::new(self, 3)
    }
}
/**GICV VM priority mask register

You can [`read`](crate::Reg::read) this register and get [`gicv_pmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_pmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_PMR)*/
pub struct GICV_PMRrs;
impl crate::RegisterSpec for GICV_PMRrs {
    type Ux = u32;
}
///`read()` method returns [`gicv_pmr::R`](R) reader structure
impl crate::Readable for GICV_PMRrs {}
///`write(|w| ..)` method takes [`gicv_pmr::W`](W) writer structure
impl crate::Writable for GICV_PMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICV_PMR to value 0
impl crate::Resettable for GICV_PMRrs {
    const RESET_VALUE: u32 = 0;
}
