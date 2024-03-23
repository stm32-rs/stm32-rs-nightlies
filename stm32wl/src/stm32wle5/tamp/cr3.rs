#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "ITAMP3NOER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3NOER {
    #[doc = "0: Internal tamper x event erases the backup registers"]
    Erase = 0,
    #[doc = "1: Internal tamper x event does not erase the backup registers"]
    NotErase = 1,
}
impl From<ITAMP3NOER> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3NOER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub type ITAMP3NOER_R = crate::BitReader<ITAMP3NOER>;
impl ITAMP3NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3NOER {
        match self.bits {
            false => ITAMP3NOER::Erase,
            true => ITAMP3NOER::NotErase,
        }
    }
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ITAMP3NOER::Erase
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        *self == ITAMP3NOER::NotErase
    }
}
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3NOER>;
impl<'a, REG> ITAMP3NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER::Erase)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER::NotErase)
    }
}
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub use ITAMP3NOER_R as ITAMP5NOER_R;
#[doc = "Field `ITAMP6NOER` reader - ITAMP6NOER"]
pub use ITAMP3NOER_R as ITAMP6NOER_R;
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub use ITAMP3NOER_R as ITAMP8NOER_R;
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub use ITAMP3NOER_W as ITAMP5NOER_W;
#[doc = "Field `ITAMP6NOER` writer - ITAMP6NOER"]
pub use ITAMP3NOER_W as ITAMP6NOER_W;
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub use ITAMP3NOER_W as ITAMP8NOER_W;
impl R {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<CR3rs> {
        ITAMP6NOER_W::new(self, 5)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
}
#[doc = "TAMP control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
