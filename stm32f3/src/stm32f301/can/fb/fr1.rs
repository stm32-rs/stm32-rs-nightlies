#[doc = "Register `FR1` reader"]
pub type R = crate::R<FR1rs>;
#[doc = "Register `FR1` writer"]
pub type W = crate::W<FR1rs>;
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
    pub fn fb(&mut self) -> FB_W<FR1rs> {
        FB_W::new(self, 0)
    }
}
#[doc = "Filter bank x register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR1rs;
impl crate::RegisterSpec for FR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr1::R`](R) reader structure"]
impl crate::Readable for FR1rs {}
#[doc = "`write(|w| ..)` method takes [`fr1::W`](W) writer structure"]
impl crate::Writable for FR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FR1 to value 0"]
impl crate::Resettable for FR1rs {
    const RESET_VALUE: u32 = 0;
}
