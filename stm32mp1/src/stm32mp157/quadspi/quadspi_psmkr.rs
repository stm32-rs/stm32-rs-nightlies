///Register `QUADSPI_PSMKR` reader
pub type R = crate::R<QUADSPI_PSMKRrs>;
///Register `QUADSPI_PSMKR` writer
pub type W = crate::W<QUADSPI_PSMKRrs>;
///Field `MASK` reader - MASK
pub type MASK_R = crate::FieldReader<u32>;
///Field `MASK` writer - MASK
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MASK
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_PSMKR")
            .field("mask", &self.mask())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MASK
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<QUADSPI_PSMKRrs> {
        MASK_W::new(self, 0)
    }
}
/**QUADSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`quadspi_psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_PSMKR)*/
pub struct QUADSPI_PSMKRrs;
impl crate::RegisterSpec for QUADSPI_PSMKRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_psmkr::R`](R) reader structure
impl crate::Readable for QUADSPI_PSMKRrs {}
///`write(|w| ..)` method takes [`quadspi_psmkr::W`](W) writer structure
impl crate::Writable for QUADSPI_PSMKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_PSMKR to value 0
impl crate::Resettable for QUADSPI_PSMKRrs {
    const RESET_VALUE: u32 = 0;
}
