#[doc = "Register `AF` reader"]
pub type R = crate::R<AFrs>;
#[doc = "Register `AF` writer"]
pub type W = crate::W<AFrs>;
#[doc = "Field `ETRSEL` reader - External trigger source selection"]
pub type ETRSEL_R = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - External trigger source selection"]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<AFrs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM2 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFrs;
impl crate::RegisterSpec for AFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af::R`](R) reader structure"]
impl crate::Readable for AFrs {}
#[doc = "`write(|w| ..)` method takes [`af::W`](W) writer structure"]
impl crate::Writable for AFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF to value 0"]
impl crate::Resettable for AFrs {
    const RESET_VALUE: u32 = 0;
}
