///Register `AHB6RSTSETR` reader
pub type R = crate::R<AHB6RSTSETRrs>;
///Register `AHB6RSTSETR` writer
pub type W = crate::W<AHB6RSTSETRrs>;
///Field `GPURST` reader - GPURST
pub type GPURST_R = crate::BitReader;
///Field `GPURST` writer - GPURST
pub type GPURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACRST` reader - ETHMACRST
pub type ETHMACRST_R = crate::BitReader;
///Field `ETHMACRST` writer - ETHMACRST
pub type ETHMACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCRST` reader - FMCRST
pub type FMCRST_R = crate::BitReader;
///Field `FMCRST` writer - FMCRST
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QSPIRST` reader - QSPIRST
pub type QSPIRST_R = crate::BitReader;
///Field `QSPIRST` writer - QSPIRST
pub type QSPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1RST` reader - SDMMC1RST
pub type SDMMC1RST_R = crate::BitReader;
///Field `SDMMC1RST` writer - SDMMC1RST
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2RST` reader - SDMMC2RST
pub type SDMMC2RST_R = crate::BitReader;
///Field `SDMMC2RST` writer - SDMMC2RST
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC1RST` reader - CRC1RST
pub type CRC1RST_R = crate::BitReader;
///Field `CRC1RST` writer - CRC1RST
pub type CRC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBHRST` reader - USBHRST
pub type USBHRST_R = crate::BitReader;
///Field `USBHRST` writer - USBHRST
pub type USBHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - GPURST
    #[inline(always)]
    pub fn gpurst(&self) -> GPURST_R {
        GPURST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - ETHMACRST
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - FMCRST
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QSPIRST
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1RST
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDMMC2RST
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CRC1RST
    #[inline(always)]
    pub fn crc1rst(&self) -> CRC1RST_R {
        CRC1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - USBHRST
    #[inline(always)]
    pub fn usbhrst(&self) -> USBHRST_R {
        USBHRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB6RSTSETR")
            .field("gpurst", &self.gpurst())
            .field("ethmacrst", &self.ethmacrst())
            .field("fmcrst", &self.fmcrst())
            .field("qspirst", &self.qspirst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .field("crc1rst", &self.crc1rst())
            .field("usbhrst", &self.usbhrst())
            .finish()
    }
}
impl W {
    ///Bit 5 - GPURST
    #[inline(always)]
    pub fn gpurst(&mut self) -> GPURST_W<'_, AHB6RSTSETRrs> {
        GPURST_W::new(self, 5)
    }
    ///Bit 10 - ETHMACRST
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<'_, AHB6RSTSETRrs> {
        ETHMACRST_W::new(self, 10)
    }
    ///Bit 12 - FMCRST
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB6RSTSETRrs> {
        FMCRST_W::new(self, 12)
    }
    ///Bit 14 - QSPIRST
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W<'_, AHB6RSTSETRrs> {
        QSPIRST_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1RST
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB6RSTSETRrs> {
        SDMMC1RST_W::new(self, 16)
    }
    ///Bit 17 - SDMMC2RST
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<'_, AHB6RSTSETRrs> {
        SDMMC2RST_W::new(self, 17)
    }
    ///Bit 20 - CRC1RST
    #[inline(always)]
    pub fn crc1rst(&mut self) -> CRC1RST_W<'_, AHB6RSTSETRrs> {
        CRC1RST_W::new(self, 20)
    }
    ///Bit 24 - USBHRST
    #[inline(always)]
    pub fn usbhrst(&mut self) -> USBHRST_W<'_, AHB6RSTSETRrs> {
        USBHRST_W::new(self, 24)
    }
}
/**This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb6rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb6rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:AHB6RSTSETR)*/
pub struct AHB6RSTSETRrs;
impl crate::RegisterSpec for AHB6RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb6rstsetr::R`](R) reader structure
impl crate::Readable for AHB6RSTSETRrs {}
///`write(|w| ..)` method takes [`ahb6rstsetr::W`](W) writer structure
impl crate::Writable for AHB6RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB6RSTSETR to value 0
impl crate::Resettable for AHB6RSTSETRrs {}
