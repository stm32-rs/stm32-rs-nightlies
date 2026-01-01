///Register `WWDG_CR` reader
pub type R = crate::R<WWDG_CRrs>;
///Register `WWDG_CR` writer
pub type W = crate::W<WWDG_CRrs>;
///Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2<sup>WDGTB\[2:0\]</sup>) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
pub type T_R = crate::FieldReader;
///Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2<sup>WDGTB\[2:0\]</sup>) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGA {
    ///0: Watchdog disabled
    B0x0 = 0,
    ///1: Watchdog enabled
    B0x1 = 1,
}
impl From<WDGA> for bool {
    #[inline(always)]
    fn from(variant: WDGA) -> Self {
        variant as u8 != 0
    }
}
///Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.
pub type WDGA_R = crate::BitReader<WDGA>;
impl WDGA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDGA {
        match self.bits {
            false => WDGA::B0x0,
            true => WDGA::B0x1,
        }
    }
    ///Watchdog disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGA::B0x0
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGA::B0x1
    }
}
///Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG, WDGA>;
impl<'a, REG> WDGA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Watchdog disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA::B0x0)
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA::B0x1)
    }
}
impl R {
    ///Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2<sup>WDGTB\[2:0\]</sup>) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG_CR")
            .field("t", &self.t())
            .field("wdga", &self.wdga())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2<sup>WDGTB\[2:0\]</sup>) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
    #[inline(always)]
    pub fn t(&mut self) -> T_W<'_, WWDG_CRrs> {
        T_W::new(self, 0)
    }
    ///Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the watchdog can generate a reset.
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W<'_, WWDG_CRrs> {
        WDGA_W::new(self, 7)
    }
}
/**WWDG control register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#WWDG:WWDG_CR)*/
pub struct WWDG_CRrs;
impl crate::RegisterSpec for WWDG_CRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_cr::R`](R) reader structure
impl crate::Readable for WWDG_CRrs {}
///`write(|w| ..)` method takes [`wwdg_cr::W`](W) writer structure
impl crate::Writable for WWDG_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WWDG_CR to value 0x7f
impl crate::Resettable for WWDG_CRrs {
    const RESET_VALUE: u32 = 0x7f;
}
