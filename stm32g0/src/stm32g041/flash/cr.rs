#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Mass erase"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Mass erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNB` reader - Page number"]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Page number"]
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OPTSTRT_R = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FSTPG_R = crate::BitReader;
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERRIE` reader - PCROP read error interrupt enable"]
pub type RDERRIE_R = crate::BitReader;
#[doc = "Field `RDERRIE` writer - PCROP read error interrupt enable"]
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_PROT` reader - Securable memory area protection enable"]
pub type SEC_PROT_R = crate::BitReader;
#[doc = "Field `SEC_PROT` writer - Securable memory area protection enable"]
pub type SEC_PROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - FLASH_CR Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - FLASH_CR Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Page number"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Securable memory area protection enable"]
    #[inline(always)]
    pub fn sec_prot(&self) -> SEC_PROT_R {
        SEC_PROT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bits 3:8 - Page number"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<CRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<CRrs> {
        OPTSTRT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<CRrs> {
        FSTPG_W::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<CRrs> {
        RDERRIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<CRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    #[doc = "Bit 28 - Securable memory area protection enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec_prot(&mut self) -> SEC_PROT_W<CRrs> {
        SEC_PROT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<CRrs> {
        OPTLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Flash control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0xc000_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
