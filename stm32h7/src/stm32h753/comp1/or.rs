#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `AFOP` reader - Selection of source for alternate function of output ports"]
pub type AFOP_R = crate::FieldReader<u16>;
#[doc = "Field `AFOP` writer - Selection of source for alternate function of output ports"]
pub type AFOP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OR` reader - Option Register"]
pub type OR_R = crate::FieldReader<u32>;
#[doc = "Field `OR` writer - Option Register"]
pub type OR_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&self) -> AFOP_R {
        AFOP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    #[must_use]
    pub fn afop(&mut self) -> AFOP_W<ORrs> {
        AFOP_W::new(self, 0)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OR_W<ORrs> {
        OR_W::new(self, 11)
    }
}
#[doc = "Comparator option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
