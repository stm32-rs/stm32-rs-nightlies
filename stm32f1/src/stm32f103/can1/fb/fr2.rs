#[doc = "Register `FR2` reader"]
pub type R = crate::R<FR2rs>;
#[doc = "Register `FR2` writer"]
pub type W = crate::W<FR2rs>;
#[doc = "Field `FB` reader - Filter bits"]
pub type FB_R = crate::FieldReader<u32>;
#[doc = "Field `FB` writer - Filter bits"]
pub type FB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<FR2rs> {
        FB_W::new(self, 0)
    }
}
#[doc = "Filter bank x register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR2rs;
impl crate::RegisterSpec for FR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr2::R`](R) reader structure"]
impl crate::Readable for FR2rs {}
#[doc = "`write(|w| ..)` method takes [`fr2::W`](W) writer structure"]
impl crate::Writable for FR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FR2 to value 0"]
impl crate::Resettable for FR2rs {
    const RESET_VALUE: u32 = 0;
}
