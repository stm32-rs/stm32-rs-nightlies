#[doc = "Register `DDRPHYC_DX2DLLCR` reader"]
pub type R = crate::R<DDRPHYC_DX2DLLCRrs>;
#[doc = "Register `DDRPHYC_DX2DLLCR` writer"]
pub type W = crate::W<DDRPHYC_DX2DLLCRrs>;
#[doc = "Field `SFBDLY` reader - SFBDLY"]
pub type SFBDLY_R = crate::FieldReader;
#[doc = "Field `SFBDLY` writer - SFBDLY"]
pub type SFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SFWDLY` reader - SFWDLY"]
pub type SFWDLY_R = crate::FieldReader;
#[doc = "Field `SFWDLY` writer - SFWDLY"]
pub type SFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MFBDLY` reader - MFBDLY"]
pub type MFBDLY_R = crate::FieldReader;
#[doc = "Field `MFBDLY` writer - MFBDLY"]
pub type MFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MFWDLY` reader - MFWDLY"]
pub type MFWDLY_R = crate::FieldReader;
#[doc = "Field `MFWDLY` writer - MFWDLY"]
pub type MFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSTART` reader - SSTART"]
pub type SSTART_R = crate::FieldReader;
#[doc = "Field `SSTART` writer - SSTART"]
pub type SSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDPHASE` reader - SDPHASE"]
pub type SDPHASE_R = crate::FieldReader;
#[doc = "Field `SDPHASE` writer - SDPHASE"]
pub type SDPHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ATESTEN` reader - ATESTEN"]
pub type ATESTEN_R = crate::BitReader;
#[doc = "Field `ATESTEN` writer - ATESTEN"]
pub type ATESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDLBMODE` reader - SDLBMODE"]
pub type SDLBMODE_R = crate::BitReader;
#[doc = "Field `SDLBMODE` writer - SDLBMODE"]
pub type SDLBMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLSRST` reader - DLLSRST"]
pub type DLLSRST_R = crate::BitReader;
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLDIS` reader - DLLDIS"]
pub type DLLDIS_R = crate::BitReader;
#[doc = "Field `DLLDIS` writer - DLLDIS"]
pub type DLLDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&self) -> SFBDLY_R {
        SFBDLY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&self) -> SFWDLY_R {
        SFWDLY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&self) -> SDPHASE_R {
        SDPHASE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&self) -> SDLBMODE_R {
        SDLBMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    #[must_use]
    pub fn sfbdly(&mut self) -> SFBDLY_W<DDRPHYC_DX2DLLCRrs> {
        SFBDLY_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    #[must_use]
    pub fn sfwdly(&mut self) -> SFWDLY_W<DDRPHYC_DX2DLLCRrs> {
        SFWDLY_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfbdly(&mut self) -> MFBDLY_W<DDRPHYC_DX2DLLCRrs> {
        MFBDLY_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfwdly(&mut self) -> MFWDLY_W<DDRPHYC_DX2DLLCRrs> {
        MFWDLY_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    #[must_use]
    pub fn sstart(&mut self) -> SSTART_W<DDRPHYC_DX2DLLCRrs> {
        SSTART_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    #[must_use]
    pub fn sdphase(&mut self) -> SDPHASE_W<DDRPHYC_DX2DLLCRrs> {
        SDPHASE_W::new(self, 14)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    #[must_use]
    pub fn atesten(&mut self) -> ATESTEN_W<DDRPHYC_DX2DLLCRrs> {
        ATESTEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn sdlbmode(&mut self) -> SDLBMODE_W<DDRPHYC_DX2DLLCRrs> {
        SDLBMODE_W::new(self, 19)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<DDRPHYC_DX2DLLCRrs> {
        DLLSRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    #[must_use]
    pub fn dlldis(&mut self) -> DLLDIS_W<DDRPHYC_DX2DLLCRrs> {
        DLLDIS_W::new(self, 31)
    }
}
#[doc = "DDRPHYC byte lane 2 DLLC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx2dllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dx2dllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX2DLLCRrs;
impl crate::RegisterSpec for DDRPHYC_DX2DLLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dx2dllcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DLLCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dx2dllcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DX2DLLCR to value 0x4000_0000"]
impl crate::Resettable for DDRPHYC_DX2DLLCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
