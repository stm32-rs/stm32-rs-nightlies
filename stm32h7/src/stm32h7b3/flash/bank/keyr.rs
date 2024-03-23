#[doc = "Register `KEYR` reader"]
pub type R = crate::R<KEYRrs>;
#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KEYRrs>;
#[doc = "Field `KEYR` reader - Bank 1 access configuration unlock key"]
pub type KEYR_R = crate::FieldReader<u32>;
#[doc = "Field `KEYR` writer - Bank 1 access configuration unlock key"]
pub type KEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr(&self) -> KEYR_R {
        KEYR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn keyr(&mut self) -> KEYR_W<KEYRrs> {
        KEYR_W::new(self, 0)
    }
}
#[doc = "FLASH key register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYRrs;
impl crate::RegisterSpec for KEYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr::R`](R) reader structure"]
impl crate::Readable for KEYRrs {}
#[doc = "`write(|w| ..)` method takes [`keyr::W`](W) writer structure"]
impl crate::Writable for KEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KEYRrs {
    const RESET_VALUE: u32 = 0;
}
