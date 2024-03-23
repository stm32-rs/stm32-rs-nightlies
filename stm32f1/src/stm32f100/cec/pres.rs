#[doc = "Register `PRES` reader"]
pub type R = crate::R<PRESrs>;
#[doc = "Register `PRES` writer"]
pub type W = crate::W<PRESrs>;
#[doc = "Field `PRESC` reader - CEC Rx Data Register"]
pub type PRESC_R = crate::FieldReader<u16>;
#[doc = "Field `PRESC` writer - CEC Rx Data Register"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - CEC Rx Data Register"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CEC Rx Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<PRESrs> {
        PRESC_W::new(self, 0)
    }
}
#[doc = "Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESrs;
impl crate::RegisterSpec for PRESrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pres::R`](R) reader structure"]
impl crate::Readable for PRESrs {}
#[doc = "`write(|w| ..)` method takes [`pres::W`](W) writer structure"]
impl crate::Writable for PRESrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRES to value 0"]
impl crate::Resettable for PRESrs {
    const RESET_VALUE: u32 = 0;
}
