///Register `RCC_STGENCKSELR` reader
pub type R = crate::R<RCC_STGENCKSELRrs>;
///Register `RCC_STGENCKSELR` writer
pub type W = crate::W<RCC_STGENCKSELRrs>;
///Field `STGENSRC` reader - STGENSRC
pub type STGENSRC_R = crate::FieldReader;
///Field `STGENSRC` writer - STGENSRC
pub type STGENSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - STGENSRC
    #[inline(always)]
    pub fn stgensrc(&self) -> STGENSRC_R {
        STGENSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_STGENCKSELR")
            .field("stgensrc", &self.stgensrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - STGENSRC
    #[inline(always)]
    #[must_use]
    pub fn stgensrc(&mut self) -> STGENSRC_W<RCC_STGENCKSELRrs> {
        STGENSRC_W::new(self, 0)
    }
}
/**This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rcc_stgenckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_stgenckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_STGENCKSELR)*/
pub struct RCC_STGENCKSELRrs;
impl crate::RegisterSpec for RCC_STGENCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_stgenckselr::R`](R) reader structure
impl crate::Readable for RCC_STGENCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_stgenckselr::W`](W) writer structure
impl crate::Writable for RCC_STGENCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_STGENCKSELR to value 0
impl crate::Resettable for RCC_STGENCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
