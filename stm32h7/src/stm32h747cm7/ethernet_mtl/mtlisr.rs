#[doc = "Register `MTLISR` reader"]
pub type R = crate::R<MTLISRrs>;
#[doc = "Register `MTLISR` writer"]
pub type W = crate::W<MTLISRrs>;
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub type Q0IS_R = crate::BitReader;
#[doc = "Field `Q0IS` writer - Queue interrupt status"]
pub type Q0IS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn q0is(&mut self) -> Q0IS_W<MTLISRrs> {
        Q0IS_W::new(self, 0)
    }
}
#[doc = "Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLISRrs;
impl crate::RegisterSpec for MTLISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlisr::R`](R) reader structure"]
impl crate::Readable for MTLISRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlisr::W`](W) writer structure"]
impl crate::Writable for MTLISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MTLISRrs {
    const RESET_VALUE: u32 = 0;
}
