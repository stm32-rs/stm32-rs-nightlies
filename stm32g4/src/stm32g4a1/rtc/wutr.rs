#[doc = "Register `WUTR` reader"]
pub type R = crate::R<WUTRrs>;
#[doc = "Register `WUTR` writer"]
pub type W = crate::W<WUTRrs>;
#[doc = "Field `WUT` reader - Wakeup auto-reload value bits"]
pub type WUT_R = crate::FieldReader<u16>;
#[doc = "Field `WUT` writer - Wakeup auto-reload value bits"]
pub type WUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits"]
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits"]
    #[inline(always)]
    #[must_use]
    pub fn wut(&mut self) -> WUT_W<WUTRrs> {
        WUT_W::new(self, 0)
    }
}
#[doc = "wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUTRrs;
impl crate::RegisterSpec for WUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wutr::R`](R) reader structure"]
impl crate::Readable for WUTRrs {}
#[doc = "`write(|w| ..)` method takes [`wutr::W`](W) writer structure"]
impl crate::Writable for WUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUTR to value 0xffff"]
impl crate::Resettable for WUTRrs {
    const RESET_VALUE: u32 = 0xffff;
}
