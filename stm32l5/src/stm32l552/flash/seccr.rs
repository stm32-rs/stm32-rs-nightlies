#[doc = "Register `SECCR` reader"]
pub type R = crate::R<SECCRrs>;
#[doc = "Register `SECCR` writer"]
pub type W = crate::W<SECCRrs>;
#[doc = "Field `SECPG` reader - SECPG"]
pub type SECPG_R = crate::BitReader;
#[doc = "Field `SECPG` writer - SECPG"]
pub type SECPG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECPER` reader - SECPER"]
pub type SECPER_R = crate::BitReader;
#[doc = "Field `SECPER` writer - SECPER"]
pub type SECPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECMER1` reader - SECMER1"]
pub type SECMER1_R = crate::BitReader;
#[doc = "Field `SECMER1` writer - SECMER1"]
pub type SECMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECPNB` reader - SECPNB"]
pub type SECPNB_R = crate::FieldReader;
#[doc = "Field `SECPNB` writer - SECPNB"]
pub type SECPNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECBKER` reader - SECBKER"]
pub type SECBKER_R = crate::BitReader;
#[doc = "Field `SECBKER` writer - SECBKER"]
pub type SECBKER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECMER2` reader - SECMER2"]
pub type SECMER2_R = crate::BitReader;
#[doc = "Field `SECMER2` writer - SECMER2"]
pub type SECMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECSTRT` reader - SECSTRT"]
pub type SECSTRT_R = crate::BitReader;
#[doc = "Field `SECSTRT` writer - SECSTRT"]
pub type SECSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECEOPIE` reader - SECEOPIE"]
pub type SECEOPIE_R = crate::BitReader;
#[doc = "Field `SECEOPIE` writer - SECEOPIE"]
pub type SECEOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECERRIE` reader - SECERRIE"]
pub type SECERRIE_R = crate::BitReader;
#[doc = "Field `SECERRIE` writer - SECERRIE"]
pub type SECERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECRDERRIE` reader - SECRDERRIE"]
pub type SECRDERRIE_R = crate::BitReader;
#[doc = "Field `SECRDERRIE` writer - SECRDERRIE"]
pub type SECRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECINV` reader - SECINV"]
pub type SECINV_R = crate::BitReader;
#[doc = "Field `SECINV` writer - SECINV"]
pub type SECINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECLOCK` reader - SECLOCK"]
pub type SECLOCK_R = crate::BitReader;
#[doc = "Field `SECLOCK` writer - SECLOCK"]
pub type SECLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    pub fn secpg(&self) -> SECPG_R {
        SECPG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    pub fn secper(&self) -> SECPER_R {
        SECPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    pub fn secmer1(&self) -> SECMER1_R {
        SECMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    pub fn secpnb(&self) -> SECPNB_R {
        SECPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    pub fn secbker(&self) -> SECBKER_R {
        SECBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    pub fn secmer2(&self) -> SECMER2_R {
        SECMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    pub fn secstrt(&self) -> SECSTRT_R {
        SECSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    pub fn seceopie(&self) -> SECEOPIE_R {
        SECEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    pub fn secerrie(&self) -> SECERRIE_R {
        SECERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    pub fn secrderrie(&self) -> SECRDERRIE_R {
        SECRDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    pub fn secinv(&self) -> SECINV_R {
        SECINV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECPG"]
    #[inline(always)]
    #[must_use]
    pub fn secpg(&mut self) -> SECPG_W<SECCRrs> {
        SECPG_W::new(self, 0)
    }
    #[doc = "Bit 1 - SECPER"]
    #[inline(always)]
    #[must_use]
    pub fn secper(&mut self) -> SECPER_W<SECCRrs> {
        SECPER_W::new(self, 1)
    }
    #[doc = "Bit 2 - SECMER1"]
    #[inline(always)]
    #[must_use]
    pub fn secmer1(&mut self) -> SECMER1_W<SECCRrs> {
        SECMER1_W::new(self, 2)
    }
    #[doc = "Bits 3:9 - SECPNB"]
    #[inline(always)]
    #[must_use]
    pub fn secpnb(&mut self) -> SECPNB_W<SECCRrs> {
        SECPNB_W::new(self, 3)
    }
    #[doc = "Bit 11 - SECBKER"]
    #[inline(always)]
    #[must_use]
    pub fn secbker(&mut self) -> SECBKER_W<SECCRrs> {
        SECBKER_W::new(self, 11)
    }
    #[doc = "Bit 15 - SECMER2"]
    #[inline(always)]
    #[must_use]
    pub fn secmer2(&mut self) -> SECMER2_W<SECCRrs> {
        SECMER2_W::new(self, 15)
    }
    #[doc = "Bit 16 - SECSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn secstrt(&mut self) -> SECSTRT_W<SECCRrs> {
        SECSTRT_W::new(self, 16)
    }
    #[doc = "Bit 24 - SECEOPIE"]
    #[inline(always)]
    #[must_use]
    pub fn seceopie(&mut self) -> SECEOPIE_W<SECCRrs> {
        SECEOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SECERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn secerrie(&mut self) -> SECERRIE_W<SECCRrs> {
        SECERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - SECRDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn secrderrie(&mut self) -> SECRDERRIE_W<SECCRrs> {
        SECRDERRIE_W::new(self, 26)
    }
    #[doc = "Bit 29 - SECINV"]
    #[inline(always)]
    #[must_use]
    pub fn secinv(&mut self) -> SECINV_W<SECCRrs> {
        SECINV_W::new(self, 29)
    }
    #[doc = "Bit 31 - SECLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn seclock(&mut self) -> SECLOCK_W<SECCRrs> {
        SECLOCK_W::new(self, 31)
    }
}
#[doc = "Flash secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCRrs;
impl crate::RegisterSpec for SECCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccr::R`](R) reader structure"]
impl crate::Readable for SECCRrs {}
#[doc = "`write(|w| ..)` method takes [`seccr::W`](W) writer structure"]
impl crate::Writable for SECCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCR to value 0x8000_0000"]
impl crate::Resettable for SECCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
