#[doc = "Register `HTR` reader"]
pub type R = crate::R<HTRrs>;
#[doc = "Register `HTR` writer"]
pub type W = crate::W<HTRrs>;
#[doc = "Field `HT` reader - Analog watchdog higher threshold"]
pub type HT_R = crate::FieldReader<u16>;
#[doc = "Field `HT` writer - Analog watchdog higher threshold"]
pub type HT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<HTRrs> {
        HT_W::new(self, 0)
    }
}
#[doc = "watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTRrs;
impl crate::RegisterSpec for HTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr::R`](R) reader structure"]
impl crate::Readable for HTRrs {}
#[doc = "`write(|w| ..)` method takes [`htr::W`](W) writer structure"]
impl crate::Writable for HTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTR to value 0x0fff"]
impl crate::Resettable for HTRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
