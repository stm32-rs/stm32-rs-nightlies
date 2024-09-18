///Register `QUADSPI_ABR` reader
pub type R = crate::R<QUADSPI_ABRrs>;
///Register `QUADSPI_ABR` writer
pub type W = crate::W<QUADSPI_ABRrs>;
///Field `ALTERNATE` reader - ALTERNATE
pub type ALTERNATE_R = crate::FieldReader<u32>;
///Field `ALTERNATE` writer - ALTERNATE
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_ABR")
            .field("alternate", &self.alternate())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<QUADSPI_ABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
/**QUADSPI alternate bytes registers

You can [`read`](crate::Reg::read) this register and get [`quadspi_abr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_abr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_ABR)*/
pub struct QUADSPI_ABRrs;
impl crate::RegisterSpec for QUADSPI_ABRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_abr::R`](R) reader structure
impl crate::Readable for QUADSPI_ABRrs {}
///`write(|w| ..)` method takes [`quadspi_abr::W`](W) writer structure
impl crate::Writable for QUADSPI_ABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_ABR to value 0
impl crate::Resettable for QUADSPI_ABRrs {
    const RESET_VALUE: u32 = 0;
}
