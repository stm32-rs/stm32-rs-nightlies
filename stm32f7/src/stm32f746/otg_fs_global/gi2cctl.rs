///Register `GI2CCTL` reader
pub type R = crate::R<GI2CCTLrs>;
///Register `GI2CCTL` writer
pub type W = crate::W<GI2CCTLrs>;
///Field `RWDATA` reader - I2C Read/Write Data
pub type RWDATA_R = crate::FieldReader;
///Field `RWDATA` writer - I2C Read/Write Data
pub type RWDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `REGADDR` reader - I2C Register Address
pub type REGADDR_R = crate::FieldReader;
///Field `REGADDR` writer - I2C Register Address
pub type REGADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADDR` reader - I2C Address
pub type ADDR_R = crate::FieldReader;
///Field `ADDR` writer - I2C Address
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `I2CEN` reader - I2C Enable
pub type I2CEN_R = crate::BitReader;
///Field `I2CEN` writer - I2C Enable
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` reader - I2C ACK
pub type ACK_R = crate::BitReader;
///Field `ACK` writer - I2C ACK
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2CDEVADR` reader - I2C Device Address
pub type I2CDEVADR_R = crate::FieldReader;
///Field `I2CDEVADR` writer - I2C Device Address
pub type I2CDEVADR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2CDATSE0` reader - I2C DatSe0 USB mode
pub type I2CDATSE0_R = crate::BitReader;
///Field `I2CDATSE0` writer - I2C DatSe0 USB mode
pub type I2CDATSE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RW` reader - Read/Write Indicator
pub type RW_R = crate::BitReader;
///Field `RW` writer - Read/Write Indicator
pub type RW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSYDNE` reader - I2C Busy/Done
pub type BSYDNE_R = crate::BitReader;
///Field `BSYDNE` writer - I2C Busy/Done
pub type BSYDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - I2C Read/Write Data
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - I2C Register Address
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:22 - I2C Address
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - I2C Enable
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - I2C ACK
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 26:27 - I2C Device Address
    #[inline(always)]
    pub fn i2cdevadr(&self) -> I2CDEVADR_R {
        I2CDEVADR_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - I2C DatSe0 USB mode
    #[inline(always)]
    pub fn i2cdatse0(&self) -> I2CDATSE0_R {
        I2CDATSE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Read/Write Indicator
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - I2C Busy/Done
    #[inline(always)]
    pub fn bsydne(&self) -> BSYDNE_R {
        BSYDNE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GI2CCTL")
            .field("rwdata", &self.rwdata())
            .field("regaddr", &self.regaddr())
            .field("addr", &self.addr())
            .field("i2cen", &self.i2cen())
            .field("ack", &self.ack())
            .field("i2cdevadr", &self.i2cdevadr())
            .field("i2cdatse0", &self.i2cdatse0())
            .field("rw", &self.rw())
            .field("bsydne", &self.bsydne())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - I2C Read/Write Data
    #[inline(always)]
    pub fn rwdata(&mut self) -> RWDATA_W<'_, GI2CCTLrs> {
        RWDATA_W::new(self, 0)
    }
    ///Bits 8:15 - I2C Register Address
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W<'_, GI2CCTLrs> {
        REGADDR_W::new(self, 8)
    }
    ///Bits 16:22 - I2C Address
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, GI2CCTLrs> {
        ADDR_W::new(self, 16)
    }
    ///Bit 23 - I2C Enable
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W<'_, GI2CCTLrs> {
        I2CEN_W::new(self, 23)
    }
    ///Bit 24 - I2C ACK
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, GI2CCTLrs> {
        ACK_W::new(self, 24)
    }
    ///Bits 26:27 - I2C Device Address
    #[inline(always)]
    pub fn i2cdevadr(&mut self) -> I2CDEVADR_W<'_, GI2CCTLrs> {
        I2CDEVADR_W::new(self, 26)
    }
    ///Bit 28 - I2C DatSe0 USB mode
    #[inline(always)]
    pub fn i2cdatse0(&mut self) -> I2CDATSE0_W<'_, GI2CCTLrs> {
        I2CDATSE0_W::new(self, 28)
    }
    ///Bit 30 - Read/Write Indicator
    #[inline(always)]
    pub fn rw(&mut self) -> RW_W<'_, GI2CCTLrs> {
        RW_W::new(self, 30)
    }
    ///Bit 31 - I2C Busy/Done
    #[inline(always)]
    pub fn bsydne(&mut self) -> BSYDNE_W<'_, GI2CCTLrs> {
        BSYDNE_W::new(self, 31)
    }
}
/**OTG I2C access register

You can [`read`](crate::Reg::read) this register and get [`gi2cctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gi2cctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#OTG_FS_GLOBAL:GI2CCTL)*/
pub struct GI2CCTLrs;
impl crate::RegisterSpec for GI2CCTLrs {
    type Ux = u32;
}
///`read()` method returns [`gi2cctl::R`](R) reader structure
impl crate::Readable for GI2CCTLrs {}
///`write(|w| ..)` method takes [`gi2cctl::W`](W) writer structure
impl crate::Writable for GI2CCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GI2CCTL to value 0x0200_0400
impl crate::Resettable for GI2CCTLrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
