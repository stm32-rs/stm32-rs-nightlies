///Register `RNG1CKSELR` reader
pub type R = crate::R<RNG1CKSELRrs>;
///Register `RNG1CKSELR` writer
pub type W = crate::W<RNG1CKSELRrs>;
///Field `RNG1SRC` reader - RNG1SRC
pub type RNG1SRC_R = crate::FieldReader;
///Field `RNG1SRC` writer - RNG1SRC
pub type RNG1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - RNG1SRC
    #[inline(always)]
    pub fn rng1src(&self) -> RNG1SRC_R {
        RNG1SRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG1CKSELR")
            .field("rng1src", &self.rng1src())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - RNG1SRC
    #[inline(always)]
    #[must_use]
    pub fn rng1src(&mut self) -> RNG1SRC_W<RNG1CKSELRrs> {
        RNG1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rng1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RNG1CKSELR)*/
pub struct RNG1CKSELRrs;
impl crate::RegisterSpec for RNG1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rng1ckselr::R`](R) reader structure
impl crate::Readable for RNG1CKSELRrs {}
///`write(|w| ..)` method takes [`rng1ckselr::W`](W) writer structure
impl crate::Writable for RNG1CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RNG1CKSELR to value 0
impl crate::Resettable for RNG1CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
