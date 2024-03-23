#[doc = "Register `DDRPHYC_DLLGCR` reader"]
pub type R = crate::R<DDRPHYC_DLLGCRrs>;
#[doc = "Register `DDRPHYC_DLLGCR` writer"]
pub type W = crate::W<DDRPHYC_DLLGCRrs>;
#[doc = "Field `DRES` reader - DRES"]
pub type DRES_R = crate::FieldReader;
#[doc = "Field `DRES` writer - DRES"]
pub type DRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IPUMP` reader - IPUMP"]
pub type IPUMP_R = crate::FieldReader;
#[doc = "Field `IPUMP` writer - IPUMP"]
pub type IPUMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESTEN` reader - TESTEN"]
pub type TESTEN_R = crate::BitReader;
#[doc = "Field `TESTEN` writer - TESTEN"]
pub type TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC` reader - DTC"]
pub type DTC_R = crate::FieldReader;
#[doc = "Field `DTC` writer - DTC"]
pub type DTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATC` reader - ATC"]
pub type ATC_R = crate::FieldReader;
#[doc = "Field `ATC` writer - ATC"]
pub type ATC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TESTSW` reader - TESTSW"]
pub type TESTSW_R = crate::BitReader;
#[doc = "Field `TESTSW` writer - TESTSW"]
pub type TESTSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIAS` reader - MBIAS"]
pub type MBIAS_R = crate::FieldReader;
#[doc = "Field `MBIAS` writer - MBIAS"]
pub type MBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SBIAS2_0` reader - SBIAS2_0"]
pub type SBIAS2_0_R = crate::FieldReader;
#[doc = "Field `SBIAS2_0` writer - SBIAS2_0"]
pub type SBIAS2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BPS200` reader - BPS200"]
pub type BPS200_R = crate::BitReader;
#[doc = "Field `BPS200` writer - BPS200"]
pub type BPS200_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIAS5_3` reader - SBIAS5_3"]
pub type SBIAS5_3_R = crate::FieldReader;
#[doc = "Field `SBIAS5_3` writer - SBIAS5_3"]
pub type SBIAS5_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FDTRMSL` reader - FDTRMSL"]
pub type FDTRMSL_R = crate::FieldReader;
#[doc = "Field `FDTRMSL` writer - FDTRMSL"]
pub type FDTRMSL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOCKDET` reader - LOCKDET"]
pub type LOCKDET_R = crate::BitReader;
#[doc = "Field `LOCKDET` writer - LOCKDET"]
pub type LOCKDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRSVD2` reader - DLLRSVD2"]
pub type DLLRSVD2_R = crate::FieldReader;
#[doc = "Field `DLLRSVD2` writer - DLLRSVD2"]
pub type DLLRSVD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&self) -> IPUMP_R {
        IPUMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&self) -> ATC_R {
        ATC_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&self) -> TESTSW_R {
        TESTSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&self) -> MBIAS_R {
        MBIAS_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&self) -> SBIAS2_0_R {
        SBIAS2_0_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&self) -> BPS200_R {
        BPS200_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&self) -> SBIAS5_3_R {
        SBIAS5_3_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&self) -> FDTRMSL_R {
        FDTRMSL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&self) -> LOCKDET_R {
        LOCKDET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&self) -> DLLRSVD2_R {
        DLLRSVD2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<DDRPHYC_DLLGCRrs> {
        DRES_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    #[must_use]
    pub fn ipump(&mut self) -> IPUMP_W<DDRPHYC_DLLGCRrs> {
        IPUMP_W::new(self, 2)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TESTEN_W<DDRPHYC_DLLGCRrs> {
        TESTEN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DTC_W<DDRPHYC_DLLGCRrs> {
        DTC_W::new(self, 6)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    #[must_use]
    pub fn atc(&mut self) -> ATC_W<DDRPHYC_DLLGCRrs> {
        ATC_W::new(self, 9)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    #[must_use]
    pub fn testsw(&mut self) -> TESTSW_W<DDRPHYC_DLLGCRrs> {
        TESTSW_W::new(self, 11)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    #[must_use]
    pub fn mbias(&mut self) -> MBIAS_W<DDRPHYC_DLLGCRrs> {
        MBIAS_W::new(self, 12)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    #[must_use]
    pub fn sbias2_0(&mut self) -> SBIAS2_0_W<DDRPHYC_DLLGCRrs> {
        SBIAS2_0_W::new(self, 20)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    #[must_use]
    pub fn bps200(&mut self) -> BPS200_W<DDRPHYC_DLLGCRrs> {
        BPS200_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    #[must_use]
    pub fn sbias5_3(&mut self) -> SBIAS5_3_W<DDRPHYC_DLLGCRrs> {
        SBIAS5_3_W::new(self, 24)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    #[must_use]
    pub fn fdtrmsl(&mut self) -> FDTRMSL_W<DDRPHYC_DLLGCRrs> {
        FDTRMSL_W::new(self, 27)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    #[must_use]
    pub fn lockdet(&mut self) -> LOCKDET_W<DDRPHYC_DLLGCRrs> {
        LOCKDET_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    #[must_use]
    pub fn dllrsvd2(&mut self) -> DLLRSVD2_W<DDRPHYC_DLLGCRrs> {
        DLLRSVD2_W::new(self, 30)
    }
}
#[doc = "DDRPHYC DDR global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dllgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dllgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DLLGCRrs;
impl crate::RegisterSpec for DDRPHYC_DLLGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dllgcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DLLGCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dllgcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DLLGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DLLGCR to value 0x0373_7000"]
impl crate::Resettable for DDRPHYC_DLLGCRrs {
    const RESET_VALUE: u32 = 0x0373_7000;
}
