///Register `SAI2CKSELR` reader
pub type R = crate::R<SAI2CKSELRrs>;
///Register `SAI2CKSELR` writer
pub type W = crate::W<SAI2CKSELRrs>;
///Field `SAI2SRC` reader - SAI2SRC
pub type SAI2SRC_R = crate::FieldReader;
///Field `SAI2SRC` writer - SAI2SRC
pub type SAI2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SAI2SRC
    #[inline(always)]
    pub fn sai2src(&self) -> SAI2SRC_R {
        SAI2SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI2CKSELR")
            .field("sai2src", &self.sai2src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SAI2SRC
    #[inline(always)]
    pub fn sai2src(&mut self) -> SAI2SRC_W<'_, SAI2CKSELRrs> {
        SAI2SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai2ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai2ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:SAI2CKSELR)*/
pub struct SAI2CKSELRrs;
impl crate::RegisterSpec for SAI2CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`sai2ckselr::R`](R) reader structure
impl crate::Readable for SAI2CKSELRrs {}
///`write(|w| ..)` method takes [`sai2ckselr::W`](W) writer structure
impl crate::Writable for SAI2CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAI2CKSELR to value 0
impl crate::Resettable for SAI2CKSELRrs {}
