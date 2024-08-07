///Register `QUADSPI_PSMAR` reader
pub type R = crate::R<QUADSPI_PSMARrs>;
///Register `QUADSPI_PSMAR` writer
pub type W = crate::W<QUADSPI_PSMARrs>;
///Field `MATCH` reader - MATCH
pub type MATCH_R = crate::FieldReader<u32>;
///Field `MATCH` writer - MATCH
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MATCH
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_PSMAR")
            .field("match_", &self.match_())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MATCH
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<QUADSPI_PSMARrs> {
        MATCH_W::new(self, 0)
    }
}
/**QUADSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`quadspi_psmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#QUADSPI:QUADSPI_PSMAR)*/
pub struct QUADSPI_PSMARrs;
impl crate::RegisterSpec for QUADSPI_PSMARrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_psmar::R`](R) reader structure
impl crate::Readable for QUADSPI_PSMARrs {}
///`write(|w| ..)` method takes [`quadspi_psmar::W`](W) writer structure
impl crate::Writable for QUADSPI_PSMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_PSMAR to value 0
impl crate::Resettable for QUADSPI_PSMARrs {
    const RESET_VALUE: u32 = 0;
}
