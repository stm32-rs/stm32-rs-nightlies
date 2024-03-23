#[doc = "Register `M1FECR` reader"]
pub type R = crate::R<M1FECRrs>;
#[doc = "Register `M1FECR` writer"]
pub type W = crate::W<M1FECRrs>;
#[doc = "Field `SEDCF` reader - ECC single error detected and corrected flag"]
pub type SEDCF_R = crate::BitReader;
#[doc = "Field `SEDCF` writer - ECC single error detected and corrected flag"]
pub type SEDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub type DEDF_R = crate::BitReader;
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub type DEDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag"]
pub type DEBWDF_R = crate::BitReader;
#[doc = "Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag"]
pub type DEBWDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    #[must_use]
    pub fn sedcf(&mut self) -> SEDCF_W<M1FECRrs> {
        SEDCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn dedf(&mut self) -> DEDF_W<M1FECRrs> {
        DEDF_W::new(self, 1)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn debwdf(&mut self) -> DEBWDF_W<M1FECRrs> {
        DEBWDF_W::new(self, 2)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FECRrs;
impl crate::RegisterSpec for M1FECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fecr::R`](R) reader structure"]
impl crate::Readable for M1FECRrs {}
#[doc = "`write(|w| ..)` method takes [`m1fecr::W`](W) writer structure"]
impl crate::Writable for M1FECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1FECR to value 0"]
impl crate::Resettable for M1FECRrs {
    const RESET_VALUE: u32 = 0;
}
