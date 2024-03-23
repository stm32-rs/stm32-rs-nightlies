#[doc = "Register `TR2` reader"]
pub type R = crate::R<TR2rs>;
#[doc = "Register `TR2` writer"]
pub type W = crate::W<TR2rs>;
#[doc = "Field `LT2` reader - Analog watchdog 2 lower threshold"]
pub type LT2_R = crate::FieldReader;
#[doc = "Field `LT2` writer - Analog watchdog 2 lower threshold"]
pub type LT2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `HT2` reader - Analog watchdog 2 higher threshold"]
pub type HT2_R = crate::FieldReader;
#[doc = "Field `HT2` writer - Analog watchdog 2 higher threshold"]
pub type HT2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lt2(&mut self) -> LT2_W<TR2rs> {
        LT2_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht2(&mut self) -> HT2_W<TR2rs> {
        HT2_W::new(self, 16)
    }
}
#[doc = "watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR2rs;
impl crate::RegisterSpec for TR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr2::R`](R) reader structure"]
impl crate::Readable for TR2rs {}
#[doc = "`write(|w| ..)` method takes [`tr2::W`](W) writer structure"]
impl crate::Writable for TR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR2 to value 0x00ff_0000"]
impl crate::Resettable for TR2rs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
