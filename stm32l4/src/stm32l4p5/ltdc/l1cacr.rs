#[doc = "Register `L1CACR` reader"]
pub type R = crate::R<L1CACRrs>;
#[doc = "Register `L1CACR` writer"]
pub type W = crate::W<L1CACRrs>;
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
    pub fn consta(&mut self) -> CONSTA_W<L1CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
#[doc = "LTDC Layer Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CACRrs;
impl crate::RegisterSpec for L1CACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cacr::R`](R) reader structure"]
impl crate::Readable for L1CACRrs {}
#[doc = "`write(|w| ..)` method takes [`l1cacr::W`](W) writer structure"]
impl crate::Writable for L1CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CACR to value 0"]
impl crate::Resettable for L1CACRrs {
    const RESET_VALUE: u32 = 0;
}
