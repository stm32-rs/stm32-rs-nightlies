#[doc = "Register `SAI_BIM` reader"]
pub type R = crate::R<SAI_BIMrs>;
#[doc = "Register `SAI_BIM` writer"]
pub type W = crate::W<SAI_BIMrs>;
#[doc = "Field `OVRUDRIE` reader - OVRUDRIE"]
pub type OVRUDRIE_R = crate::BitReader;
#[doc = "Field `OVRUDRIE` writer - OVRUDRIE"]
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTEDETIE` reader - MUTEDETIE"]
pub type MUTEDETIE_R = crate::BitReader;
#[doc = "Field `MUTEDETIE` writer - MUTEDETIE"]
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCKCFGIE` reader - WCKCFGIE"]
pub type WCKCFGIE_R = crate::BitReader;
#[doc = "Field `WCKCFGIE` writer - WCKCFGIE"]
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQIE` reader - FREQIE"]
pub type FREQIE_R = crate::BitReader;
#[doc = "Field `FREQIE` writer - FREQIE"]
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNRDYIE` reader - CNRDYIE"]
pub type CNRDYIE_R = crate::BitReader;
#[doc = "Field `CNRDYIE` writer - CNRDYIE"]
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFSDETIE` reader - AFSDETIE"]
pub type AFSDETIE_R = crate::BitReader;
#[doc = "Field `AFSDETIE` writer - AFSDETIE"]
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSDETIE` reader - LFSDETIE"]
pub type LFSDETIE_R = crate::BitReader;
#[doc = "Field `LFSDETIE` writer - LFSDETIE"]
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<SAI_BIMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<SAI_BIMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<SAI_BIMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<SAI_BIMrs> {
        FREQIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<SAI_BIMrs> {
        CNRDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<SAI_BIMrs> {
        AFSDETIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<SAI_BIMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_BIMrs;
impl crate::RegisterSpec for SAI_BIMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_bim::R`](R) reader structure"]
impl crate::Readable for SAI_BIMrs {}
#[doc = "`write(|w| ..)` method takes [`sai_bim::W`](W) writer structure"]
impl crate::Writable for SAI_BIMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_BIM to value 0"]
impl crate::Resettable for SAI_BIMrs {
    const RESET_VALUE: u32 = 0;
}
