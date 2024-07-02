///Register `RCC_AHB1SMENR` reader
pub type R = crate::R<RCC_AHB1SMENRrs>;
///Register `RCC_AHB1SMENR` writer
pub type W = crate::W<RCC_AHB1SMENRrs>;
///Field `GPDMA1SMEN` reader - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_R = crate::BitReader;
///Field `GPDMA1SMEN` writer - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICSMEN` reader - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode.
pub type CORDICSMEN_R = crate::BitReader;
///Field `CORDICSMEN` writer - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode.
pub type CORDICSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACSMEN` reader - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software.
pub type FMACSMEN_R = crate::BitReader;
///Field `FMACSMEN` writer - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software.
pub type FMACSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1SMEN` reader - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type MDF1SMEN_R = crate::BitReader;
///Field `MDF1SMEN` writer - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type MDF1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGSMEN` reader - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type JPEGSMEN_R = crate::BitReader;
///Field `JPEGSMEN` writer - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type JPEGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSMEN` reader - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TSCSMEN_R = crate::BitReader;
///Field `TSCSMEN` writer - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSMEN` reader - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type RAMCFGSMEN_R = crate::BitReader;
///Field `RAMCFGSMEN` writer - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type RAMCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DMA2DSMEN_R = crate::BitReader;
///Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DMA2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GFXMMUSMEN_R = crate::BitReader;
///Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GFXMMUSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DSMEN` reader - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GPU2DSMEN_R = crate::BitReader;
///Field `GPU2DSMEN` writer - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type GPU2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE2SMEN` reader - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DCACHE2SMEN_R = crate::BitReader;
///Field `DCACHE2SMEN` writer - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type DCACHE2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTZC1SMEN` reader - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GTZC1SMEN_R = crate::BitReader;
///Field `GTZC1SMEN` writer - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GTZC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMSMEN` reader - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software
pub type BKPSRAMSMEN_R = crate::BitReader;
///Field `BKPSRAMSMEN` writer - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software
pub type BKPSRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHESMEN` reader - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type ICACHESMEN_R = crate::BitReader;
///Field `ICACHESMEN` writer - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type ICACHESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE1SMEN` reader - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type DCACHE1SMEN_R = crate::BitReader;
///Field `DCACHE1SMEN` writer - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type DCACHE1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SMEN` reader - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SRAM1SMEN_R = crate::BitReader;
///Field `SRAM1SMEN` writer - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn gpdma1smen(&self) -> GPDMA1SMEN_R {
        GPDMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode.
    #[inline(always)]
    pub fn cordicsmen(&self) -> CORDICSMEN_R {
        CORDICSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software.
    #[inline(always)]
    pub fn fmacsmen(&self) -> FMACSMEN_R {
        FMACSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn mdf1smen(&self) -> MDF1SMEN_R {
        MDF1SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn jpegsmen(&self) -> JPEGSMEN_R {
        JPEGSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn ramcfgsmen(&self) -> RAMCFGSMEN_R {
        RAMCFGSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn gpu2dsmen(&self) -> GPU2DSMEN_R {
        GPU2DSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dcache2smen(&self) -> DCACHE2SMEN_R {
        DCACHE2SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc1smen(&self) -> GTZC1SMEN_R {
        GTZC1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software
    #[inline(always)]
    pub fn bkpsramsmen(&self) -> BKPSRAMSMEN_R {
        BKPSRAMSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn icachesmen(&self) -> ICACHESMEN_R {
        ICACHESMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn dcache1smen(&self) -> DCACHE1SMEN_R {
        DCACHE1SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB1SMENR")
            .field("gpdma1smen", &self.gpdma1smen())
            .field("cordicsmen", &self.cordicsmen())
            .field("fmacsmen", &self.fmacsmen())
            .field("mdf1smen", &self.mdf1smen())
            .field("flashsmen", &self.flashsmen())
            .field("crcsmen", &self.crcsmen())
            .field("jpegsmen", &self.jpegsmen())
            .field("tscsmen", &self.tscsmen())
            .field("ramcfgsmen", &self.ramcfgsmen())
            .field("dma2dsmen", &self.dma2dsmen())
            .field("gfxmmusmen", &self.gfxmmusmen())
            .field("gpu2dsmen", &self.gpu2dsmen())
            .field("dcache2smen", &self.dcache2smen())
            .field("gtzc1smen", &self.gtzc1smen())
            .field("bkpsramsmen", &self.bkpsramsmen())
            .field("icachesmen", &self.icachesmen())
            .field("dcache1smen", &self.dcache1smen())
            .field("sram1smen", &self.sram1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn gpdma1smen(&mut self) -> GPDMA1SMEN_W<RCC_AHB1SMENRrs> {
        GPDMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode.
    #[inline(always)]
    #[must_use]
    pub fn cordicsmen(&mut self) -> CORDICSMEN_W<RCC_AHB1SMENRrs> {
        CORDICSMEN_W::new(self, 1)
    }
    ///Bit 2 - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fmacsmen(&mut self) -> FMACSMEN_W<RCC_AHB1SMENRrs> {
        FMACSMEN_W::new(self, 2)
    }
    ///Bit 3 - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn mdf1smen(&mut self) -> MDF1SMEN_W<RCC_AHB1SMENRrs> {
        MDF1SMEN_W::new(self, 3)
    }
    ///Bit 8 - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<RCC_AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<RCC_AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 15 - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn jpegsmen(&mut self) -> JPEGSMEN_W<RCC_AHB1SMENRrs> {
        JPEGSMEN_W::new(self, 15)
    }
    ///Bit 16 - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<RCC_AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    ///Bit 17 - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsmen(&mut self) -> RAMCFGSMEN_W<RCC_AHB1SMENRrs> {
        RAMCFGSMEN_W::new(self, 17)
    }
    ///Bit 18 - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<RCC_AHB1SMENRrs> {
        DMA2DSMEN_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<RCC_AHB1SMENRrs> {
        GFXMMUSMEN_W::new(self, 19)
    }
    ///Bit 20 - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn gpu2dsmen(&mut self) -> GPU2DSMEN_W<RCC_AHB1SMENRrs> {
        GPU2DSMEN_W::new(self, 20)
    }
    ///Bit 21 - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn dcache2smen(&mut self) -> DCACHE2SMEN_W<RCC_AHB1SMENRrs> {
        DCACHE2SMEN_W::new(self, 21)
    }
    ///Bit 24 - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc1smen(&mut self) -> GTZC1SMEN_W<RCC_AHB1SMENRrs> {
        GTZC1SMEN_W::new(self, 24)
    }
    ///Bit 28 - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software
    #[inline(always)]
    #[must_use]
    pub fn bkpsramsmen(&mut self) -> BKPSRAMSMEN_W<RCC_AHB1SMENRrs> {
        BKPSRAMSMEN_W::new(self, 28)
    }
    ///Bit 29 - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn icachesmen(&mut self) -> ICACHESMEN_W<RCC_AHB1SMENRrs> {
        ICACHESMEN_W::new(self, 29)
    }
    ///Bit 30 - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dcache1smen(&mut self) -> DCACHE1SMEN_W<RCC_AHB1SMENRrs> {
        DCACHE1SMEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<RCC_AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 31)
    }
}
/**RCC AHB1 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB1SMENR)*/
pub struct RCC_AHB1SMENRrs;
impl crate::RegisterSpec for RCC_AHB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahb1smenr::R`](R) reader structure
impl crate::Readable for RCC_AHB1SMENRrs {}
///`write(|w| ..)` method takes [`rcc_ahb1smenr::W`](W) writer structure
impl crate::Writable for RCC_AHB1SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_AHB1SMENR to value 0xffff_ffff
impl crate::Resettable for RCC_AHB1SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
