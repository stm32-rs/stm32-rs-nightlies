#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `COREID` reader - CoreID of semaphore to be cleared"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `COREID` writer - CoreID of semaphore to be cleared"]
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY` reader - Semaphore clear Key"]
pub type KEY_R = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Semaphore clear Key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:11 - CoreID of semaphore to be cleared"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - CoreID of semaphore to be cleared"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<CRrs> {
        COREID_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CRrs> {
        KEY_W::new(self, 16)
    }
}
#[doc = "Semaphore Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
