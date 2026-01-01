///Register `SDMMC3CKSELR` reader
pub type R = crate::R<SDMMC3CKSELRrs>;
///Register `SDMMC3CKSELR` writer
pub type W = crate::W<SDMMC3CKSELRrs>;
///Field `SDMMC3SRC` reader - SDMMC3SRC
pub type SDMMC3SRC_R = crate::FieldReader;
///Field `SDMMC3SRC` writer - SDMMC3SRC
pub type SDMMC3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SDMMC3SRC
    #[inline(always)]
    pub fn sdmmc3src(&self) -> SDMMC3SRC_R {
        SDMMC3SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC3CKSELR")
            .field("sdmmc3src", &self.sdmmc3src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SDMMC3SRC
    #[inline(always)]
    pub fn sdmmc3src(&mut self) -> SDMMC3SRC_W<'_, SDMMC3CKSELRrs> {
        SDMMC3SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sdmmc3ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc3ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:SDMMC3CKSELR)*/
pub struct SDMMC3CKSELRrs;
impl crate::RegisterSpec for SDMMC3CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc3ckselr::R`](R) reader structure
impl crate::Readable for SDMMC3CKSELRrs {}
///`write(|w| ..)` method takes [`sdmmc3ckselr::W`](W) writer structure
impl crate::Writable for SDMMC3CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDMMC3CKSELR to value 0
impl crate::Resettable for SDMMC3CKSELRrs {}
