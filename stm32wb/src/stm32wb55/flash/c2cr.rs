#[doc = "Register `C2CR` reader"]
pub type R = crate::R<C2CRrs>;
#[doc = "Register `C2CR` writer"]
pub type W = crate::W<C2CRrs>;
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Masse erase"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Masse erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNB` reader - Page Number selection"]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Page Number selection"]
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Masse erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - Page Number selection"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<C2CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<C2CRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Masse erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<C2CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - Page Number selection"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<C2CRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<C2CRrs> {
        STRT_W::new(self, 16)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<C2CRrs> {
        FSTPG_W::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<C2CRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<C2CRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<C2CRrs> {
        RDERRIE_W::new(self, 26)
    }
}
#[doc = "CPU2 cortex M0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr::R`](R) reader structure"]
impl crate::Readable for C2CRrs {}
#[doc = "`write(|w| ..)` method takes [`c2cr::W`](W) writer structure"]
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2CRrs {
    const RESET_VALUE: u32 = 0;
}
