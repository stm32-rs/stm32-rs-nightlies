///Register `SAI4CKSELR` reader
pub type R = crate::R<SAI4CKSELRrs>;
///Register `SAI4CKSELR` writer
pub type W = crate::W<SAI4CKSELRrs>;
///Field `SAI4SRC` reader - SAI4SRC
pub type SAI4SRC_R = crate::FieldReader;
///Field `SAI4SRC` writer - SAI4SRC
pub type SAI4SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SAI4SRC
    #[inline(always)]
    pub fn sai4src(&self) -> SAI4SRC_R {
        SAI4SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI4CKSELR")
            .field("sai4src", &self.sai4src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SAI4SRC
    #[inline(always)]
    pub fn sai4src(&mut self) -> SAI4SRC_W<'_, SAI4CKSELRrs> {
        SAI4SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai4ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai4ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:SAI4CKSELR)*/
pub struct SAI4CKSELRrs;
impl crate::RegisterSpec for SAI4CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`sai4ckselr::R`](R) reader structure
impl crate::Readable for SAI4CKSELRrs {}
///`write(|w| ..)` method takes [`sai4ckselr::W`](W) writer structure
impl crate::Writable for SAI4CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAI4CKSELR to value 0
impl crate::Resettable for SAI4CKSELRrs {}
