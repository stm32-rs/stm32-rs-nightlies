#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `OR1` reader - Option register bit 1"]
pub type OR1_R = crate::BitReader;
#[doc = "Field `OR1` writer - Option register bit 1"]
pub type OR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OR2` reader - Option register bit 2"]
pub type OR2_R = crate::BitReader;
#[doc = "Field `OR2` writer - Option register bit 2"]
pub type OR2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option register bit 1"]
    #[inline(always)]
    pub fn or1(&self) -> OR1_R {
        OR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option register bit 2"]
    #[inline(always)]
    pub fn or2(&self) -> OR2_R {
        OR2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or1(&mut self) -> OR1_W<ORrs> {
        OR1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option register bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn or2(&mut self) -> OR2_W<ORrs> {
        OR2_W::new(self, 1)
    }
}
#[doc = "Option Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
