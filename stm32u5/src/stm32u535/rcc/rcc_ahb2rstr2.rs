#[doc = "Register `RCC_AHB2RSTR2` reader"]
pub type R = crate::R<RCC_AHB2RSTR2rs>;
#[doc = "Register `RCC_AHB2RSTR2` writer"]
pub type W = crate::W<RCC_AHB2RSTR2rs>;
#[doc = "Field `FSMCRST` reader - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type FSMCRST_R = crate::BitReader;
#[doc = "Field `FSMCRST` writer - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type FSMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1RST` reader - OCTOSPI1 reset This bit is set and cleared by software."]
pub type OCTOSPI1RST_R = crate::BitReader;
#[doc = "Field `OCTOSPI1RST` writer - OCTOSPI1 reset This bit is set and cleared by software."]
pub type OCTOSPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI2RST` reader - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPI2RST_R = crate::BitReader;
#[doc = "Field `OCTOSPI2RST` writer - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPI1RST` reader - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type HSPI1RST_R = crate::BitReader;
#[doc = "Field `HSPI1RST` writer - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type HSPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - OCTOSPI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn octospi2rst(&self) -> OCTOSPI2RST_R {
        OCTOSPI2RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn hspi1rst(&self) -> HSPI1RST_R {
        HSPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<RCC_AHB2RSTR2rs> {
        FSMCRST_W::new(self, 0)
    }
    #[doc = "Bit 4 - OCTOSPI1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<RCC_AHB2RSTR2rs> {
        OCTOSPI1RST_W::new(self, 4)
    }
    #[doc = "Bit 8 - OCTOSPI2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn octospi2rst(&mut self) -> OCTOSPI2RST_W<RCC_AHB2RSTR2rs> {
        OCTOSPI2RST_W::new(self, 8)
    }
    #[doc = "Bit 12 - HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn hspi1rst(&mut self) -> HSPI1RST_W<RCC_AHB2RSTR2rs> {
        HSPI1RST_W::new(self, 12)
    }
}
#[doc = "RCC AHB2 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB2RSTR2rs;
impl crate::RegisterSpec for RCC_AHB2RSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2rstr2::R`](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2rstr2::W`](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB2RSTR2 to value 0"]
impl crate::Resettable for RCC_AHB2RSTR2rs {
    const RESET_VALUE: u32 = 0;
}
