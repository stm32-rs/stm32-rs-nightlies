#[doc = "Register `KEYR` reader"]
pub type R = crate::R<KEYRrs>;
#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KEYRrs>;
#[doc = "Field `KEY` reader - Semaphore Clear Key"]
pub type KEY_R = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Semaphore Clear Key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYRrs> {
        KEY_W::new(self, 16)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
