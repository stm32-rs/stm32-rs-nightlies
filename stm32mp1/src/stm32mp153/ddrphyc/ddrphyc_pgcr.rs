#[doc = "Register `DDRPHYC_PGCR` reader"]
pub type R = crate::R<DDRPHYC_PGCRrs>;
#[doc = "Register `DDRPHYC_PGCR` writer"]
pub type W = crate::W<DDRPHYC_PGCRrs>;
#[doc = "Field `ITMDMD` reader - ITMDMD"]
pub type ITMDMD_R = crate::BitReader;
#[doc = "Field `ITMDMD` writer - ITMDMD"]
pub type ITMDMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSCFG` reader - DQSCFG"]
pub type DQSCFG_R = crate::BitReader;
#[doc = "Field `DQSCFG` writer - DQSCFG"]
pub type DQSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFTCMP` reader - DFTCMP"]
pub type DFTCMP_R = crate::BitReader;
#[doc = "Field `DFTCMP` writer - DFTCMP"]
pub type DFTCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFTLMT` reader - DFTLMT"]
pub type DFTLMT_R = crate::FieldReader;
#[doc = "Field `DFTLMT` writer - DFTLMT"]
pub type DFTLMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOSEL` reader - DTOSEL"]
pub type DTOSEL_R = crate::FieldReader;
#[doc = "Field `DTOSEL` writer - DTOSEL"]
pub type DTOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CKEN` reader - CKEN"]
pub type CKEN_R = crate::FieldReader;
#[doc = "Field `CKEN` writer - CKEN"]
pub type CKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKDV` reader - CKDV"]
pub type CKDV_R = crate::FieldReader;
#[doc = "Field `CKDV` writer - CKDV"]
pub type CKDV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKINV` reader - CKINV"]
pub type CKINV_R = crate::BitReader;
#[doc = "Field `CKINV` writer - CKINV"]
pub type CKINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLB` reader - IOLB"]
pub type IOLB_R = crate::BitReader;
#[doc = "Field `IOLB` writer - IOLB"]
pub type IOLB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IODDRM` reader - IODDRM"]
pub type IODDRM_R = crate::FieldReader;
#[doc = "Field `IODDRM` writer - IODDRM"]
pub type IODDRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RANKEN` reader - RANKEN"]
pub type RANKEN_R = crate::FieldReader;
#[doc = "Field `RANKEN` writer - RANKEN"]
pub type RANKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ZKSEL` reader - ZKSEL"]
pub type ZKSEL_R = crate::FieldReader;
#[doc = "Field `ZKSEL` writer - ZKSEL"]
pub type ZKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDDISDX` reader - PDDISDX"]
pub type PDDISDX_R = crate::BitReader;
#[doc = "Field `PDDISDX` writer - PDDISDX"]
pub type PDDISDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFSHDT` reader - RFSHDT"]
pub type RFSHDT_R = crate::FieldReader;
#[doc = "Field `RFSHDT` writer - RFSHDT"]
pub type RFSHDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBDQSS` reader - LBDQSS"]
pub type LBDQSS_R = crate::BitReader;
#[doc = "Field `LBDQSS` writer - LBDQSS"]
pub type LBDQSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBGDQS` reader - LBGDQS"]
pub type LBGDQS_R = crate::BitReader;
#[doc = "Field `LBGDQS` writer - LBGDQS"]
pub type LBGDQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBMODE` reader - LBMODE"]
pub type LBMODE_R = crate::BitReader;
#[doc = "Field `LBMODE` writer - LBMODE"]
pub type LBMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&self) -> ITMDMD_R {
        ITMDMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&self) -> DQSCFG_R {
        DQSCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&self) -> DFTCMP_R {
        DFTCMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&self) -> DFTLMT_R {
        DFTLMT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&self) -> DTOSEL_R {
        DTOSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&self) -> CKINV_R {
        CKINV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&self) -> IOLB_R {
        IOLB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&self) -> IODDRM_R {
        IODDRM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&self) -> RANKEN_R {
        RANKEN_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&self) -> ZKSEL_R {
        ZKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&self) -> PDDISDX_R {
        PDDISDX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&self) -> RFSHDT_R {
        RFSHDT_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&self) -> LBDQSS_R {
        LBDQSS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&self) -> LBGDQS_R {
        LBGDQS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&self) -> LBMODE_R {
        LBMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    #[must_use]
    pub fn itmdmd(&mut self) -> ITMDMD_W<DDRPHYC_PGCRrs> {
        ITMDMD_W::new(self, 0)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn dqscfg(&mut self) -> DQSCFG_W<DDRPHYC_PGCRrs> {
        DQSCFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    #[must_use]
    pub fn dftcmp(&mut self) -> DFTCMP_W<DDRPHYC_PGCRrs> {
        DFTCMP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    #[must_use]
    pub fn dftlmt(&mut self) -> DFTLMT_W<DDRPHYC_PGCRrs> {
        DFTLMT_W::new(self, 3)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    #[must_use]
    pub fn dtosel(&mut self) -> DTOSEL_W<DDRPHYC_PGCRrs> {
        DTOSEL_W::new(self, 5)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<DDRPHYC_PGCRrs> {
        CKEN_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    #[must_use]
    pub fn ckdv(&mut self) -> CKDV_W<DDRPHYC_PGCRrs> {
        CKDV_W::new(self, 12)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    #[must_use]
    pub fn ckinv(&mut self) -> CKINV_W<DDRPHYC_PGCRrs> {
        CKINV_W::new(self, 14)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    #[must_use]
    pub fn iolb(&mut self) -> IOLB_W<DDRPHYC_PGCRrs> {
        IOLB_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    #[must_use]
    pub fn ioddrm(&mut self) -> IODDRM_W<DDRPHYC_PGCRrs> {
        IODDRM_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    #[must_use]
    pub fn ranken(&mut self) -> RANKEN_W<DDRPHYC_PGCRrs> {
        RANKEN_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn zksel(&mut self) -> ZKSEL_W<DDRPHYC_PGCRrs> {
        ZKSEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    #[must_use]
    pub fn pddisdx(&mut self) -> PDDISDX_W<DDRPHYC_PGCRrs> {
        PDDISDX_W::new(self, 24)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    #[must_use]
    pub fn rfshdt(&mut self) -> RFSHDT_W<DDRPHYC_PGCRrs> {
        RFSHDT_W::new(self, 25)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    #[must_use]
    pub fn lbdqss(&mut self) -> LBDQSS_W<DDRPHYC_PGCRrs> {
        LBDQSS_W::new(self, 29)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    #[must_use]
    pub fn lbgdqs(&mut self) -> LBGDQS_W<DDRPHYC_PGCRrs> {
        LBGDQS_W::new(self, 30)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn lbmode(&mut self) -> LBMODE_W<DDRPHYC_PGCRrs> {
        LBMODE_W::new(self, 31)
    }
}
#[doc = "DDRPHYC PHY global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_pgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_pgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PGCRrs;
impl crate::RegisterSpec for DDRPHYC_PGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_pgcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_PGCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_pgcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_PGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PGCR to value 0x01bc_2e04"]
impl crate::Resettable for DDRPHYC_PGCRrs {
    const RESET_VALUE: u32 = 0x01bc_2e04;
}
