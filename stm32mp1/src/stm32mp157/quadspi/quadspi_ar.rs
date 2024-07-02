///Register `QUADSPI_AR` reader
pub type R = crate::R<QUADSPI_ARrs>;
///Register `QUADSPI_AR` writer
pub type W = crate::W<QUADSPI_ARrs>;
///Field `ADDRESS` reader - ADDRESS
pub type ADDRESS_R = crate::FieldReader<u32>;
///Field `ADDRESS` writer - ADDRESS
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ADDRESS
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_AR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ADDRESS
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<QUADSPI_ARrs> {
        ADDRESS_W::new(self, 0)
    }
}
/**QUADSPI address register

You can [`read`](crate::Reg::read) this register and get [`quadspi_ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_AR)*/
pub struct QUADSPI_ARrs;
impl crate::RegisterSpec for QUADSPI_ARrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_ar::R`](R) reader structure
impl crate::Readable for QUADSPI_ARrs {}
///`write(|w| ..)` method takes [`quadspi_ar::W`](W) writer structure
impl crate::Writable for QUADSPI_ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_AR to value 0
impl crate::Resettable for QUADSPI_ARrs {
    const RESET_VALUE: u32 = 0;
}
