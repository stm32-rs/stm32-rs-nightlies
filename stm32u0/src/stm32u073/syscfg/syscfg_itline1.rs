///Register `SYSCFG_ITLINE1` reader
pub type R = crate::R<SYSCFG_ITLINE1rs>;
///Field `PVDOUT` reader - PVD supply monitoring interrupt request pending (EXTI line 16).
pub type PVDOUT_R = crate::BitReader;
///Field `PVMOUT1` reader - V<sub>DDUSB</sub> supply monitoring interrupt request pending (EXTI line 19)
pub type PVMOUT1_R = crate::BitReader;
///Field `PVMOUT3` reader - ADC supply monitoring interrupt request pending (EXTI line 20)
pub type PVMOUT3_R = crate::BitReader;
///Field `PVMOUT4` reader - DAC supply monitoring interrupt request pending (EXTI line 21)
pub type PVMOUT4_R = crate::BitReader;
impl R {
    ///Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16).
    #[inline(always)]
    pub fn pvdout(&self) -> PVDOUT_R {
        PVDOUT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - V<sub>DDUSB</sub> supply monitoring interrupt request pending (EXTI line 19)
    #[inline(always)]
    pub fn pvmout1(&self) -> PVMOUT1_R {
        PVMOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC supply monitoring interrupt request pending (EXTI line 20)
    #[inline(always)]
    pub fn pvmout3(&self) -> PVMOUT3_R {
        PVMOUT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DAC supply monitoring interrupt request pending (EXTI line 21)
    #[inline(always)]
    pub fn pvmout4(&self) -> PVMOUT4_R {
        PVMOUT4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE1")
            .field("pvdout", &self.pvdout())
            .field("pvmout1", &self.pvmout1())
            .field("pvmout3", &self.pvmout3())
            .field("pvmout4", &self.pvmout4())
            .finish()
    }
}
/**SYSCFG interrupt line 1 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:SYSCFG_ITLINE1)*/
pub struct SYSCFG_ITLINE1rs;
impl crate::RegisterSpec for SYSCFG_ITLINE1rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline1::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE1rs {}
///`reset()` method sets SYSCFG_ITLINE1 to value 0
impl crate::Resettable for SYSCFG_ITLINE1rs {
    const RESET_VALUE: u32 = 0;
}
