#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `SADD` reader - Slave address bit (master mode)"]
pub type SADD_R = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address bit (master mode)"]
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub type RD_WRN_R = crate::BitReader;
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub type ADD10_R = crate::BitReader;
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_R = crate::BitReader;
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NBYTES_R = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub type RELOAD_R = crate::BitReader;
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub type AUTOEND_R = crate::BitReader;
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub type PECBYTE_R = crate::BitReader;
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<CR2rs> {
        SADD_W::new(self, 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<CR2rs> {
        ADD10_W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR2rs> {
        START_W::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CR2rs> {
        NACK_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<CR2rs> {
        NBYTES_W::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<CR2rs> {
        RELOAD_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
