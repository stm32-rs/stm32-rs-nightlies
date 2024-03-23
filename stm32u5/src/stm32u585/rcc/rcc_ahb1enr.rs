#[doc = "Register `RCC_AHB1ENR` reader"]
pub type R = crate::R<RCC_AHB1ENRrs>;
#[doc = "Register `RCC_AHB1ENR` writer"]
pub type W = crate::W<RCC_AHB1ENRrs>;
#[doc = "Field `GPDMA1EN` reader - GPDMA1 clock enable This bit is set and cleared by software."]
pub type GPDMA1EN_R = crate::BitReader;
#[doc = "Field `GPDMA1EN` writer - GPDMA1 clock enable This bit is set and cleared by software."]
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICEN` reader - CORDIC clock enable This bit is set and cleared by software."]
pub type CORDICEN_R = crate::BitReader;
#[doc = "Field `CORDICEN` writer - CORDIC clock enable This bit is set and cleared by software."]
pub type CORDICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACEN` reader - FMAC clock enable This bit is set and reset by software."]
pub type FMACEN_R = crate::BitReader;
#[doc = "Field `FMACEN` writer - FMAC clock enable This bit is set and reset by software."]
pub type FMACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDF1EN` reader - MDF1 clock enable This bit is set and reset by software."]
pub type MDF1EN_R = crate::BitReader;
#[doc = "Field `MDF1EN` writer - MDF1 clock enable This bit is set and reset by software."]
pub type MDF1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode."]
pub type FLASHEN_R = crate::BitReader;
#[doc = "Field `FLASHEN` writer - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode."]
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable This bit is set and cleared by software."]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable This bit is set and cleared by software."]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEGEN` reader - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGEN_R = crate::BitReader;
#[doc = "Field `JPEGEN` writer - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCEN` reader - Touch sensing controller clock enable This bit is set and cleared by software."]
pub type TSCEN_R = crate::BitReader;
#[doc = "Field `TSCEN` writer - Touch sensing controller clock enable This bit is set and cleared by software."]
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGEN` reader - RAMCFG clock enable This bit is set and cleared by software."]
pub type RAMCFGEN_R = crate::BitReader;
#[doc = "Field `RAMCFGEN` writer - RAMCFG clock enable This bit is set and cleared by software."]
pub type RAMCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DEN` reader - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DEN_R = crate::BitReader;
#[doc = "Field `DMA2DEN` writer - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMUEN` reader - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMUEN_R = crate::BitReader;
#[doc = "Field `GFXMMUEN` writer - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPU2DEN` reader - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DEN_R = crate::BitReader;
#[doc = "Field `GPU2DEN` writer - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE2EN` reader - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DCACHE2EN_R = crate::BitReader;
#[doc = "Field `DCACHE2EN` writer - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DCACHE2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTZC1EN` reader - GTZC1 clock enable This bit is set and reset by software."]
pub type GTZC1EN_R = crate::BitReader;
#[doc = "Field `GTZC1EN` writer - GTZC1 clock enable This bit is set and reset by software."]
pub type GTZC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMEN` reader - BKPSRAM clock enable This bit is set and reset by software."]
pub type BKPSRAMEN_R = crate::BitReader;
#[doc = "Field `BKPSRAMEN` writer - BKPSRAM clock enable This bit is set and reset by software."]
pub type BKPSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE1EN` reader - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed."]
pub type DCACHE1EN_R = crate::BitReader;
#[doc = "Field `DCACHE1EN` writer - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed."]
pub type DCACHE1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1EN` reader - SRAM1 clock enable This bit is set and reset by software."]
pub type SRAM1EN_R = crate::BitReader;
#[doc = "Field `SRAM1EN` writer - SRAM1 clock enable This bit is set and reset by software."]
pub type SRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CORDIC clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FMAC clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDF1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn mdf1en(&self) -> MDF1EN_R {
        MDF1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode."]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn jpegen(&self) -> JPEGEN_R {
        JPEGEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch sensing controller clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpu2den(&self) -> GPU2DEN_R {
        GPU2DEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dcache2en(&self) -> DCACHE2EN_R {
        DCACHE2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - GTZC1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn gtzc1en(&self) -> GTZC1EN_R {
        GTZC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPSRAM clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed."]
    #[inline(always)]
    pub fn dcache1en(&self) -> DCACHE1EN_R {
        DCACHE1EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<RCC_AHB1ENRrs> {
        GPDMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CORDIC clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<RCC_AHB1ENRrs> {
        CORDICEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - FMAC clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<RCC_AHB1ENRrs> {
        FMACEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - MDF1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn mdf1en(&mut self) -> MDF1EN_W<RCC_AHB1ENRrs> {
        MDF1EN_W::new(self, 3)
    }
    #[doc = "Bit 8 - FLASH clock enable This bit is set and cleared by software. This bit can be disabled only when the flash memory is in power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<RCC_AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<RCC_AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn jpegen(&mut self) -> JPEGEN_W<RCC_AHB1ENRrs> {
        JPEGEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Touch sensing controller clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<RCC_AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - RAMCFG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<RCC_AHB1ENRrs> {
        RAMCFGEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<RCC_AHB1ENRrs> {
        DMA2DEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<RCC_AHB1ENRrs> {
        GFXMMUEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpu2den(&mut self) -> GPU2DEN_W<RCC_AHB1ENRrs> {
        GPU2DEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dcache2en(&mut self) -> DCACHE2EN_W<RCC_AHB1ENRrs> {
        DCACHE2EN_W::new(self, 21)
    }
    #[doc = "Bit 24 - GTZC1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gtzc1en(&mut self) -> GTZC1EN_W<RCC_AHB1ENRrs> {
        GTZC1EN_W::new(self, 24)
    }
    #[doc = "Bit 28 - BKPSRAM clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<RCC_AHB1ENRrs> {
        BKPSRAMEN_W::new(self, 28)
    }
    #[doc = "Bit 30 - DCACHE1 clock enable This bit is set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn dcache1en(&mut self) -> DCACHE1EN_W<RCC_AHB1ENRrs> {
        DCACHE1EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM1 clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<RCC_AHB1ENRrs> {
        SRAM1EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB1ENRrs;
impl crate::RegisterSpec for RCC_AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1enr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1enr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1ENR to value 0xd020_0100"]
impl crate::Resettable for RCC_AHB1ENRrs {
    const RESET_VALUE: u32 = 0xd020_0100;
}
