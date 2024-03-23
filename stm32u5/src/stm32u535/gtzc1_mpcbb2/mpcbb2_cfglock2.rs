#[doc = "Register `MPCBB2_CFGLOCK2` reader"]
pub type R = crate::R<MPCBB2_CFGLOCK2rs>;
#[doc = "Register `MPCBB2_CFGLOCK2` writer"]
pub type W = crate::W<MPCBB2_CFGLOCK2rs>;
#[doc = "Field `SPLCK32` reader - SPLCK32"]
pub type SPLCK32_R = crate::BitReader;
#[doc = "Field `SPLCK32` writer - SPLCK32"]
pub type SPLCK32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK33` reader - SPLCK33"]
pub type SPLCK33_R = crate::BitReader;
#[doc = "Field `SPLCK33` writer - SPLCK33"]
pub type SPLCK33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK34` reader - SPLCK34"]
pub type SPLCK34_R = crate::BitReader;
#[doc = "Field `SPLCK34` writer - SPLCK34"]
pub type SPLCK34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK35` reader - SPLCK35"]
pub type SPLCK35_R = crate::BitReader;
#[doc = "Field `SPLCK35` writer - SPLCK35"]
pub type SPLCK35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK36` reader - SPLCK36"]
pub type SPLCK36_R = crate::BitReader;
#[doc = "Field `SPLCK36` writer - SPLCK36"]
pub type SPLCK36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK37` reader - SPLCK37"]
pub type SPLCK37_R = crate::BitReader;
#[doc = "Field `SPLCK37` writer - SPLCK37"]
pub type SPLCK37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK38` reader - SPLCK38"]
pub type SPLCK38_R = crate::BitReader;
#[doc = "Field `SPLCK38` writer - SPLCK38"]
pub type SPLCK38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK39` reader - SPLCK39"]
pub type SPLCK39_R = crate::BitReader;
#[doc = "Field `SPLCK39` writer - SPLCK39"]
pub type SPLCK39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK40` reader - SPLCK40"]
pub type SPLCK40_R = crate::BitReader;
#[doc = "Field `SPLCK40` writer - SPLCK40"]
pub type SPLCK40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK41` reader - SPLCK41"]
pub type SPLCK41_R = crate::BitReader;
#[doc = "Field `SPLCK41` writer - SPLCK41"]
pub type SPLCK41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK42` reader - SPLCK42"]
pub type SPLCK42_R = crate::BitReader;
#[doc = "Field `SPLCK42` writer - SPLCK42"]
pub type SPLCK42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK43` reader - SPLCK43"]
pub type SPLCK43_R = crate::BitReader;
#[doc = "Field `SPLCK43` writer - SPLCK43"]
pub type SPLCK43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK44` reader - SPLCK44"]
pub type SPLCK44_R = crate::BitReader;
#[doc = "Field `SPLCK44` writer - SPLCK44"]
pub type SPLCK44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK45` reader - SPLCK45"]
pub type SPLCK45_R = crate::BitReader;
#[doc = "Field `SPLCK45` writer - SPLCK45"]
pub type SPLCK45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK46` reader - SPLCK46"]
pub type SPLCK46_R = crate::BitReader;
#[doc = "Field `SPLCK46` writer - SPLCK46"]
pub type SPLCK46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK47` reader - SPLCK47"]
pub type SPLCK47_R = crate::BitReader;
#[doc = "Field `SPLCK47` writer - SPLCK47"]
pub type SPLCK47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK48` reader - SPLCK48"]
pub type SPLCK48_R = crate::BitReader;
#[doc = "Field `SPLCK48` writer - SPLCK48"]
pub type SPLCK48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK49` reader - SPLCK49"]
pub type SPLCK49_R = crate::BitReader;
#[doc = "Field `SPLCK49` writer - SPLCK49"]
pub type SPLCK49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK50` reader - SPLCK50"]
pub type SPLCK50_R = crate::BitReader;
#[doc = "Field `SPLCK50` writer - SPLCK50"]
pub type SPLCK50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLCK51` reader - SPLCK51"]
pub type SPLCK51_R = crate::BitReader;
#[doc = "Field `SPLCK51` writer - SPLCK51"]
pub type SPLCK51_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPLCK32"]
    #[inline(always)]
    pub fn splck32(&self) -> SPLCK32_R {
        SPLCK32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPLCK33"]
    #[inline(always)]
    pub fn splck33(&self) -> SPLCK33_R {
        SPLCK33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPLCK34"]
    #[inline(always)]
    pub fn splck34(&self) -> SPLCK34_R {
        SPLCK34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPLCK35"]
    #[inline(always)]
    pub fn splck35(&self) -> SPLCK35_R {
        SPLCK35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPLCK36"]
    #[inline(always)]
    pub fn splck36(&self) -> SPLCK36_R {
        SPLCK36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPLCK37"]
    #[inline(always)]
    pub fn splck37(&self) -> SPLCK37_R {
        SPLCK37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPLCK38"]
    #[inline(always)]
    pub fn splck38(&self) -> SPLCK38_R {
        SPLCK38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPLCK39"]
    #[inline(always)]
    pub fn splck39(&self) -> SPLCK39_R {
        SPLCK39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPLCK40"]
    #[inline(always)]
    pub fn splck40(&self) -> SPLCK40_R {
        SPLCK40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPLCK41"]
    #[inline(always)]
    pub fn splck41(&self) -> SPLCK41_R {
        SPLCK41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPLCK42"]
    #[inline(always)]
    pub fn splck42(&self) -> SPLCK42_R {
        SPLCK42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPLCK43"]
    #[inline(always)]
    pub fn splck43(&self) -> SPLCK43_R {
        SPLCK43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPLCK44"]
    #[inline(always)]
    pub fn splck44(&self) -> SPLCK44_R {
        SPLCK44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPLCK45"]
    #[inline(always)]
    pub fn splck45(&self) -> SPLCK45_R {
        SPLCK45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPLCK46"]
    #[inline(always)]
    pub fn splck46(&self) -> SPLCK46_R {
        SPLCK46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPLCK47"]
    #[inline(always)]
    pub fn splck47(&self) -> SPLCK47_R {
        SPLCK47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPLCK48"]
    #[inline(always)]
    pub fn splck48(&self) -> SPLCK48_R {
        SPLCK48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPLCK49"]
    #[inline(always)]
    pub fn splck49(&self) -> SPLCK49_R {
        SPLCK49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SPLCK50"]
    #[inline(always)]
    pub fn splck50(&self) -> SPLCK50_R {
        SPLCK50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPLCK51"]
    #[inline(always)]
    pub fn splck51(&self) -> SPLCK51_R {
        SPLCK51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPLCK32"]
    #[inline(always)]
    #[must_use]
    pub fn splck32(&mut self) -> SPLCK32_W<MPCBB2_CFGLOCK2rs> {
        SPLCK32_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPLCK33"]
    #[inline(always)]
    #[must_use]
    pub fn splck33(&mut self) -> SPLCK33_W<MPCBB2_CFGLOCK2rs> {
        SPLCK33_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPLCK34"]
    #[inline(always)]
    #[must_use]
    pub fn splck34(&mut self) -> SPLCK34_W<MPCBB2_CFGLOCK2rs> {
        SPLCK34_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPLCK35"]
    #[inline(always)]
    #[must_use]
    pub fn splck35(&mut self) -> SPLCK35_W<MPCBB2_CFGLOCK2rs> {
        SPLCK35_W::new(self, 3)
    }
    #[doc = "Bit 4 - SPLCK36"]
    #[inline(always)]
    #[must_use]
    pub fn splck36(&mut self) -> SPLCK36_W<MPCBB2_CFGLOCK2rs> {
        SPLCK36_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPLCK37"]
    #[inline(always)]
    #[must_use]
    pub fn splck37(&mut self) -> SPLCK37_W<MPCBB2_CFGLOCK2rs> {
        SPLCK37_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPLCK38"]
    #[inline(always)]
    #[must_use]
    pub fn splck38(&mut self) -> SPLCK38_W<MPCBB2_CFGLOCK2rs> {
        SPLCK38_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPLCK39"]
    #[inline(always)]
    #[must_use]
    pub fn splck39(&mut self) -> SPLCK39_W<MPCBB2_CFGLOCK2rs> {
        SPLCK39_W::new(self, 7)
    }
    #[doc = "Bit 8 - SPLCK40"]
    #[inline(always)]
    #[must_use]
    pub fn splck40(&mut self) -> SPLCK40_W<MPCBB2_CFGLOCK2rs> {
        SPLCK40_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPLCK41"]
    #[inline(always)]
    #[must_use]
    pub fn splck41(&mut self) -> SPLCK41_W<MPCBB2_CFGLOCK2rs> {
        SPLCK41_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPLCK42"]
    #[inline(always)]
    #[must_use]
    pub fn splck42(&mut self) -> SPLCK42_W<MPCBB2_CFGLOCK2rs> {
        SPLCK42_W::new(self, 10)
    }
    #[doc = "Bit 11 - SPLCK43"]
    #[inline(always)]
    #[must_use]
    pub fn splck43(&mut self) -> SPLCK43_W<MPCBB2_CFGLOCK2rs> {
        SPLCK43_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPLCK44"]
    #[inline(always)]
    #[must_use]
    pub fn splck44(&mut self) -> SPLCK44_W<MPCBB2_CFGLOCK2rs> {
        SPLCK44_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPLCK45"]
    #[inline(always)]
    #[must_use]
    pub fn splck45(&mut self) -> SPLCK45_W<MPCBB2_CFGLOCK2rs> {
        SPLCK45_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPLCK46"]
    #[inline(always)]
    #[must_use]
    pub fn splck46(&mut self) -> SPLCK46_W<MPCBB2_CFGLOCK2rs> {
        SPLCK46_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPLCK47"]
    #[inline(always)]
    #[must_use]
    pub fn splck47(&mut self) -> SPLCK47_W<MPCBB2_CFGLOCK2rs> {
        SPLCK47_W::new(self, 15)
    }
    #[doc = "Bit 16 - SPLCK48"]
    #[inline(always)]
    #[must_use]
    pub fn splck48(&mut self) -> SPLCK48_W<MPCBB2_CFGLOCK2rs> {
        SPLCK48_W::new(self, 16)
    }
    #[doc = "Bit 17 - SPLCK49"]
    #[inline(always)]
    #[must_use]
    pub fn splck49(&mut self) -> SPLCK49_W<MPCBB2_CFGLOCK2rs> {
        SPLCK49_W::new(self, 17)
    }
    #[doc = "Bit 18 - SPLCK50"]
    #[inline(always)]
    #[must_use]
    pub fn splck50(&mut self) -> SPLCK50_W<MPCBB2_CFGLOCK2rs> {
        SPLCK50_W::new(self, 18)
    }
    #[doc = "Bit 19 - SPLCK51"]
    #[inline(always)]
    #[must_use]
    pub fn splck51(&mut self) -> SPLCK51_W<MPCBB2_CFGLOCK2rs> {
        SPLCK51_W::new(self, 19)
    }
}
#[doc = "GTZC1 SRAMz MPCBB configuration lock register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb2_cfglock2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb2_cfglock2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCBB2_CFGLOCK2rs;
impl crate::RegisterSpec for MPCBB2_CFGLOCK2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcbb2_cfglock2::R`](R) reader structure"]
impl crate::Readable for MPCBB2_CFGLOCK2rs {}
#[doc = "`write(|w| ..)` method takes [`mpcbb2_cfglock2::W`](W) writer structure"]
impl crate::Writable for MPCBB2_CFGLOCK2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCBB2_CFGLOCK2 to value 0"]
impl crate::Resettable for MPCBB2_CFGLOCK2rs {
    const RESET_VALUE: u32 = 0;
}
