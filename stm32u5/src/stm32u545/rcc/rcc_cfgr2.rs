///Register `RCC_CFGR2` reader
pub type R = crate::R<RCC_CFGR2rs>;
///Register `RCC_CFGR2` writer
pub type W = crate::W<RCC_CFGR2rs>;
///Field `HPRE` reader - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPRE1` reader - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DPRE` reader - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type DPRE_R = crate::FieldReader;
///Field `DPRE` writer - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type DPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB1DIS` reader - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_R = crate::BitReader;
///Field `AHB1DIS` writer - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2DIS1` reader - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_R = crate::BitReader;
///Field `AHB2DIS1` writer - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2DIS2` reader - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
pub type AHB2DIS2_R = crate::BitReader;
///Field `AHB2DIS2` writer - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
pub type AHB2DIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1DIS` reader - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_R = crate::BitReader;
///Field `APB1DIS` writer - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2DIS` reader - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
pub type APB2DIS_R = crate::BitReader;
///Field `APB2DIS` writer - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
pub type APB2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn dpre(&self) -> DPRE_R {
        DPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    pub fn ahb1dis(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    pub fn ahb2dis1(&self) -> AHB2DIS1_R {
        AHB2DIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
    #[inline(always)]
    pub fn ahb2dis2(&self) -> AHB2DIS2_R {
        AHB2DIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    pub fn apb1dis(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
    #[inline(always)]
    pub fn apb2dis(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CFGR2")
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("dpre", &self.dpre())
            .field("ahb1dis", &self.ahb1dis())
            .field("ahb2dis1", &self.ahb2dis1())
            .field("ahb2dis2", &self.ahb2dis2())
            .field("apb1dis", &self.apb1dis())
            .field("apb2dis", &self.apb2dis())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AHB prescaler This bitfiled is set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Table�118). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<RCC_CFGR2rs> {
        HPRE_W::new(self, 0)
    }
    ///Bits 4:6 - APB1 prescaler This bitfiled is set and cleared by software to control the division factor of APB1 clock (PCLK1). 0xx: PCLK1 not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<RCC_CFGR2rs> {
        PPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - APB2 prescaler This bitfiled is set and cleared by software to control the division factor of APB2 clock (PCLK2). 0xx: PCLK2 not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<RCC_CFGR2rs> {
        PPRE2_W::new(self, 8)
    }
    ///Bits 12:14 - DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn dpre(&mut self) -> DPRE_W<RCC_CFGR2rs> {
        DPRE_W::new(self, 12)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    #[must_use]
    pub fn ahb1dis(&mut self) -> AHB1DIS_W<RCC_CFGR2rs> {
        AHB1DIS_W::new(self, 16)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    #[must_use]
    pub fn ahb2dis1(&mut self) -> AHB2DIS1_W<RCC_CFGR2rs> {
        AHB2DIS1_W::new(self, 17)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR2 are off.
    #[inline(always)]
    #[must_use]
    pub fn ahb2dis2(&mut self) -> AHB2DIS2_W<RCC_CFGR2rs> {
        AHB2DIS2_W::new(self, 18)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    #[must_use]
    pub fn apb1dis(&mut self) -> APB1DIS_W<RCC_CFGR2rs> {
        APB1DIS_W::new(self, 19)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all APB2 peripherals clocks are off.
    #[inline(always)]
    #[must_use]
    pub fn apb2dis(&mut self) -> APB2DIS_W<RCC_CFGR2rs> {
        APB2DIS_W::new(self, 20)
    }
}
/**RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:RCC_CFGR2)*/
pub struct RCC_CFGR2rs;
impl crate::RegisterSpec for RCC_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cfgr2::R`](R) reader structure
impl crate::Readable for RCC_CFGR2rs {}
///`write(|w| ..)` method takes [`rcc_cfgr2::W`](W) writer structure
impl crate::Writable for RCC_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CFGR2 to value 0x6000
impl crate::Resettable for RCC_CFGR2rs {
    const RESET_VALUE: u32 = 0x6000;
}
