#[doc = "Register `RCC_AHB2ENR2` reader"]
pub type R = crate::R<RCC_AHB2ENR2rs>;
#[doc = "Register `RCC_AHB2ENR2` writer"]
pub type W = crate::W<RCC_AHB2ENR2rs>;
#[doc = "Field `FSMCEN` reader - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type FSMCEN_R = crate::BitReader;
#[doc = "Field `FSMCEN` writer - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type FSMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable This bit is set and cleared by software."]
pub type OCTOSPI1EN_R = crate::BitReader;
#[doc = "Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable This bit is set and cleared by software."]
pub type OCTOSPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI2EN` reader - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPI2EN_R = crate::BitReader;
#[doc = "Field `OCTOSPI2EN` writer - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPI1EN` reader - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type HSPI1EN_R = crate::BitReader;
#[doc = "Field `HSPI1EN` writer - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type HSPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM6EN` reader - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM6EN_R = crate::BitReader;
#[doc = "Field `SRAM6EN` writer - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM5EN` reader - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM5EN_R = crate::BitReader;
#[doc = "Field `SRAM5EN` writer - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - OCTOSPI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn hspi1en(&self) -> HSPI1EN_R {
        HSPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sram6en(&self) -> SRAM6EN_R {
        SRAM6EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sram5en(&self) -> SRAM5EN_R {
        SRAM5EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FSMC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<RCC_AHB2ENR2rs> {
        FSMCEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - OCTOSPI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<RCC_AHB2ENR2rs> {
        OCTOSPI1EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - OCTOSPI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<RCC_AHB2ENR2rs> {
        OCTOSPI2EN_W::new(self, 8)
    }
    #[doc = "Bit 12 - HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn hspi1en(&mut self) -> HSPI1EN_W<RCC_AHB2ENR2rs> {
        HSPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 30 - SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sram6en(&mut self) -> SRAM6EN_W<RCC_AHB2ENR2rs> {
        SRAM6EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sram5en(&mut self) -> SRAM5EN_W<RCC_AHB2ENR2rs> {
        SRAM5EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB2 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2enr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2enr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB2ENR2rs;
impl crate::RegisterSpec for RCC_AHB2ENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2enr2::R`](R) reader structure"]
impl crate::Readable for RCC_AHB2ENR2rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2enr2::W`](W) writer structure"]
impl crate::Writable for RCC_AHB2ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB2ENR2 to value 0x8000_0000"]
impl crate::Resettable for RCC_AHB2ENR2rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
