#[doc = "Register `I2C_CR2` reader"]
pub type R = crate::R<I2C_CR2rs>;
#[doc = "Register `I2C_CR2` writer"]
pub type W = crate::W<I2C_CR2rs>;
#[doc = "Field `SADD` reader - SADD"]
pub type SADD_R = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - SADD"]
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RD_WRN` reader - RD_WRN"]
pub type RD_WRN_R = crate::BitReader;
#[doc = "Field `RD_WRN` writer - RD_WRN"]
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10` reader - ADD10"]
pub type ADD10_R = crate::BitReader;
#[doc = "Field `ADD10` writer - ADD10"]
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD10R` reader - HEAD10R"]
pub type HEAD10R_R = crate::BitReader;
#[doc = "Field `HEAD10R` writer - HEAD10R"]
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - START"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - START"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - STOP"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - NACK"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBYTES` reader - NBYTES"]
pub type NBYTES_R = crate::FieldReader;
#[doc = "Field `NBYTES` writer - NBYTES"]
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RELOAD` reader - RELOAD"]
pub type RELOAD_R = crate::BitReader;
#[doc = "Field `RELOAD` writer - RELOAD"]
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOEND` reader - AUTOEND"]
pub type AUTOEND_R = crate::BitReader;
#[doc = "Field `AUTOEND` writer - AUTOEND"]
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECBYTE` reader - PECBYTE"]
pub type PECBYTE_R = crate::BitReader;
#[doc = "Field `PECBYTE` writer - PECBYTE"]
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - SADD"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - RD_WRN"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HEAD10R"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - NBYTES"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AUTOEND"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PECBYTE"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - SADD"]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<I2C_CR2rs> {
        SADD_W::new(self, 0)
    }
    #[doc = "Bit 10 - RD_WRN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<I2C_CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADD10"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<I2C_CR2rs> {
        ADD10_W::new(self, 11)
    }
    #[doc = "Bit 12 - HEAD10R"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<I2C_CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    #[doc = "Bit 13 - START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<I2C_CR2rs> {
        START_W::new(self, 13)
    }
    #[doc = "Bit 14 - STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<I2C_CR2rs> {
        STOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - NACK"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<I2C_CR2rs> {
        NACK_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - NBYTES"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<I2C_CR2rs> {
        NBYTES_W::new(self, 16)
    }
    #[doc = "Bit 24 - RELOAD"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<I2C_CR2rs> {
        RELOAD_W::new(self, 24)
    }
    #[doc = "Bit 25 - AUTOEND"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<I2C_CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    #[doc = "Bit 26 - PECBYTE"]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<I2C_CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CR2rs;
impl crate::RegisterSpec for I2C_CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr2::R`](R) reader structure"]
impl crate::Readable for I2C_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr2::W`](W) writer structure"]
impl crate::Writable for I2C_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CR2 to value 0"]
impl crate::Resettable for I2C_CR2rs {
    const RESET_VALUE: u32 = 0;
}
