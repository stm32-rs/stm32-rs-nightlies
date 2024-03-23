#[doc = "Register `WTCR` reader"]
pub type R = crate::R<WTCRrs>;
#[doc = "Register `WTCR` writer"]
pub type W = crate::W<WTCRrs>;
#[doc = "Field `DCYC` reader - Number of dummy cycles"]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles"]
pub type DCYC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<WTCRrs> {
        DCYC_W::new(self, 0)
    }
}
#[doc = "write timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WTCRrs;
impl crate::RegisterSpec for WTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtcr::R`](R) reader structure"]
impl crate::Readable for WTCRrs {}
#[doc = "`write(|w| ..)` method takes [`wtcr::W`](W) writer structure"]
impl crate::Writable for WTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTCR to value 0"]
impl crate::Resettable for WTCRrs {
    const RESET_VALUE: u32 = 0;
}
