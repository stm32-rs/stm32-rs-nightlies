#[doc = "Register `MPER` reader"]
pub type R = crate::R<MPERrs>;
#[doc = "Register `MPER` writer"]
pub type W = crate::W<MPERrs>;
#[doc = "Field `MPER` reader - Master Timer Period value"]
pub type MPER_R = crate::FieldReader<u16>;
#[doc = "Field `MPER` writer - Master Timer Period value"]
pub type MPER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<MPERrs> {
        MPER_W::new(self, 0)
    }
}
#[doc = "Master Timer Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPERrs;
impl crate::RegisterSpec for MPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mper::R`](R) reader structure"]
impl crate::Readable for MPERrs {}
#[doc = "`write(|w| ..)` method takes [`mper::W`](W) writer structure"]
impl crate::Writable for MPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPER to value 0xffff"]
impl crate::Resettable for MPERrs {
    const RESET_VALUE: u32 = 0xffff;
}
