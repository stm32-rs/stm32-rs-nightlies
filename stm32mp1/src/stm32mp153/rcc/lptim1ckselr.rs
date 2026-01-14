///Register `LPTIM1CKSELR` reader
pub type R = crate::R<LPTIM1CKSELRrs>;
///Register `LPTIM1CKSELR` writer
pub type W = crate::W<LPTIM1CKSELRrs>;
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
        f.debug_struct("LPTIM1CKSELR")
            .field("lptim1src", &self.lptim1src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPTIM1SRC
    #[inline(always)]
    pub fn lptim1src(&mut self) -> LPTIM1SRC_W<'_, LPTIM1CKSELRrs> {
        LPTIM1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the LPTIM1 block.

You can [`read`](crate::Reg::read) this register and get [`lptim1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:LPTIM1CKSELR)*/
pub struct LPTIM1CKSELRrs;
impl crate::RegisterSpec for LPTIM1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim1ckselr::R`](R) reader structure
impl crate::Readable for LPTIM1CKSELRrs {}
///`write(|w| ..)` method takes [`lptim1ckselr::W`](W) writer structure
impl crate::Writable for LPTIM1CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPTIM1CKSELR to value 0
impl crate::Resettable for LPTIM1CKSELRrs {}
