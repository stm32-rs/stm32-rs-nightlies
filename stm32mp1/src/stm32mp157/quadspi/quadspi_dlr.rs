#[doc = "Register `QUADSPI_DLR` reader"]
pub type R = crate::R<QUADSPI_DLRrs>;
#[doc = "Register `QUADSPI_DLR` writer"]
pub type W = crate::W<QUADSPI_DLRrs>;
#[doc = "Field `DL` reader - DL"]
pub type DL_R = crate::FieldReader<u32>;
#[doc = "Field `DL` writer - DL"]
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DL"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DL"]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<QUADSPI_DLRrs> {
        DL_W::new(self, 0)
    }
}
#[doc = "QUADSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_dlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_dlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_DLRrs;
impl crate::RegisterSpec for QUADSPI_DLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_dlr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_DLRrs {}
#[doc = "`write(|w| ..)` method takes [`quadspi_dlr::W`](W) writer structure"]
impl crate::Writable for QUADSPI_DLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_DLR to value 0"]
impl crate::Resettable for QUADSPI_DLRrs {
    const RESET_VALUE: u32 = 0;
}
