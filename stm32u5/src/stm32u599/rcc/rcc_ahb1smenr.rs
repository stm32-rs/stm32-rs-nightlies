#[doc = "Register `RCC_AHB1SMENR` reader"]
pub type R = crate::R<RCC_AHB1SMENRrs>;
#[doc = "Register `RCC_AHB1SMENR` writer"]
pub type W = crate::W<RCC_AHB1SMENRrs>;
#[doc = "Field `GPDMA1SMEN` reader - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type GPDMA1SMEN_R = crate::BitReader;
#[doc = "Field `GPDMA1SMEN` writer - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type GPDMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICSMEN` reader - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode."]
pub type CORDICSMEN_R = crate::BitReader;
#[doc = "Field `CORDICSMEN` writer - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode."]
pub type CORDICSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACSMEN` reader - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software."]
pub type FMACSMEN_R = crate::BitReader;
#[doc = "Field `FMACSMEN` writer - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software."]
pub type FMACSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDF1SMEN` reader - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type MDF1SMEN_R = crate::BitReader;
#[doc = "Field `MDF1SMEN` writer - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type MDF1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHSMEN` reader - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type FLASHSMEN_R = crate::BitReader;
#[doc = "Field `FLASHSMEN` writer - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type CRCSMEN_R = crate::BitReader;
#[doc = "Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEGSMEN` reader - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGSMEN_R = crate::BitReader;
#[doc = "Field `JPEGSMEN` writer - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSMEN` reader - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TSCSMEN_R = crate::BitReader;
#[doc = "Field `TSCSMEN` writer - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGSMEN` reader - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type RAMCFGSMEN_R = crate::BitReader;
#[doc = "Field `RAMCFGSMEN` writer - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type RAMCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DSMEN_R = crate::BitReader;
#[doc = "Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMUSMEN_R = crate::BitReader;
#[doc = "Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMUSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPU2DSMEN` reader - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DSMEN_R = crate::BitReader;
#[doc = "Field `GPU2DSMEN` writer - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE2SMEN` reader - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DCACHE2SMEN_R = crate::BitReader;
#[doc = "Field `DCACHE2SMEN` writer - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DCACHE2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTZC1SMEN` reader - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type GTZC1SMEN_R = crate::BitReader;
#[doc = "Field `GTZC1SMEN` writer - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type GTZC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMSMEN` reader - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software"]
pub type BKPSRAMSMEN_R = crate::BitReader;
#[doc = "Field `BKPSRAMSMEN` writer - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software"]
pub type BKPSRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHESMEN` reader - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type ICACHESMEN_R = crate::BitReader;
#[doc = "Field `ICACHESMEN` writer - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type ICACHESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE1SMEN` reader - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type DCACHE1SMEN_R = crate::BitReader;
#[doc = "Field `DCACHE1SMEN` writer - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type DCACHE1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SRAM1SMEN_R = crate::BitReader;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn gpdma1smen(&self) -> GPDMA1SMEN_R {
        GPDMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode."]
    #[inline(always)]
    pub fn cordicsmen(&self) -> CORDICSMEN_R {
        CORDICSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fmacsmen(&self) -> FMACSMEN_R {
        FMACSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn mdf1smen(&self) -> MDF1SMEN_R {
        MDF1SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn jpegsmen(&self) -> JPEGSMEN_R {
        JPEGSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ramcfgsmen(&self) -> RAMCFGSMEN_R {
        RAMCFGSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpu2dsmen(&self) -> GPU2DSMEN_R {
        GPU2DSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dcache2smen(&self) -> DCACHE2SMEN_R {
        DCACHE2SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gtzc1smen(&self) -> GTZC1SMEN_R {
        GTZC1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software"]
    #[inline(always)]
    pub fn bkpsramsmen(&self) -> BKPSRAMSMEN_R {
        BKPSRAMSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn icachesmen(&self) -> ICACHESMEN_R {
        ICACHESMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dcache1smen(&self) -> DCACHE1SMEN_R {
        DCACHE1SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1smen(&mut self) -> GPDMA1SMEN_W<RCC_AHB1SMENRrs> {
        GPDMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CORDIC clocks enable during Sleep and Stop modes This bit is set and cleared by software during Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn cordicsmen(&mut self) -> CORDICSMEN_W<RCC_AHB1SMENRrs> {
        CORDICSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - FMAC clocks enable during Sleep and Stop modes. This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacsmen(&mut self) -> FMACSMEN_W<RCC_AHB1SMENRrs> {
        FMACSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - MDF1 clocks enable during Sleep and Stop modes. This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn mdf1smen(&mut self) -> MDF1SMEN_W<RCC_AHB1SMENRrs> {
        MDF1SMEN_W::new(self, 3)
    }
    #[doc = "Bit 8 - FLASH clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<RCC_AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<RCC_AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn jpegsmen(&mut self) -> JPEGSMEN_W<RCC_AHB1SMENRrs> {
        JPEGSMEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - TSC clocks enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<RCC_AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsmen(&mut self) -> RAMCFGSMEN_W<RCC_AHB1SMENRrs> {
        RAMCFGSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<RCC_AHB1SMENRrs> {
        DMA2DSMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<RCC_AHB1SMENRrs> {
        GFXMMUSMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpu2dsmen(&mut self) -> GPU2DSMEN_W<RCC_AHB1SMENRrs> {
        GPU2DSMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dcache2smen(&mut self) -> DCACHE2SMEN_W<RCC_AHB1SMENRrs> {
        DCACHE2SMEN_W::new(self, 21)
    }
    #[doc = "Bit 24 - GTZC1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gtzc1smen(&mut self) -> GTZC1SMEN_W<RCC_AHB1SMENRrs> {
        GTZC1SMEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - BKPSRAM clock enable during Sleep and Stop modes This bit is set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramsmen(&mut self) -> BKPSRAMSMEN_W<RCC_AHB1SMENRrs> {
        BKPSRAMSMEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ICACHE clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn icachesmen(&mut self) -> ICACHESMEN_W<RCC_AHB1SMENRrs> {
        ICACHESMEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - DCACHE1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcache1smen(&mut self) -> DCACHE1SMEN_W<RCC_AHB1SMENRrs> {
        DCACHE1SMEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<RCC_AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 peripheral clock enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB1SMENRrs;
impl crate::RegisterSpec for RCC_AHB1SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1smenr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB1SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1smenr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB1SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1SMENR to value 0xffff_ffff"]
impl crate::Resettable for RCC_AHB1SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
