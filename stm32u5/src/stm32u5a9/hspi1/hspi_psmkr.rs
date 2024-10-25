///Register `HSPI_PSMKR` reader
pub type R = crate::R<HSPI_PSMKRrs>;
///Register `HSPI_PSMKR` writer
pub type W = crate::W<HSPI_PSMKRrs>;
///Field `MASK` reader - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
pub type MASK_R = crate::FieldReader<u32>;
///Field `MASK` writer - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_PSMKR")
            .field("mask", &self.mask())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Status mask Mask to be applied to the status bytes received in Polling mode For bit n:
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<HSPI_PSMKRrs> {
        MASK_W::new(self, 0)
    }
}
/**HSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`hspi_psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:HSPI_PSMKR)*/
pub struct HSPI_PSMKRrs;
impl crate::RegisterSpec for HSPI_PSMKRrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_psmkr::R`](R) reader structure
impl crate::Readable for HSPI_PSMKRrs {}
///`write(|w| ..)` method takes [`hspi_psmkr::W`](W) writer structure
impl crate::Writable for HSPI_PSMKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_PSMKR to value 0
impl crate::Resettable for HSPI_PSMKRrs {
    const RESET_VALUE: u32 = 0;
}
