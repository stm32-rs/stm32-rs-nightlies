///Register `RCC_LPTIM1CKSELR` reader
pub type R = crate::R<RCC_LPTIM1CKSELRrs>;
///Register `RCC_LPTIM1CKSELR` writer
pub type W = crate::W<RCC_LPTIM1CKSELRrs>;
///Field `LPTIM1SRC` reader - LPTIM1SRC
pub type LPTIM1SRC_R = crate::FieldReader;
///Field `LPTIM1SRC` writer - LPTIM1SRC
pub type LPTIM1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - LPTIM1SRC
    #[inline(always)]
    pub fn lptim1src(&self) -> LPTIM1SRC_R {
        LPTIM1SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM1CKSELR")
            .field("lptim1src", &self.lptim1src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPTIM1SRC
    #[inline(always)]
    #[must_use]
    pub fn lptim1src(&mut self) -> LPTIM1SRC_W<RCC_LPTIM1CKSELRrs> {
        LPTIM1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the LPTIM1 block.

You can [`read`](crate::Reg::read) this register and get [`rcc_lptim1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_lptim1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_LPTIM1CKSELR)*/
pub struct RCC_LPTIM1CKSELRrs;
impl crate::RegisterSpec for RCC_LPTIM1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_lptim1ckselr::R`](R) reader structure
impl crate::Readable for RCC_LPTIM1CKSELRrs {}
///`write(|w| ..)` method takes [`rcc_lptim1ckselr::W`](W) writer structure
impl crate::Writable for RCC_LPTIM1CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_LPTIM1CKSELR to value 0
impl crate::Resettable for RCC_LPTIM1CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
