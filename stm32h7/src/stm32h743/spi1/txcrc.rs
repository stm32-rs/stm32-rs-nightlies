#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TXCRCrs>;
#[doc = "Register `TXCRC` writer"]
pub type W = crate::W<TXCRCrs>;
#[doc = "Field `TXCRC` reader - CRC register for transmitter"]
pub type TXCRC_R = crate::FieldReader<u32>;
#[doc = "Field `TXCRC` writer - CRC register for transmitter"]
pub type TXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn txcrc(&mut self) -> TXCRC_W<TXCRCrs> {
        TXCRC_W::new(self, 0)
    }
}
#[doc = "Transmitter CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRCrs;
impl crate::RegisterSpec for TXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TXCRCrs {}
#[doc = "`write(|w| ..)` method takes [`txcrc::W`](W) writer structure"]
impl crate::Writable for TXCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TXCRCrs {
    const RESET_VALUE: u32 = 0;
}
