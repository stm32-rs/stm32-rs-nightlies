#[doc = "Register `CH4WDATR` reader"]
pub type R = crate::R<CH4WDATRrs>;
#[doc = "Register `CH4WDATR` writer"]
pub type W = crate::W<CH4WDATRrs>;
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<CH4WDATRrs> {
        WDATA_W::new(self, 0)
    }
}
#[doc = "CHWDAT4R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4wdatr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4wdatr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4WDATRrs;
impl crate::RegisterSpec for CH4WDATRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4wdatr::R`](R) reader structure"]
impl crate::Readable for CH4WDATRrs {}
#[doc = "`write(|w| ..)` method takes [`ch4wdatr::W`](W) writer structure"]
impl crate::Writable for CH4WDATRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4WDATR to value 0"]
impl crate::Resettable for CH4WDATRrs {
    const RESET_VALUE: u32 = 0;
}
