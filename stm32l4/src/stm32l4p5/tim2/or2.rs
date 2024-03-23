#[doc = "Register `OR2` reader"]
pub type R = crate::R<OR2rs>;
#[doc = "Register `OR2` writer"]
pub type W = crate::W<OR2rs>;
#[doc = "Field `ETRSEL` reader - ETR source selection"]
pub type ETRSEL_R = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - ETR source selection"]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 14:16 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - ETR source selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<OR2rs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "option register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR2rs;
impl crate::RegisterSpec for OR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or2::R`](R) reader structure"]
impl crate::Readable for OR2rs {}
#[doc = "`write(|w| ..)` method takes [`or2::W`](W) writer structure"]
impl crate::Writable for OR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR2 to value 0"]
impl crate::Resettable for OR2rs {
    const RESET_VALUE: u32 = 0;
}
