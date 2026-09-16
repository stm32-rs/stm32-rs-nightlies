///Register `SAI1CKSELR` reader
pub type R = crate::R<SAI1CKSELRrs>;
///Register `SAI1CKSELR` writer
pub type W = crate::W<SAI1CKSELRrs>;
///Field `SAI1SRC` reader - SAI1SRC
pub type SAI1SRC_R = crate::FieldReader;
///Field `SAI1SRC` writer - SAI1SRC
pub type SAI1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SAI1SRC
    #[inline(always)]
    pub fn sai1src(&self) -> SAI1SRC_R {
        SAI1SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI1CKSELR")
            .field("sai1src", &self.sai1src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SAI1SRC
    #[inline(always)]
    pub fn sai1src(&mut self) -> SAI1SRC_W<'_, SAI1CKSELRrs> {
        SAI1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI1CKSELR)*/
pub struct SAI1CKSELRrs;
impl crate::RegisterSpec for SAI1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`sai1ckselr::R`](R) reader structure
impl crate::Readable for SAI1CKSELRrs {}
///`write(|w| ..)` method takes [`sai1ckselr::W`](W) writer structure
impl crate::Writable for SAI1CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SAI1CKSELR to value 0
impl crate::Resettable for SAI1CKSELRrs {}
