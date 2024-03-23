#[doc = "Register `TR` reader"]
pub type R = crate::R<TRrs>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TRrs>;
#[doc = "Field `LT` reader - Analog watchdog lower threshold"]
pub type LT_R = crate::FieldReader<u16>;
#[doc = "Field `LT` writer - Analog watchdog lower threshold"]
pub type LT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT` reader - Analog watchdog higher threshold"]
pub type HT_R = crate::FieldReader<u16>;
#[doc = "Field `HT` writer - Analog watchdog higher threshold"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<TRrs> {
        LT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Analog watchdog higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<TRrs> {
        HT_W::new(self, 16)
    }
}
#[doc = "watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRrs;
impl crate::RegisterSpec for TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TRrs {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR to value 0x0fff_0000"]
impl crate::Resettable for TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
