#[doc = "Register `RCC_AHB1RSTR` reader"]
pub type R = crate::R<RCC_AHB1RSTRrs>;
#[doc = "Register `RCC_AHB1RSTR` writer"]
pub type W = crate::W<RCC_AHB1RSTRrs>;
#[doc = "Field `GPDMA1RST` reader - GPDMA1 reset This bit is set and cleared by software."]
pub type GPDMA1RST_R = crate::BitReader;
#[doc = "Field `GPDMA1RST` writer - GPDMA1 reset This bit is set and cleared by software."]
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICRST` reader - CORDIC reset This bit is set and cleared by software."]
pub type CORDICRST_R = crate::BitReader;
#[doc = "Field `CORDICRST` writer - CORDIC reset This bit is set and cleared by software."]
pub type CORDICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACRST` reader - FMAC reset This bit is set and cleared by software."]
pub type FMACRST_R = crate::BitReader;
#[doc = "Field `FMACRST` writer - FMAC reset This bit is set and cleared by software."]
pub type FMACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDF1RST` reader - MDF1 reset This bit is set and cleared by software."]
pub type MDF1RST_R = crate::BitReader;
#[doc = "Field `MDF1RST` writer - MDF1 reset This bit is set and cleared by software."]
pub type MDF1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset This bit is set and cleared by software."]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset This bit is set and cleared by software."]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEGRST` reader - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGRST_R = crate::BitReader;
#[doc = "Field `JPEGRST` writer - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type JPEGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCRST` reader - TSC reset This bit is set and cleared by software."]
pub type TSCRST_R = crate::BitReader;
#[doc = "Field `TSCRST` writer - TSC reset This bit is set and cleared by software."]
pub type TSCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGRST` reader - RAMCFG reset This bit is set and cleared by software."]
pub type RAMCFGRST_R = crate::BitReader;
#[doc = "Field `RAMCFGRST` writer - RAMCFG reset This bit is set and cleared by software."]
pub type RAMCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DRST` reader - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DRST_R = crate::BitReader;
#[doc = "Field `DMA2DRST` writer - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DMA2DRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMURST` reader - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMURST_R = crate::BitReader;
#[doc = "Field `GFXMMURST` writer - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXMMURST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPU2DRST` reader - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DRST_R = crate::BitReader;
#[doc = "Field `GPU2DRST` writer - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPU2DRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPDMA1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CORDIC reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FMAC reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDF1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn mdf1rst(&self) -> MDF1RST_R {
        MDF1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn jpegrst(&self) -> JPEGRST_R {
        JPEGRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TSC reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpu2drst(&self) -> GPU2DRST_R {
        GPU2DRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<RCC_AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - CORDIC reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CORDICRST_W<RCC_AHB1RSTRrs> {
        CORDICRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - FMAC reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FMACRST_W<RCC_AHB1RSTRrs> {
        FMACRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - MDF1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mdf1rst(&mut self) -> MDF1RST_W<RCC_AHB1RSTRrs> {
        MDF1RST_W::new(self, 3)
    }
    #[doc = "Bit 12 - CRC reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<RCC_AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 15 - JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn jpegrst(&mut self) -> JPEGRST_W<RCC_AHB1RSTRrs> {
        JPEGRST_W::new(self, 15)
    }
    #[doc = "Bit 16 - TSC reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<RCC_AHB1RSTRrs> {
        TSCRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - RAMCFG reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<RCC_AHB1RSTRrs> {
        RAMCFGRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<RCC_AHB1RSTRrs> {
        DMA2DRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<RCC_AHB1RSTRrs> {
        GFXMMURST_W::new(self, 19)
    }
    #[doc = "Bit 20 - GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpu2drst(&mut self) -> GPU2DRST_W<RCC_AHB1RSTRrs> {
        GPU2DRST_W::new(self, 20)
    }
}
#[doc = "RCC AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB1RSTRrs;
impl crate::RegisterSpec for RCC_AHB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1rstr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1rstr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1RSTR to value 0"]
impl crate::Resettable for RCC_AHB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
