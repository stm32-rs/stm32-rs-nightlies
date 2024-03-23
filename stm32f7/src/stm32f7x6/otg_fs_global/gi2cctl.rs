#[doc = "Register `GI2CCTL` reader"]
pub type R = crate::R<GI2CCTLrs>;
#[doc = "Register `GI2CCTL` writer"]
pub type W = crate::W<GI2CCTLrs>;
#[doc = "Field `RWDATA` reader - I2C Read/Write Data"]
pub type RWDATA_R = crate::FieldReader;
#[doc = "Field `RWDATA` writer - I2C Read/Write Data"]
pub type RWDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGADDR` reader - I2C Register Address"]
pub type REGADDR_R = crate::FieldReader;
#[doc = "Field `REGADDR` writer - I2C Register Address"]
pub type REGADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR` reader - I2C Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - I2C Address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - I2C ACK"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - I2C ACK"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CDEVADR` reader - I2C Device Address"]
pub type I2CDEVADR_R = crate::FieldReader;
#[doc = "Field `I2CDEVADR` writer - I2C Device Address"]
pub type I2CDEVADR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDATSE0` reader - I2C DatSe0 USB mode"]
pub type I2CDATSE0_R = crate::BitReader;
#[doc = "Field `I2CDATSE0` writer - I2C DatSe0 USB mode"]
pub type I2CDATSE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW` reader - Read/Write Indicator"]
pub type RW_R = crate::BitReader;
#[doc = "Field `RW` writer - Read/Write Indicator"]
pub type RW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSYDNE` reader - I2C Busy/Done"]
pub type BSYDNE_R = crate::BitReader;
#[doc = "Field `BSYDNE` writer - I2C Busy/Done"]
pub type BSYDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&self) -> I2CDEVADR_R {
        I2CDEVADR_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&self) -> I2CDATSE0_R {
        I2CDATSE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&self) -> BSYDNE_R {
        BSYDNE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn rwdata(&mut self) -> RWDATA_W<GI2CCTLrs> {
        RWDATA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn regaddr(&mut self) -> REGADDR_W<GI2CCTLrs> {
        REGADDR_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<GI2CCTLrs> {
        ADDR_W::new(self, 16)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<GI2CCTLrs> {
        I2CEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<GI2CCTLrs> {
        ACK_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdevadr(&mut self) -> I2CDEVADR_W<GI2CCTLrs> {
        I2CDEVADR_W::new(self, 26)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdatse0(&mut self) -> I2CDATSE0_W<GI2CCTLrs> {
        I2CDATSE0_W::new(self, 28)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rw(&mut self) -> RW_W<GI2CCTLrs> {
        RW_W::new(self, 30)
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    #[must_use]
    pub fn bsydne(&mut self) -> BSYDNE_W<GI2CCTLrs> {
        BSYDNE_W::new(self, 31)
    }
}
#[doc = "OTG I2C access register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gi2cctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gi2cctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GI2CCTLrs;
impl crate::RegisterSpec for GI2CCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gi2cctl::R`](R) reader structure"]
impl crate::Readable for GI2CCTLrs {}
#[doc = "`write(|w| ..)` method takes [`gi2cctl::W`](W) writer structure"]
impl crate::Writable for GI2CCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GI2CCTL to value 0x0200_0400"]
impl crate::Resettable for GI2CCTLrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
