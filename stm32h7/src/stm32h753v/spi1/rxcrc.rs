#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RXCRCrs>;
#[doc = "Register `RXCRC` writer"]
pub type W = crate::W<RXCRCrs>;
#[doc = "Field `RXCRC` reader - CRC register for receiver"]
pub type RXCRC_R = crate::FieldReader<u32>;
#[doc = "Field `RXCRC` writer - CRC register for receiver"]
pub type RXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrc(&mut self) -> RXCRC_W<RXCRCrs> {
        RXCRC_W::new(self, 0)
    }
}
#[doc = "Receiver CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCrs;
impl crate::RegisterSpec for RXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RXCRCrs {}
#[doc = "`write(|w| ..)` method takes [`rxcrc::W`](W) writer structure"]
impl crate::Writable for RXCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRCrs {
    const RESET_VALUE: u32 = 0;
}
