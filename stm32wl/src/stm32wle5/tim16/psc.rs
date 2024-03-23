#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSCrs>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSCrs>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCrs> {
        PSC_W::new(self, 0)
    }
}
#[doc = "TIM16/TIM17 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCrs;
impl crate::RegisterSpec for PSCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSCrs {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSCrs {
    const RESET_VALUE: u32 = 0;
}
