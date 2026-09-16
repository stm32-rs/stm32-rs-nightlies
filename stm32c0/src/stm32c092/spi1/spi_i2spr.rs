///Register `SPI_I2SPR` reader
pub type R = crate::R<SPI_I2SPRrs>;
///Register `SPI_I2SPR` writer
pub type W = crate::W<SPI_I2SPRrs>;
///Field `I2SDIV` reader - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values. Refer to Section 27.7.3 on page 812. Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. Note: They are not used in SPI mode.
pub type I2SDIV_R = crate::FieldReader;
///Field `I2SDIV` writer - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values. Refer to Section 27.7.3 on page 812. Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. Note: They are not used in SPI mode.
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Odd factor for the prescaler Refer to Section 27.7.3 on page 812. Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD {
    ///0: Real divider value is = I2SDIV *2
    B0x0 = 0,
    ///1: Real divider value is = (I2SDIV * 2) + 1
    B0x1 = 1,
}
impl From<ODD> for bool {
    #[inline(always)]
    fn from(variant: ODD) -> Self {
        variant as u8 != 0
    }
}
///Field `ODD` reader - Odd factor for the prescaler Refer to Section 27.7.3 on page 812. Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
pub type ODD_R = crate::BitReader<ODD>;
impl ODD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODD {
        match self.bits {
            false => ODD::B0x0,
            true => ODD::B0x1,
        }
    }
    ///Real divider value is = I2SDIV *2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ODD::B0x0
    }
    ///Real divider value is = (I2SDIV * 2) + 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ODD::B0x1
    }
}
///Field `ODD` writer - Odd factor for the prescaler Refer to Section 27.7.3 on page 812. Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Real divider value is = I2SDIV *2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::B0x0)
    }
    ///Real divider value is = (I2SDIV * 2) + 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ODD::B0x1)
    }
}
/**Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE {
    ///0: Master clock output is disabled
    B0x0 = 0,
    ///1: Master clock output is enabled
    B0x1 = 1,
}
impl From<MCKOE> for bool {
    #[inline(always)]
    fn from(variant: MCKOE) -> Self {
        variant as u8 != 0
    }
}
///Field `MCKOE` reader - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
pub type MCKOE_R = crate::BitReader<MCKOE>;
impl MCKOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE {
        match self.bits {
            false => MCKOE::B0x0,
            true => MCKOE::B0x1,
        }
    }
    ///Master clock output is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCKOE::B0x0
    }
    ///Master clock output is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCKOE::B0x1
    }
}
///Field `MCKOE` writer - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master clock output is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::B0x0)
    }
    ///Master clock output is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE::B0x1)
    }
}
impl R {
    ///Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values. Refer to Section 27.7.3 on page 812. Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Odd factor for the prescaler Refer to Section 27.7.3 on page 812. Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_I2SPR")
            .field("i2sdiv", &self.i2sdiv())
            .field("odd", &self.odd())
            .field("mckoe", &self.mckoe())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values. Refer to Section 27.7.3 on page 812. Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<'_, SPI_I2SPRrs> {
        I2SDIV_W::new(self, 0)
    }
    ///Bit 8 - Odd factor for the prescaler Refer to Section 27.7.3 on page 812. Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<'_, SPI_I2SPRrs> {
        ODD_W::new(self, 8)
    }
    ///Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<'_, SPI_I2SPRrs> {
        MCKOE_W::new(self, 9)
    }
}
/**SPIx_I2S prescaler register

You can [`read`](crate::Reg::read) this register and get [`spi_i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SPI1:SPI_I2SPR)*/
pub struct SPI_I2SPRrs;
impl crate::RegisterSpec for SPI_I2SPRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_i2spr::R`](R) reader structure
impl crate::Readable for SPI_I2SPRrs {}
///`write(|w| ..)` method takes [`spi_i2spr::W`](W) writer structure
impl crate::Writable for SPI_I2SPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_I2SPR to value 0
impl crate::Resettable for SPI_I2SPRrs {}
