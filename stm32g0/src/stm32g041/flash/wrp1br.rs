#[doc = "Register `WRP1BR` reader"]
pub type R = crate::R<WRP1BRrs>;
#[doc = "Register `WRP1BR` writer"]
pub type W = crate::W<WRP1BRrs>;
#[doc = "Field `WRP1B_STRT` reader - WRP area B start offset"]
pub type WRP1B_STRT_R = crate::FieldReader;
#[doc = "Field `WRP1B_STRT` writer - WRP area B start offset"]
pub type WRP1B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRP1B_END` reader - WRP area B end offset"]
pub type WRP1B_END_R = crate::FieldReader;
#[doc = "Field `WRP1B_END` writer - WRP area B end offset"]
pub type WRP1B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - WRP area B start offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> WRP1B_STRT_R {
        WRP1B_STRT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WRP area B end offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> WRP1B_END_R {
        WRP1B_END_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WRP area B start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_strt(&mut self) -> WRP1B_STRT_W<WRP1BRrs> {
        WRP1B_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - WRP area B end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_end(&mut self) -> WRP1B_END_W<WRP1BRrs> {
        WRP1B_END_W::new(self, 16)
    }
}
#[doc = "Flash WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1BRrs;
impl crate::RegisterSpec for WRP1BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1br::R`](R) reader structure"]
impl crate::Readable for WRP1BRrs {}
#[doc = "`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure"]
impl crate::Writable for WRP1BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1BR to value 0xff"]
impl crate::Resettable for WRP1BRrs {
    const RESET_VALUE: u32 = 0xff;
}
