///Register `LPTIM45CKSELR` reader
pub type R = crate::R<LPTIM45CKSELRrs>;
///Register `LPTIM45CKSELR` writer
pub type W = crate::W<LPTIM45CKSELRrs>;
///Field `LPTIM45SRC` reader - LPTIM45SRC
pub type LPTIM45SRC_R = crate::FieldReader;
///Field `LPTIM45SRC` writer - LPTIM45SRC
pub type LPTIM45SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - LPTIM45SRC
    #[inline(always)]
    pub fn lptim45src(&self) -> LPTIM45SRC_R {
        LPTIM45SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM45CKSELR")
            .field("lptim45src", &self.lptim45src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPTIM45SRC
    #[inline(always)]
    pub fn lptim45src(&mut self) -> LPTIM45SRC_W<'_, LPTIM45CKSELRrs> {
        LPTIM45SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.

You can [`read`](crate::Reg::read) this register and get [`lptim45ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim45ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:LPTIM45CKSELR)*/
pub struct LPTIM45CKSELRrs;
impl crate::RegisterSpec for LPTIM45CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim45ckselr::R`](R) reader structure
impl crate::Readable for LPTIM45CKSELRrs {}
///`write(|w| ..)` method takes [`lptim45ckselr::W`](W) writer structure
impl crate::Writable for LPTIM45CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPTIM45CKSELR to value 0
impl crate::Resettable for LPTIM45CKSELRrs {}
