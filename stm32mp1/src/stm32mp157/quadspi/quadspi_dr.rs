///Register `QUADSPI_DR` reader
pub type R = crate::R<QUADSPI_DRrs>;
///Register `QUADSPI_DR` writer
pub type W = crate::W<QUADSPI_DRrs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_DR")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<QUADSPI_DRrs> {
        DATA_W::new(self, 0)
    }
}
/**QUADSPI data register

You can [`read`](crate::Reg::read) this register and get [`quadspi_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_DR)*/
pub struct QUADSPI_DRrs;
impl crate::RegisterSpec for QUADSPI_DRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_dr::R`](R) reader structure
impl crate::Readable for QUADSPI_DRrs {}
///`write(|w| ..)` method takes [`quadspi_dr::W`](W) writer structure
impl crate::Writable for QUADSPI_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_DR to value 0
impl crate::Resettable for QUADSPI_DRrs {
    const RESET_VALUE: u32 = 0;
}
