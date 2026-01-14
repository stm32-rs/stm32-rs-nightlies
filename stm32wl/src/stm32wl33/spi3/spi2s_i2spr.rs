///Register `SPI2S_I2SPR` reader
pub type R = crate::R<SPI2S_I2SPRrs>;
///Register `SPI2S_I2SPR` writer
pub type W = crate::W<SPI2S_I2SPRrs>;
///Field `I2SDIV` reader - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values.
pub type I2SDIV_R = crate::FieldReader;
///Field `I2SDIV` writer - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values.
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ODD` reader - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1
pub type ODD_R = crate::BitReader;
///Field `ODD` writer - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKOE` reader - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled
pub type MCKOE_R = crate::BitReader;
///Field `MCKOE` writer - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values.
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2S_I2SPR")
            .field("i2sdiv", &self.i2sdiv())
            .field("odd", &self.odd())
            .field("mckoe", &self.mckoe())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values.
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<'_, SPI2S_I2SPRrs> {
        I2SDIV_W::new(self, 0)
    }
    ///Bit 8 - Odd factor for the prescaler - 0: Real divider value is = I2SDIV *2 - 1: Real divider value is = (I2SDIV * 2)+1
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<'_, SPI2S_I2SPRrs> {
        ODD_W::new(self, 8)
    }
    ///Bit 9 - Master clock output enable - 0: Master clock output is disabled - 1: Master clock output is enabled
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<'_, SPI2S_I2SPRrs> {
        MCKOE_W::new(self, 9)
    }
}
/**SPI2S_I2SPR register

You can [`read`](crate::Reg::read) this register and get [`spi2s_i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI2S_I2SPR)*/
pub struct SPI2S_I2SPRrs;
impl crate::RegisterSpec for SPI2S_I2SPRrs {
    type Ux = u32;
}
///`read()` method returns [`spi2s_i2spr::R`](R) reader structure
impl crate::Readable for SPI2S_I2SPRrs {}
///`write(|w| ..)` method takes [`spi2s_i2spr::W`](W) writer structure
impl crate::Writable for SPI2S_I2SPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S_I2SPR to value 0x02
impl crate::Resettable for SPI2S_I2SPRrs {
    const RESET_VALUE: u32 = 0x02;
}
