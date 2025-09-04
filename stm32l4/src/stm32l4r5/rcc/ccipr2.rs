///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `I2C4SEL` reader - I2C4 clock source selection
pub type I2C4SEL_R = crate::FieldReader;
///Field `I2C4SEL` writer - I2C4 clock source selection
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DFSDMSEL` reader - Digital filter for sigma delta modulator kernel clock source selection
pub type DFSDMSEL_R = crate::BitReader;
///Field `DFSDMSEL` writer - Digital filter for sigma delta modulator kernel clock source selection
pub type DFSDMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADFSDMSEL` reader - Digital filter for sigma delta modulator audio clock source selection
pub type ADFSDMSEL_R = crate::FieldReader;
///Field `ADFSDMSEL` writer - Digital filter for sigma delta modulator audio clock source selection
pub type ADFSDMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAI1SEL` reader - SAI1 clock source selection
pub type SAI1SEL_R = crate::FieldReader;
///Field `SAI1SEL` writer - SAI1 clock source selection
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAI2SEL` reader - SAI2 clock source selection
pub type SAI2SEL_R = crate::FieldReader;
///Field `SAI2SEL` writer - SAI2 clock source selection
pub type SAI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DSISEL` reader - clock selection
pub type DSISEL_R = crate::BitReader;
///Field `DSISEL` writer - clock selection
pub type DSISEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMCSEL` reader - SDMMC clock selection
pub type SDMMCSEL_R = crate::BitReader;
///Field `SDMMCSEL` writer - SDMMC clock selection
pub type SDMMCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2DIVR` reader - division factor for LTDC clock
pub type PLLSAI2DIVR_R = crate::FieldReader;
///Field `PLLSAI2DIVR` writer - division factor for LTDC clock
pub type PLLSAI2DIVR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPISEL` reader - Octospi clock source selection
pub type OSPISEL_R = crate::FieldReader;
///Field `OSPISEL` writer - Octospi clock source selection
pub type OSPISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Digital filter for sigma delta modulator kernel clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:7 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - clock selection
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - SDMMC clock selection
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - division factor for LTDC clock
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("i2c4sel", &self.i2c4sel())
            .field("dfsdmsel", &self.dfsdmsel())
            .field("adfsdmsel", &self.adfsdmsel())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .field("dsisel", &self.dsisel())
            .field("sdmmcsel", &self.sdmmcsel())
            .field("pllsai2divr", &self.pllsai2divr())
            .field("ospisel", &self.ospisel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<CCIPR2rs> {
        I2C4SEL_W::new(self, 0)
    }
    ///Bit 2 - Digital filter for sigma delta modulator kernel clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<CCIPR2rs> {
        DFSDMSEL_W::new(self, 2)
    }
    ///Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection
    #[inline(always)]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W<CCIPR2rs> {
        ADFSDMSEL_W::new(self, 3)
    }
    ///Bits 5:7 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<CCIPR2rs> {
        SAI1SEL_W::new(self, 5)
    }
    ///Bits 8:10 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<CCIPR2rs> {
        SAI2SEL_W::new(self, 8)
    }
    ///Bit 12 - clock selection
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W<CCIPR2rs> {
        DSISEL_W::new(self, 12)
    }
    ///Bit 14 - SDMMC clock selection
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<CCIPR2rs> {
        SDMMCSEL_W::new(self, 14)
    }
    ///Bits 16:17 - division factor for LTDC clock
    #[inline(always)]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W<CCIPR2rs> {
        PLLSAI2DIVR_W::new(self, 16)
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    pub fn ospisel(&mut self) -> OSPISEL_W<CCIPR2rs> {
        OSPISEL_W::new(self, 20)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
