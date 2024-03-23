#[doc = "Register `VAL` reader"]
pub type R = crate::R<VALrs>;
#[doc = "Register `VAL` writer"]
pub type W = crate::W<VALrs>;
#[doc = "Field `CURRENT` reader - Current counter value"]
pub type CURRENT_R = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Current counter value"]
pub type CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Current counter value"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<VALrs> {
        CURRENT_W::new(self, 0)
    }
}
#[doc = "SysTick current value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val::R`](R) reader structure"]
impl crate::Readable for VALrs {}
#[doc = "`write(|w| ..)` method takes [`val::W`](W) writer structure"]
impl crate::Writable for VALrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAL to value 0"]
impl crate::Resettable for VALrs {
    const RESET_VALUE: u32 = 0;
}
