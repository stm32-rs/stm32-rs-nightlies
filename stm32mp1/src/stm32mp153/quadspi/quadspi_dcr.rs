#[doc = "Register `QUADSPI_DCR` reader"]
pub type R = crate::R<QUADSPI_DCRrs>;
#[doc = "Register `QUADSPI_DCR` writer"]
pub type W = crate::W<QUADSPI_DCRrs>;
#[doc = "Field `CKMODE` reader - CKMODE"]
pub type CKMODE_R = crate::BitReader;
#[doc = "Field `CKMODE` writer - CKMODE"]
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSHT` reader - CSHT"]
pub type CSHT_R = crate::FieldReader;
#[doc = "Field `CSHT` writer - CSHT"]
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSIZE` reader - FSIZE"]
pub type FSIZE_R = crate::FieldReader;
#[doc = "Field `FSIZE` writer - FSIZE"]
pub type FSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<QUADSPI_DCRrs> {
        CKMODE_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<QUADSPI_DCRrs> {
        CSHT_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn fsize(&mut self) -> FSIZE_W<QUADSPI_DCRrs> {
        FSIZE_W::new(self, 16)
    }
}
#[doc = "QUADSPI device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_dcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_dcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_DCRrs;
impl crate::RegisterSpec for QUADSPI_DCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_dcr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_DCRrs {}
#[doc = "`write(|w| ..)` method takes [`quadspi_dcr::W`](W) writer structure"]
impl crate::Writable for QUADSPI_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_DCR to value 0"]
impl crate::Resettable for QUADSPI_DCRrs {
    const RESET_VALUE: u32 = 0;
}
