#[doc = "Register `RCC_CFGR3` reader"]
pub type R = crate::R<RCC_CFGR3rs>;
#[doc = "Register `RCC_CFGR3` writer"]
pub type W = crate::W<RCC_CFGR3rs>;
#[doc = "Field `PPRE3` reader - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
pub type PPRE3_R = crate::FieldReader;
#[doc = "Field `PPRE3` writer - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
pub type PPRE3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AHB3DIS` reader - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
pub type AHB3DIS_R = crate::BitReader;
#[doc = "Field `AHB3DIS` writer - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
pub type AHB3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB3DIS` reader - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
pub type APB3DIS_R = crate::BitReader;
#[doc = "Field `APB3DIS` writer - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
pub type APB3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
    #[inline(always)]
    pub fn ahb3dis(&self) -> AHB3DIS_R {
        AHB3DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
    #[inline(always)]
    #[must_use]
    pub fn ppre3(&mut self) -> PPRE3_W<RCC_CFGR3rs> {
        PPRE3_W::new(self, 4)
    }
    #[doc = "Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
    #[inline(always)]
    #[must_use]
    pub fn ahb3dis(&mut self) -> AHB3DIS_W<RCC_CFGR3rs> {
        AHB3DIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
    #[inline(always)]
    #[must_use]
    pub fn apb3dis(&mut self) -> APB3DIS_W<RCC_CFGR3rs> {
        APB3DIS_W::new(self, 17)
    }
}
#[doc = "RCC clock configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_CFGR3rs;
impl crate::RegisterSpec for RCC_CFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cfgr3::R`](R) reader structure"]
impl crate::Readable for RCC_CFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_cfgr3::W`](W) writer structure"]
impl crate::Writable for RCC_CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CFGR3 to value 0"]
impl crate::Resettable for RCC_CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
