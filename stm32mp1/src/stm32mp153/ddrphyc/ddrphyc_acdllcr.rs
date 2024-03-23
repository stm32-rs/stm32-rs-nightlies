#[doc = "Register `DDRPHYC_ACDLLCR` reader"]
pub type R = crate::R<DDRPHYC_ACDLLCRrs>;
#[doc = "Register `DDRPHYC_ACDLLCR` writer"]
pub type W = crate::W<DDRPHYC_ACDLLCRrs>;
#[doc = "Field `MFBDLY` reader - MFBDLY"]
pub type MFBDLY_R = crate::FieldReader;
#[doc = "Field `MFBDLY` writer - MFBDLY"]
pub type MFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MFWDLY` reader - MFWDLY"]
pub type MFWDLY_R = crate::FieldReader;
#[doc = "Field `MFWDLY` writer - MFWDLY"]
pub type MFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATESTEN` reader - ATESTEN"]
pub type ATESTEN_R = crate::BitReader;
#[doc = "Field `ATESTEN` writer - ATESTEN"]
pub type ATESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLSRST` reader - DLLSRST"]
pub type DLLSRST_R = crate::BitReader;
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLDIS` reader - DLLDIS"]
pub type DLLDIS_R = crate::BitReader;
#[doc = "Field `DLLDIS` writer - DLLDIS"]
pub type DLLDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfbdly(&mut self) -> MFBDLY_W<DDRPHYC_ACDLLCRrs> {
        MFBDLY_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfwdly(&mut self) -> MFWDLY_W<DDRPHYC_ACDLLCRrs> {
        MFWDLY_W::new(self, 9)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    #[must_use]
    pub fn atesten(&mut self) -> ATESTEN_W<DDRPHYC_ACDLLCRrs> {
        ATESTEN_W::new(self, 18)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<DDRPHYC_ACDLLCRrs> {
        DLLSRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    #[must_use]
    pub fn dlldis(&mut self) -> DLLDIS_W<DDRPHYC_ACDLLCRrs> {
        DLLDIS_W::new(self, 31)
    }
}
#[doc = "DDRPHYC AC DLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_acdllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_acdllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ACDLLCRrs;
impl crate::RegisterSpec for DDRPHYC_ACDLLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_acdllcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ACDLLCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_acdllcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_ACDLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ACDLLCR to value 0x4000_0000"]
impl crate::Resettable for DDRPHYC_ACDLLCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
