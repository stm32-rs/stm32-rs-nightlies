///Register `CFGR4` reader
pub type R = crate::R<CFGR4rs>;
///Register `CFGR4` writer
pub type W = crate::W<CFGR4rs>;
///Field `HPRE5` reader - AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided
pub type HPRE5_R = crate::FieldReader;
///Field `HPRE5` writer - AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided
pub type HPRE5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HDIV5` reader - AHB5 divider when SWS select HSI16 or HSE32 Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI16 or HSE32: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.
pub type HDIV5_R = crate::BitReader;
///Field `HDIV5` writer - AHB5 divider when SWS select HSI16 or HSE32 Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI16 or HSE32: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.
pub type HDIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided
    #[inline(always)]
    pub fn hpre5(&self) -> HPRE5_R {
        HPRE5_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - AHB5 divider when SWS select HSI16 or HSE32 Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI16 or HSE32: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.
    #[inline(always)]
    pub fn hdiv5(&self) -> HDIV5_R {
        HDIV5_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR4")
            .field("hpre5", &self.hpre5())
            .field("hdiv5", &self.hdiv5())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided
    #[inline(always)]
    pub fn hpre5(&mut self) -> HPRE5_W<'_, CFGR4rs> {
        HPRE5_W::new(self, 0)
    }
    ///Bit 4 - AHB5 divider when SWS select HSI16 or HSE32 Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI16 or HSE32: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.
    #[inline(always)]
    pub fn hdiv5(&mut self) -> HDIV5_W<'_, CFGR4rs> {
        HDIV5_W::new(self, 4)
    }
}
/**RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:CFGR4)*/
pub struct CFGR4rs;
impl crate::RegisterSpec for CFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr4::R`](R) reader structure
impl crate::Readable for CFGR4rs {}
///`write(|w| ..)` method takes [`cfgr4::W`](W) writer structure
impl crate::Writable for CFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR4 to value 0x10
impl crate::Resettable for CFGR4rs {
    const RESET_VALUE: u32 = 0x10;
}
