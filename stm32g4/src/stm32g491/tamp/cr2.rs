#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `TAMP1NOER` reader - TAMP1NOER"]
pub type TAMP1NOER_R = crate::BitReader;
#[doc = "Field `TAMP1NOER` writer - TAMP1NOER"]
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2NOER` reader - TAMP2NOER"]
pub type TAMP2NOER_R = crate::BitReader;
#[doc = "Field `TAMP2NOER` writer - TAMP2NOER"]
pub type TAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3NOER` reader - TAMP3NOER"]
pub type TAMP3NOER_R = crate::BitReader;
#[doc = "Field `TAMP3NOER` writer - TAMP3NOER"]
pub type TAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1MSK` reader - TAMP1MSK"]
pub type TAMP1MSK_R = crate::BitReader;
#[doc = "Field `TAMP1MSK` writer - TAMP1MSK"]
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2MSK` reader - TAMP2MSK"]
pub type TAMP2MSK_R = crate::BitReader;
#[doc = "Field `TAMP2MSK` writer - TAMP2MSK"]
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3MSK` reader - TAMP3MSK"]
pub type TAMP3MSK_R = crate::BitReader;
#[doc = "Field `TAMP3MSK` writer - TAMP3MSK"]
pub type TAMP3MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - TAMP1TRG"]
pub type TAMP1TRG_R = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - TAMP1TRG"]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - TAMP2TRG"]
pub type TAMP2TRG_R = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - TAMP2TRG"]
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3TRG` reader - TAMP3TRG"]
pub type TAMP3TRG_R = crate::BitReader;
#[doc = "Field `TAMP3TRG` writer - TAMP3TRG"]
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<CR2rs> {
        TAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<CR2rs> {
        TAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W<CR2rs> {
        TAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<CR2rs> {
        TAMP3MSK_W::new(self, 18)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<CR2rs> {
        TAMP3TRG_W::new(self, 26)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
