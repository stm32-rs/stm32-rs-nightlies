#[doc = "Register `TXDR` reader"]
pub type R = crate::R<TXDRrs>;
#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TXDRrs>;
#[doc = "Field `TXDATA` reader - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
pub type TXDATA_R = crate::FieldReader;
#[doc = "Field `TXDATA` writer - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<TXDRrs> {
        TXDATA_W::new(self, 0)
    }
}
#[doc = "I2C transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TXDRrs {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDRrs {
    const RESET_VALUE: u32 = 0;
}
