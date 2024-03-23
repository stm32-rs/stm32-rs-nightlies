#[doc = "Register `WRP1AR` reader"]
pub type R = crate::R<WRP1ARrs>;
#[doc = "Register `WRP1AR` writer"]
pub type W = crate::W<WRP1ARrs>;
#[doc = "Field `WRP1A_STRT` reader - Bank 1 WRP first area start offset"]
pub type WRP1A_STRT_R = crate::FieldReader;
#[doc = "Field `WRP1A_STRT` writer - Bank 1 WRP first area start offset"]
pub type WRP1A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP1A_END` reader - Bank 1 WRP first area A end offset"]
pub type WRP1A_END_R = crate::FieldReader;
#[doc = "Field `WRP1A_END` writer - Bank 1 WRP first area A end offset"]
pub type WRP1A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_strt(&mut self) -> WRP1A_STRT_W<WRP1ARrs> {
        WRP1A_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_end(&mut self) -> WRP1A_END_W<WRP1ARrs> {
        WRP1A_END_W::new(self, 16)
    }
}
#[doc = "Flash Bank 1 WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1ARrs;
impl crate::RegisterSpec for WRP1ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1ar::R`](R) reader structure"]
impl crate::Readable for WRP1ARrs {}
#[doc = "`write(|w| ..)` method takes [`wrp1ar::W`](W) writer structure"]
impl crate::Writable for WRP1ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1AR to value 0"]
impl crate::Resettable for WRP1ARrs {
    const RESET_VALUE: u32 = 0;
}
