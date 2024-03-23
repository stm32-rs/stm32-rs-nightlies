#[doc = "Register `CKGCR` reader"]
pub type R = crate::R<CKGCRrs>;
#[doc = "Register `CKGCR` writer"]
pub type W = crate::W<CKGCRrs>;
#[doc = "Field `CKGDEN` reader - CKGDEN"]
pub type CKGDEN_R = crate::BitReader;
#[doc = "Field `CKGDEN` writer - CKGDEN"]
pub type CKGDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK0EN` reader - CCK0EN"]
pub type CCK0EN_R = crate::BitReader;
#[doc = "Field `CCK0EN` writer - CCK0EN"]
pub type CCK0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK1EN` reader - CCK1EN"]
pub type CCK1EN_R = crate::BitReader;
#[doc = "Field `CCK1EN` writer - CCK1EN"]
pub type CCK1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKGMOD` reader - CKGMOD"]
pub type CKGMOD_R = crate::BitReader;
#[doc = "Field `CKGMOD` writer - CKGMOD"]
pub type CKGMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK0DIR` reader - CCK0DIR"]
pub type CCK0DIR_R = crate::BitReader;
#[doc = "Field `CCK0DIR` writer - CCK0DIR"]
pub type CCK0DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCK1DIR` reader - CCK1DIR"]
pub type CCK1DIR_R = crate::BitReader;
#[doc = "Field `CCK1DIR` writer - CCK1DIR"]
pub type CCK1DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSENS` reader - TRGSENS"]
pub type TRGSENS_R = crate::BitReader;
#[doc = "Field `TRGSENS` writer - TRGSENS"]
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSRC` reader - TRGSRC"]
pub type TRGSRC_R = crate::FieldReader;
#[doc = "Field `TRGSRC` writer - TRGSRC"]
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCKDIV` reader - CCKDIV"]
pub type CCKDIV_R = crate::FieldReader;
#[doc = "Field `CCKDIV` writer - CCKDIV"]
pub type CCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PROCDIV` reader - PROCDIV"]
pub type PROCDIV_R = crate::FieldReader;
#[doc = "Field `PROCDIV` writer - PROCDIV"]
pub type PROCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CKGACTIVE` reader - CKGACTIVE"]
pub type CKGACTIVE_R = crate::BitReader;
#[doc = "Field `CKGACTIVE` writer - CKGACTIVE"]
pub type CKGACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CKGDEN"]
    #[inline(always)]
    pub fn ckgden(&self) -> CKGDEN_R {
        CKGDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCK0EN"]
    #[inline(always)]
    pub fn cck0en(&self) -> CCK0EN_R {
        CCK0EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCK1EN"]
    #[inline(always)]
    pub fn cck1en(&self) -> CCK1EN_R {
        CCK1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CKGMOD"]
    #[inline(always)]
    pub fn ckgmod(&self) -> CKGMOD_R {
        CKGMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCK0DIR"]
    #[inline(always)]
    pub fn cck0dir(&self) -> CCK0DIR_R {
        CCK0DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCK1DIR"]
    #[inline(always)]
    pub fn cck1dir(&self) -> CCK1DIR_R {
        CCK1DIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - TRGSENS"]
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - TRGSRC"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CCKDIV"]
    #[inline(always)]
    pub fn cckdiv(&self) -> CCKDIV_R {
        CCKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - PROCDIV"]
    #[inline(always)]
    pub fn procdiv(&self) -> PROCDIV_R {
        PROCDIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - CKGACTIVE"]
    #[inline(always)]
    pub fn ckgactive(&self) -> CKGACTIVE_R {
        CKGACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKGDEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckgden(&mut self) -> CKGDEN_W<CKGCRrs> {
        CKGDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCK0EN"]
    #[inline(always)]
    #[must_use]
    pub fn cck0en(&mut self) -> CCK0EN_W<CKGCRrs> {
        CCK0EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCK1EN"]
    #[inline(always)]
    #[must_use]
    pub fn cck1en(&mut self) -> CCK1EN_W<CKGCRrs> {
        CCK1EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - CKGMOD"]
    #[inline(always)]
    #[must_use]
    pub fn ckgmod(&mut self) -> CKGMOD_W<CKGCRrs> {
        CKGMOD_W::new(self, 4)
    }
    #[doc = "Bit 5 - CCK0DIR"]
    #[inline(always)]
    #[must_use]
    pub fn cck0dir(&mut self) -> CCK0DIR_W<CKGCRrs> {
        CCK0DIR_W::new(self, 5)
    }
    #[doc = "Bit 6 - CCK1DIR"]
    #[inline(always)]
    #[must_use]
    pub fn cck1dir(&mut self) -> CCK1DIR_W<CKGCRrs> {
        CCK1DIR_W::new(self, 6)
    }
    #[doc = "Bit 8 - TRGSENS"]
    #[inline(always)]
    #[must_use]
    pub fn trgsens(&mut self) -> TRGSENS_W<CKGCRrs> {
        TRGSENS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - TRGSRC"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<CKGCRrs> {
        TRGSRC_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - CCKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn cckdiv(&mut self) -> CCKDIV_W<CKGCRrs> {
        CCKDIV_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PROCDIV"]
    #[inline(always)]
    #[must_use]
    pub fn procdiv(&mut self) -> PROCDIV_W<CKGCRrs> {
        PROCDIV_W::new(self, 24)
    }
    #[doc = "Bit 31 - CKGACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn ckgactive(&mut self) -> CKGACTIVE_W<CKGCRrs> {
        CKGACTIVE_W::new(self, 31)
    }
}
#[doc = "MDF clock generator control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGCRrs;
impl crate::RegisterSpec for CKGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgcr::R`](R) reader structure"]
impl crate::Readable for CKGCRrs {}
#[doc = "`write(|w| ..)` method takes [`ckgcr::W`](W) writer structure"]
impl crate::Writable for CKGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGCR to value 0"]
impl crate::Resettable for CKGCRrs {
    const RESET_VALUE: u32 = 0;
}
