///Register `GICC_PMR` reader
pub type R = crate::R<GICC_PMRrs>;
///Register `GICC_PMR` writer
pub type W = crate::W<GICC_PMRrs>;
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
        f.debug_struct("GICC_PMR")
            .field("priority", &self.priority())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICC_PMRrs> {
        PRIORITY_W::new(self, 3)
    }
}
/**GICC input priority mask register

You can [`read`](crate::Reg::read) this register and get [`gicc_pmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_pmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_PMR)*/
pub struct GICC_PMRrs;
impl crate::RegisterSpec for GICC_PMRrs {
    type Ux = u32;
}
///`read()` method returns [`gicc_pmr::R`](R) reader structure
impl crate::Readable for GICC_PMRrs {}
///`write(|w| ..)` method takes [`gicc_pmr::W`](W) writer structure
impl crate::Writable for GICC_PMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICC_PMR to value 0
impl crate::Resettable for GICC_PMRrs {
    const RESET_VALUE: u32 = 0;
}
