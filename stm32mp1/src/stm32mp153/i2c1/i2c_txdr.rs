#[doc = "Register `I2C_TXDR` reader"]
pub type R = crate::R<I2C_TXDRrs>;
#[doc = "Register `I2C_TXDR` writer"]
pub type W = crate::W<I2C_TXDRrs>;
#[doc = "Field `TXDATA` reader - TXDATA"]
pub type TXDATA_R = crate::FieldReader;
#[doc = "Field `TXDATA` writer - TXDATA"]
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<I2C_TXDRrs> {
        TXDATA_W::new(self, 0)
    }
}
#[doc = "Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_txdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_TXDRrs;
impl crate::RegisterSpec for I2C_TXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_txdr::R`](R) reader structure"]
impl crate::Readable for I2C_TXDRrs {}
#[doc = "`write(|w| ..)` method takes [`i2c_txdr::W`](W) writer structure"]
impl crate::Writable for I2C_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TXDR to value 0"]
impl crate::Resettable for I2C_TXDRrs {
    const RESET_VALUE: u32 = 0;
}
