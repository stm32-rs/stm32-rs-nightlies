#[doc = "Register `CACR` reader"]
pub type R = crate::R<CACRrs>;
#[doc = "Register `CACR` writer"]
pub type W = crate::W<CACRrs>;
#[doc = "Field `CONSTA` reader - Constant Alpha"]
pub type CONSTA_R = crate::FieldReader;
#[doc = "Field `CONSTA` writer - Constant Alpha"]
pub type CONSTA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
#[doc = "Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACRrs;
impl crate::RegisterSpec for CACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CACRrs {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACRrs {
    const RESET_VALUE: u32 = 0;
}
