#[doc = "Register `WPABR` reader"]
pub type R = crate::R<WPABRrs>;
#[doc = "Register `WPABR` writer"]
pub type W = crate::W<WPABRrs>;
#[doc = "Field `ALTERNATE` reader - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
pub type ALTERNATE_R = crate::FieldReader<u32>;
#[doc = "Field `ALTERNATE` writer - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
pub type ALTERNATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<WPABRrs> {
        ALTERNATE_W::new(self, 0)
    }
}
#[doc = "OCTOSPI wrap alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpabr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpabr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPABRrs;
impl crate::RegisterSpec for WPABRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpabr::R`](R) reader structure"]
impl crate::Readable for WPABRrs {}
#[doc = "`write(|w| ..)` method takes [`wpabr::W`](W) writer structure"]
impl crate::Writable for WPABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPABR to value 0"]
impl crate::Resettable for WPABRrs {
    const RESET_VALUE: u32 = 0;
}
