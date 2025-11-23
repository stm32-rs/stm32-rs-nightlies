///Register `LPTIM23CKSELR` reader
pub type R = crate::R<LPTIM23CKSELRrs>;
///Register `LPTIM23CKSELR` writer
pub type W = crate::W<LPTIM23CKSELRrs>;
///Field `LPTIM23SRC` reader - LPTIM23SRC
pub type LPTIM23SRC_R = crate::FieldReader;
///Field `LPTIM23SRC` writer - LPTIM23SRC
pub type LPTIM23SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - LPTIM23SRC
    #[inline(always)]
    pub fn lptim23src(&self) -> LPTIM23SRC_R {
        LPTIM23SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM23CKSELR")
            .field("lptim23src", &self.lptim23src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - LPTIM23SRC
    #[inline(always)]
    pub fn lptim23src(&mut self) -> LPTIM23SRC_W<'_, LPTIM23CKSELRrs> {
        LPTIM23SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.

You can [`read`](crate::Reg::read) this register and get [`lptim23ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim23ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:LPTIM23CKSELR)*/
pub struct LPTIM23CKSELRrs;
impl crate::RegisterSpec for LPTIM23CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim23ckselr::R`](R) reader structure
impl crate::Readable for LPTIM23CKSELRrs {}
///`write(|w| ..)` method takes [`lptim23ckselr::W`](W) writer structure
impl crate::Writable for LPTIM23CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPTIM23CKSELR to value 0
impl crate::Resettable for LPTIM23CKSELRrs {}
