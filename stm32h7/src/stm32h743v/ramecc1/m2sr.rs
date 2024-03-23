#[doc = "Register `M2SR` reader"]
pub type R = crate::R<M2SRrs>;
#[doc = "Register `M2SR` writer"]
pub type W = crate::W<M2SRrs>;
#[doc = "Field `SEDCF` reader - ECC single error detected flag"]
pub type SEDCF_R = crate::BitReader;
#[doc = "Field `SEDCF` writer - ECC single error detected flag"]
pub type SEDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub type DEDF_R = crate::BitReader;
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub type DEDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBWDF` reader - ECC double error on byte write flag"]
pub type DEBWDF_R = crate::BitReader;
#[doc = "Field `DEBWDF` writer - ECC double error on byte write flag"]
pub type DEBWDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC single error detected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn sedcf(&mut self) -> SEDCF_W<M2SRrs> {
        SEDCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn dedf(&mut self) -> DEDF_W<M2SRrs> {
        DEDF_W::new(self, 1)
    }
    #[doc = "Bit 2 - ECC double error on byte write flag"]
    #[inline(always)]
    #[must_use]
    pub fn debwdf(&mut self) -> DEBWDF_W<M2SRrs> {
        DEBWDF_W::new(self, 2)
    }
}
#[doc = "RAMECC monitor 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2SRrs;
impl crate::RegisterSpec for M2SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2sr::R`](R) reader structure"]
impl crate::Readable for M2SRrs {}
#[doc = "`write(|w| ..)` method takes [`m2sr::W`](W) writer structure"]
impl crate::Writable for M2SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M2SR to value 0"]
impl crate::Resettable for M2SRrs {
    const RESET_VALUE: u32 = 0;
}
