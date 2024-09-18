///Register `QUADSPI_LPTR` reader
pub type R = crate::R<QUADSPI_LPTRrs>;
///Register `QUADSPI_LPTR` writer
pub type W = crate::W<QUADSPI_LPTRrs>;
///Field `TIMEOUT` reader - TIMEOUT
pub type TIMEOUT_R = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - TIMEOUT
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - TIMEOUT
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_LPTR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TIMEOUT
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<QUADSPI_LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
/**QUADSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`quadspi_lptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_lptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_LPTR)*/
pub struct QUADSPI_LPTRrs;
impl crate::RegisterSpec for QUADSPI_LPTRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_lptr::R`](R) reader structure
impl crate::Readable for QUADSPI_LPTRrs {}
///`write(|w| ..)` method takes [`quadspi_lptr::W`](W) writer structure
impl crate::Writable for QUADSPI_LPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_LPTR to value 0
impl crate::Resettable for QUADSPI_LPTRrs {
    const RESET_VALUE: u32 = 0;
}
