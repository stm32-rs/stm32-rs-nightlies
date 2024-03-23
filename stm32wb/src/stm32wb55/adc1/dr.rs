#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `RDATA_0_6` reader - Regular Data converted 0_6"]
pub type RDATA_0_6_R = crate::FieldReader;
#[doc = "Field `RDATA_0_6` writer - Regular Data converted 0_6"]
pub type RDATA_0_6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RDATA_7_15` reader - 15"]
pub type RDATA_7_15_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    pub fn rdata_0_6(&self) -> RDATA_0_6_R {
        RDATA_0_6_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:15 - 15"]
    #[inline(always)]
    pub fn rdata_7_15(&self) -> RDATA_7_15_R {
        RDATA_7_15_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    #[must_use]
    pub fn rdata_0_6(&mut self) -> RDATA_0_6_W<DRrs> {
        RDATA_0_6_W::new(self, 0)
    }
}
#[doc = "ADC group regular conversion data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
