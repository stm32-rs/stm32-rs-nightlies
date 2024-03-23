#[doc = "Register `DMAMUX_C0CR` reader"]
pub type R = crate::R<DMAMUX_C0CRrs>;
#[doc = "Register `DMAMUX_C0CR` writer"]
pub type W = crate::W<DMAMUX_C0CRrs>;
#[doc = "Field `DMAREQ_ID` reader - DMAREQ_ID"]
pub type DMAREQ_ID_R = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - DMAREQ_ID"]
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SOIE` reader - SOIE"]
pub type SOIE_R = crate::BitReader;
#[doc = "Field `SOIE` writer - SOIE"]
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGE` reader - EGE"]
pub type EGE_R = crate::BitReader;
#[doc = "Field `EGE` writer - EGE"]
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - SE"]
pub type SE_R = crate::BitReader;
#[doc = "Field `SE` writer - SE"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPOL` reader - SPOL"]
pub type SPOL_R = crate::FieldReader;
#[doc = "Field `SPOL` writer - SPOL"]
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBREQ` reader - NBREQ"]
pub type NBREQ_R = crate::FieldReader;
#[doc = "Field `NBREQ` writer - NBREQ"]
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - SYNC_ID"]
pub type SYNC_ID_R = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - SYNC_ID"]
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - DMAREQ_ID"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - SOIE"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EGE"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - SE"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPOL"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - NBREQ"]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - SYNC_ID"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMAREQ_ID"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<DMAMUX_C0CRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - SOIE"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<DMAMUX_C0CRrs> {
        SOIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - EGE"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<DMAMUX_C0CRrs> {
        EGE_W::new(self, 9)
    }
    #[doc = "Bit 16 - SE"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<DMAMUX_C0CRrs> {
        SE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - SPOL"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<DMAMUX_C0CRrs> {
        SPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - NBREQ"]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<DMAMUX_C0CRrs> {
        NBREQ_W::new(self, 19)
    }
    #[doc = "Bits 24:26 - SYNC_ID"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<DMAMUX_C0CRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
#[doc = "DMAMUX request line multiplexer channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_C0CRrs;
impl crate::RegisterSpec for DMAMUX_C0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_c0cr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_C0CRrs {}
#[doc = "`write(|w| ..)` method takes [`dmamux_c0cr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_C0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_C0CR to value 0"]
impl crate::Resettable for DMAMUX_C0CRrs {
    const RESET_VALUE: u32 = 0;
}
