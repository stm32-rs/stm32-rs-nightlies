#[doc = "Register `TWCR` reader"]
pub type R = crate::R<TWCRrs>;
#[doc = "Register `TWCR` writer"]
pub type W = crate::W<TWCRrs>;
#[doc = "Field `TOTALH` reader - Total Height (in units of horizontal scan line)"]
pub type TOTALH_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALH` writer - Total Height (in units of horizontal scan line)"]
pub type TOTALH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 11, u16>;
#[doc = "Field `TOTALW` reader - Total Width (in units of pixel clock period)"]
pub type TOTALW_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALW` writer - Total Width (in units of pixel clock period)"]
pub type TOTALW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn totalh(&mut self) -> TOTALH_W<TWCRrs> {
        TOTALH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    #[must_use]
    pub fn totalw(&mut self) -> TOTALW_W<TWCRrs> {
        TOTALW_W::new(self, 16)
    }
}
#[doc = "Total Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWCRrs;
impl crate::RegisterSpec for TWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twcr::R`](R) reader structure"]
impl crate::Readable for TWCRrs {}
#[doc = "`write(|w| ..)` method takes [`twcr::W`](W) writer structure"]
impl crate::Writable for TWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TWCR to value 0"]
impl crate::Resettable for TWCRrs {
    const RESET_VALUE: u32 = 0;
}
