///Register `TZAHB6RSTCLRR` reader
pub type R = crate::R<TZAHB6RSTCLRRrs>;
///Register `TZAHB6RSTCLRR` writer
pub type W = crate::W<TZAHB6RSTCLRRrs>;
///Field `MDMARST` reader - MDMARST
pub type MDMARST_R = crate::BitReader;
///Field `MDMARST` writer - MDMARST
pub type MDMARST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MDMARST
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZAHB6RSTCLRR")
            .field("mdmarst", &self.mdmarst())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMARST
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W<'_, TZAHB6RSTCLRRrs> {
        MDMARST_W::new(self, 0)
    }
}
/**This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`tzahb6rstclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzahb6rstclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:TZAHB6RSTCLRR)*/
pub struct TZAHB6RSTCLRRrs;
impl crate::RegisterSpec for TZAHB6RSTCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`tzahb6rstclrr::R`](R) reader structure
impl crate::Readable for TZAHB6RSTCLRRrs {}
///`write(|w| ..)` method takes [`tzahb6rstclrr::W`](W) writer structure
impl crate::Writable for TZAHB6RSTCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZAHB6RSTCLRR to value 0
impl crate::Resettable for TZAHB6RSTCLRRrs {}
