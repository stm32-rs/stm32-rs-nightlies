#[doc = "Register `M1AR` reader"]
pub type R = crate::R<M1ARrs>;
#[doc = "Register `M1AR` writer"]
pub type W = crate::W<M1ARrs>;
#[doc = "Field `M1A` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_R = crate::FieldReader<u32>;
#[doc = "Field `M1A` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1a(&mut self) -> M1A_W<M1ARrs> {
        M1A_W::new(self, 0)
    }
}
#[doc = "stream x memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1ARrs;
impl crate::RegisterSpec for M1ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1ar::R`](R) reader structure"]
impl crate::Readable for M1ARrs {}
#[doc = "`write(|w| ..)` method takes [`m1ar::W`](W) writer structure"]
impl crate::Writable for M1ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1AR to value 0"]
impl crate::Resettable for M1ARrs {
    const RESET_VALUE: u32 = 0;
}
