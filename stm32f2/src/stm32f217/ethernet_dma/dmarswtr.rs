#[doc = "Register `DMARSWTR` reader"]
pub type R = crate::R<DMARSWTRrs>;
#[doc = "Register `DMARSWTR` writer"]
pub type W = crate::W<DMARSWTRrs>;
#[doc = "Field `RSWTC` reader - Receive status watchdog timer count"]
pub type RSWTC_R = crate::FieldReader;
#[doc = "Field `RSWTC` writer - Receive status watchdog timer count"]
pub type RSWTC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive status watchdog timer count"]
    #[inline(always)]
    pub fn rswtc(&self) -> RSWTC_R {
        RSWTC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive status watchdog timer count"]
    #[inline(always)]
    #[must_use]
    pub fn rswtc(&mut self) -> RSWTC_W<DMARSWTRrs> {
        RSWTC_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive status watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarswtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarswtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARSWTRrs;
impl crate::RegisterSpec for DMARSWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarswtr::R`](R) reader structure"]
impl crate::Readable for DMARSWTRrs {}
#[doc = "`write(|w| ..)` method takes [`dmarswtr::W`](W) writer structure"]
impl crate::Writable for DMARSWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARSWTR to value 0"]
impl crate::Resettable for DMARSWTRrs {
    const RESET_VALUE: u32 = 0;
}
