///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
pub type T_R = crate::FieldReader;
///Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGA {
    ///0: Watchdog disabled
    Disabled = 0,
    ///1: Watchdog enabled
    Enabled = 1,
}
impl From<WDGA> for bool {
    #[inline(always)]
    fn from(variant: WDGA) -> Self {
        variant as u8 != 0
    }
}
///Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.
pub type WDGA_R = crate::BitReader<WDGA>;
impl WDGA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDGA {
        match self.bits {
            false => WDGA::Disabled,
            true => WDGA::Enabled,
        }
    }
    ///Watchdog disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDGA::Disabled
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDGA::Enabled
    }
}
///Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG, WDGA>;
impl<'a, REG> WDGA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Watchdog disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA::Disabled)
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA::Enabled)
    }
}
impl R {
    ///Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("t", &self.t())
            .field("wdga", &self.wdga())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
    #[inline(always)]
    pub fn t(&mut self) -> T_W<CRrs> {
        T_W::new(self, 0)
    }
    ///Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W<CRrs> {
        WDGA_W::new(self, 7)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#WWDG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u16;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CR to value 0x7f
impl crate::Resettable for CRrs {
    const RESET_VALUE: u16 = 0x7f;
}