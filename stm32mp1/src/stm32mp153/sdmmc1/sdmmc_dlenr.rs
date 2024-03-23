#[doc = "Register `SDMMC_DLENR` reader"]
pub type R = crate::R<SDMMC_DLENRrs>;
#[doc = "Register `SDMMC_DLENR` writer"]
pub type W = crate::W<SDMMC_DLENRrs>;
#[doc = "Field `DATALENGTH` reader - DATALENGTH"]
pub type DATALENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `DATALENGTH` writer - DATALENGTH"]
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<SDMMC_DLENRrs> {
        DATALENGTH_W::new(self, 0)
    }
}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dlenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dlenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_DLENRrs;
impl crate::RegisterSpec for SDMMC_DLENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dlenr::R`](R) reader structure"]
impl crate::Readable for SDMMC_DLENRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_dlenr::W`](W) writer structure"]
impl crate::Writable for SDMMC_DLENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DLENR to value 0"]
impl crate::Resettable for SDMMC_DLENRrs {
    const RESET_VALUE: u32 = 0;
}
