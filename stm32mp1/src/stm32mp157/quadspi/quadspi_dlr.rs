///Register `QUADSPI_DLR` reader
pub type R = crate::R<QUADSPI_DLRrs>;
///Register `QUADSPI_DLR` writer
pub type W = crate::W<QUADSPI_DLRrs>;
///Field `DL` reader - DL
pub type DL_R = crate::FieldReader<u32>;
///Field `DL` writer - DL
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DL
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_DLR")
            .field("dl", &self.dl())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DL
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<QUADSPI_DLRrs> {
        DL_W::new(self, 0)
    }
}
/**QUADSPI data length register

You can [`read`](crate::Reg::read) this register and get [`quadspi_dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:QUADSPI_DLR)*/
pub struct QUADSPI_DLRrs;
impl crate::RegisterSpec for QUADSPI_DLRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_dlr::R`](R) reader structure
impl crate::Readable for QUADSPI_DLRrs {}
///`write(|w| ..)` method takes [`quadspi_dlr::W`](W) writer structure
impl crate::Writable for QUADSPI_DLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_DLR to value 0
impl crate::Resettable for QUADSPI_DLRrs {
    const RESET_VALUE: u32 = 0;
}
