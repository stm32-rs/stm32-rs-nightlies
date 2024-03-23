#[doc = "Register `AR` reader"]
pub type R = crate::R<ARrs>;
#[doc = "Register `AR` writer"]
pub type W = crate::W<ARrs>;
#[doc = "Field `ADDRESS` reader - ADDRESS"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - ADDRESS"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDRESS"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<ARrs> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARrs;
impl crate::RegisterSpec for ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar::R`](R) reader structure"]
impl crate::Readable for ARrs {}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ARrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for ARrs {
    const RESET_VALUE: u32 = 0;
}
