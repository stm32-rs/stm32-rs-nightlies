///Register `I2SPR` reader
pub type R = crate::R<I2SPRrs>;
///Register `I2SPR` writer
pub type W = crate::W<I2SPRrs>;
/**Field `I2SDIV` reader - I2S linear prescaler I2SDIV \[7:0\]
= 0 or I2SDIV \[7:0\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode.*/
pub type I2SDIV_R = crate::FieldReader;
/**Field `I2SDIV` writer - I2S linear prescaler I2SDIV \[7:0\]
= 0 or I2SDIV \[7:0\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode.*/
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ODD` reader - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
pub type ODD_R = crate::BitReader;
///Field `ODD` writer - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKOE` reader - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
pub type MCKOE_R = crate::BitReader;
///Field `MCKOE` writer - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\]
    = 0 or I2SDIV \[7:0\]
    = 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode.*/
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SPR")
            .field("i2sdiv", &self.i2sdiv())
            .field("odd", &self.odd())
            .field("mckoe", &self.mckoe())
            .finish()
    }
}
impl W {
    /**Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\]
    = 0 or I2SDIV \[7:0\]
    = 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode.*/
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<I2SPRrs> {
        I2SDIV_W::new(self, 0)
    }
    ///Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<I2SPRrs> {
        ODD_W::new(self, 8)
    }
    ///Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<I2SPRrs> {
        MCKOE_W::new(self, 9)
    }
}
/**SPI_I2S prescaler register

You can [`read`](crate::Reg::read) this register and get [`i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SPI1:I2SPR)*/
pub struct I2SPRrs;
impl crate::RegisterSpec for I2SPRrs {
    type Ux = u16;
}
///`read()` method returns [`i2spr::R`](R) reader structure
impl crate::Readable for I2SPRrs {}
///`write(|w| ..)` method takes [`i2spr::W`](W) writer structure
impl crate::Writable for I2SPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets I2SPR to value 0x02
impl crate::Resettable for I2SPRrs {
    const RESET_VALUE: u16 = 0x02;
}
