#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `OR_0` reader - Option register bit 0"]
pub type OR_0_R = crate::BitReader;
#[doc = "Field `OR_0` writer - Option register bit 0"]
pub type OR_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OR_1` reader - Option register bit 1"]
pub type OR_1_R = crate::BitReader;
#[doc = "Field `OR_1` writer - Option register bit 1"]
pub type OR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn or_0(&mut self) -> OR_0_W<ORrs> {
        OR_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or_1(&mut self) -> OR_1_W<ORrs> {
        OR_1_W::new(self, 1)
    }
}
#[doc = "??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
