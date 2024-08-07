///Register `RCC_RNG2CKSELR` reader
pub type R = crate::R<RCC_RNG2CKSELRrs>;
///Register `RCC_RNG2CKSELR` writer
pub type W = crate::W<RCC_RNG2CKSELRrs>;
///Field `RNG2SRC` reader - RNG2SRC
pub type RNG2SRC_R = crate::FieldReader;
///Field `RNG2SRC` writer - RNG2SRC
pub type RNG2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - RNG2SRC
    #[inline(always)]
    pub fn rng2src(&self) -> RNG2SRC_R {
        RNG2SRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RNG2CKSELR")
            .field("rng2src", &self.rng2src())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - RNG2SRC
    #[inline(always)]
    #[must_use]
    pub fn rng2src(&mut self) -> RNG2SRC_W<RCC_RNG2CKSELRrs> {
        RNG2SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the RNG2.

You can [`read`](crate::Reg::read) this register and get [`rcc_rng2ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_rng2ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_RNG2CKSELR)*/
pub struct RCC_RNG2CKSELRrs;
impl crate::RegisterSpec for RCC_RNG2CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_rng2ckselr::R`](R) reader structure
impl crate::Readable for RCC_RNG2CKSELRrs {}
///`write(|w| ..)` method takes [`rcc_rng2ckselr::W`](W) writer structure
impl crate::Writable for RCC_RNG2CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_RNG2CKSELR to value 0
impl crate::Resettable for RCC_RNG2CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
