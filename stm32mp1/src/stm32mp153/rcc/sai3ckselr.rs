///Register `SAI3CKSELR` reader
pub type R = crate::R<SAI3CKSELRrs>;
///Register `SAI3CKSELR` writer
pub type W = crate::W<SAI3CKSELRrs>;
///Field `SAI3SRC` reader - SAI3SRC
pub type SAI3SRC_R = crate::FieldReader;
///Field `SAI3SRC` writer - SAI3SRC
pub type SAI3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SAI3SRC
    #[inline(always)]
    pub fn sai3src(&self) -> SAI3SRC_R {
        SAI3SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI3CKSELR")
            .field("sai3src", &self.sai3src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SAI3SRC
    #[inline(always)]
    pub fn sai3src(&mut self) -> SAI3SRC_W<'_, SAI3CKSELRrs> {
        SAI3SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai3ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai3ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI3CKSELR)*/
pub struct SAI3CKSELRrs;
impl crate::RegisterSpec for SAI3CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`sai3ckselr::R`](R) reader structure
impl crate::Readable for SAI3CKSELRrs {}
///`write(|w| ..)` method takes [`sai3ckselr::W`](W) writer structure
impl crate::Writable for SAI3CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAI3CKSELR to value 0
impl crate::Resettable for SAI3CKSELRrs {}
