#[doc = "Register `IWDG_PR` reader"]
pub type R = crate::R<IWDG_PRrs>;
#[doc = "Register `IWDG_PR` writer"]
pub type W = crate::W<IWDG_PRrs>;
#[doc = "Field `PR` reader - PR"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - PR"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<IWDG_PRrs> {
        PR_W::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_PRrs;
impl crate::RegisterSpec for IWDG_PRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_pr::R`](R) reader structure"]
impl crate::Readable for IWDG_PRrs {}
#[doc = "`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure"]
impl crate::Writable for IWDG_PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_PR to value 0x07"]
impl crate::Resettable for IWDG_PRrs {
    const RESET_VALUE: u32 = 0x07;
}
