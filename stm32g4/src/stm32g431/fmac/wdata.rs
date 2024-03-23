#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATArs>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WDATArs> {
        WDATA_W::new(self, 0)
    }
}
#[doc = "FMAC Write Data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0;
}
