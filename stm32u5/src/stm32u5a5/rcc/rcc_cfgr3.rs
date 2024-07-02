///Register `RCC_CFGR3` reader
pub type R = crate::R<RCC_CFGR3rs>;
///Register `RCC_CFGR3` writer
pub type W = crate::W<RCC_CFGR3rs>;
///Field `PPRE3` reader - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_R = crate::FieldReader;
///Field `PPRE3` writer - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB3DIS` reader - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_R = crate::BitReader;
///Field `AHB3DIS` writer - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3DIS` reader - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_R = crate::BitReader;
///Field `APB3DIS` writer - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    pub fn ahb3dis(&self) -> AHB3DIS_R {
        AHB3DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CFGR3")
            .field("ppre3", &self.ppre3())
            .field("ahb3dis", &self.ahb3dis())
            .field("apb3dis", &self.apb3dis())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - APB3 prescaler This bitfield is set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre3(&mut self) -> PPRE3_W<RCC_CFGR3rs> {
        PPRE3_W::new(self, 4)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    #[must_use]
    pub fn ahb3dis(&mut self) -> AHB3DIS_W<RCC_CFGR3rs> {
        AHB3DIS_W::new(self, 16)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    #[must_use]
    pub fn apb3dis(&mut self) -> APB3DIS_W<RCC_CFGR3rs> {
        APB3DIS_W::new(self, 17)
    }
}
/**RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:RCC_CFGR3)*/
pub struct RCC_CFGR3rs;
impl crate::RegisterSpec for RCC_CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cfgr3::R`](R) reader structure
impl crate::Readable for RCC_CFGR3rs {}
///`write(|w| ..)` method takes [`rcc_cfgr3::W`](W) writer structure
impl crate::Writable for RCC_CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CFGR3 to value 0
impl crate::Resettable for RCC_CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
