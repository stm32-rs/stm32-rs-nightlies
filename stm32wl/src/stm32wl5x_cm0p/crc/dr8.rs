#[doc = "Register `DR8` reader"]
pub type R = crate::R<DR8rs>;
#[doc = "Register `DR8` writer"]
pub type W = crate::W<DR8rs>;
#[doc = "Field `DR8` reader - Data register bits"]
pub type DR8_R = crate::FieldReader;
#[doc = "Field `DR8` writer - Data register bits"]
pub type DR8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn dr8(&self) -> DR8_R {
        DR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn dr8(&mut self) -> DR8_W<DR8rs> {
        DR8_W::new(self, 0)
    }
}
#[doc = "Data register - byte sized\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR8rs;
impl crate::RegisterSpec for DR8rs {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::Readable for DR8rs {}
#[doc = "`write(|w| ..)` method takes [`dr8::W`](W) writer structure"]
impl crate::Writable for DR8rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DR8 to value 0xff"]
impl crate::Resettable for DR8rs {
    const RESET_VALUE: u8 = 0xff;
}
