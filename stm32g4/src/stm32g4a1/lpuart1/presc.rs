#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PRESCrs>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PRESCrs>;
#[doc = "Field `PRESCALER` reader - PRESCALER"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - PRESCALER"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PRESCALER"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRESCALER"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESCrs;
impl crate::RegisterSpec for PRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PRESCrs {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESCrs {
    const RESET_VALUE: u32 = 0;
}
