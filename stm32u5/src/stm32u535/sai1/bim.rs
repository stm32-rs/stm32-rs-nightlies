#[doc = "Register `BIM` reader"]
pub type R = crate::R<BIMrs>;
#[doc = "Register `BIM` writer"]
pub type W = crate::W<BIMrs>;
#[doc = "Field `OVRUDRIE` reader - Overrun/underrun interrupt enable"]
pub type OVRUDRIE_R = crate::BitReader;
#[doc = "Field `OVRUDRIE` writer - Overrun/underrun interrupt enable"]
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTEDETIE` reader - Mute detection interrupt enable"]
pub type MUTEDETIE_R = crate::BitReader;
#[doc = "Field `MUTEDETIE` writer - Mute detection interrupt enable"]
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable"]
pub type WCKCFGIE_R = crate::BitReader;
#[doc = "Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable"]
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQIE` reader - FIFO request interrupt enable"]
pub type FREQIE_R = crate::BitReader;
#[doc = "Field `FREQIE` writer - FIFO request interrupt enable"]
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNRDYIE` reader - Codec not ready interrupt enable"]
pub type CNRDYIE_R = crate::BitReader;
#[doc = "Field `CNRDYIE` writer - Codec not ready interrupt enable"]
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable"]
pub type AFSDETIE_R = crate::BitReader;
#[doc = "Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable"]
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable"]
pub type LFSDETIE_R = crate::BitReader;
#[doc = "Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable"]
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<BIMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<BIMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<BIMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<BIMrs> {
        FREQIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<BIMrs> {
        CNRDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<BIMrs> {
        AFSDETIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<BIMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
#[doc = "B Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIMrs;
impl crate::RegisterSpec for BIMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bim::R`](R) reader structure"]
impl crate::Readable for BIMrs {}
#[doc = "`write(|w| ..)` method takes [`bim::W`](W) writer structure"]
impl crate::Writable for BIMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIM to value 0"]
impl crate::Resettable for BIMrs {
    const RESET_VALUE: u32 = 0;
}
