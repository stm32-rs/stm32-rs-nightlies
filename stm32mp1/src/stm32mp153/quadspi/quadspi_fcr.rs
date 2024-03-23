#[doc = "Register `QUADSPI_FCR` writer"]
pub type W = crate::W<QUADSPI_FCRrs>;
#[doc = "Field `CTEF` writer - CTEF"]
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` writer - CTCF"]
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMF` writer - CSMF"]
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTOF` writer - CTOF"]
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CTEF"]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<QUADSPI_FCRrs> {
        CTEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CTCF"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<QUADSPI_FCRrs> {
        CTCF_W::new(self, 1)
    }
    #[doc = "Bit 3 - CSMF"]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<QUADSPI_FCRrs> {
        CSMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CTOF"]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<QUADSPI_FCRrs> {
        CTOF_W::new(self, 4)
    }
}
#[doc = "QUADSPI flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_FCRrs;
impl crate::RegisterSpec for QUADSPI_FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`quadspi_fcr::W`](W) writer structure"]
impl crate::Writable for QUADSPI_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_FCR to value 0"]
impl crate::Resettable for QUADSPI_FCRrs {
    const RESET_VALUE: u32 = 0;
}
