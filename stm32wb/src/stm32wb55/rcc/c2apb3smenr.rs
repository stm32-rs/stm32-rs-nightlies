///Register `C2APB3SMENR` reader
pub type R = crate::R<C2APB3SMENRrs>;
///Register `C2APB3SMENR` writer
pub type W = crate::W<C2APB3SMENRrs>;
/**BLE interface clocks enable during CPU2 Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLESMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<BLESMEN> for bool {
    #[inline(always)]
    fn from(variant: BLESMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BLESMEN` reader - BLE interface clocks enable during CPU2 Sleep mode
pub type BLESMEN_R = crate::BitReader<BLESMEN>;
impl BLESMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BLESMEN {
        match self.bits {
            false => BLESMEN::Disabled,
            true => BLESMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLESMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLESMEN::Enabled
    }
}
///Field `BLESMEN` writer - BLE interface clocks enable during CPU2 Sleep mode
pub type BLESMEN_W<'a, REG> = crate::BitWriter<'a, REG, BLESMEN>;
impl<'a, REG> BLESMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLESMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLESMEN::Enabled)
    }
}
///Field `SMEN802` reader - 802.15.4 interface clocks enable during CPU2 Sleep modes
pub use BLESMEN_R as SMEN802_R;
///Field `SMEN802` writer - 802.15.4 interface clocks enable during CPU2 Sleep modes
pub use BLESMEN_W as SMEN802_W;
impl R {
    ///Bit 0 - BLE interface clocks enable during CPU2 Sleep mode
    #[inline(always)]
    pub fn blesmen(&self) -> BLESMEN_R {
        BLESMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes
    #[inline(always)]
    pub fn smen802(&self) -> SMEN802_R {
        SMEN802_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB3SMENR")
            .field("blesmen", &self.blesmen())
            .field("smen802", &self.smen802())
            .finish()
    }
}
impl W {
    ///Bit 0 - BLE interface clocks enable during CPU2 Sleep mode
    #[inline(always)]
    pub fn blesmen(&mut self) -> BLESMEN_W<'_, C2APB3SMENRrs> {
        BLESMEN_W::new(self, 0)
    }
    ///Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes
    #[inline(always)]
    pub fn smen802(&mut self) -> SMEN802_W<'_, C2APB3SMENRrs> {
        SMEN802_W::new(self, 1)
    }
}
/**CPU2 APB3SMENR

You can [`read`](crate::Reg::read) this register and get [`c2apb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2APB3SMENR)*/
pub struct C2APB3SMENRrs;
impl crate::RegisterSpec for C2APB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb3smenr::R`](R) reader structure
impl crate::Readable for C2APB3SMENRrs {}
///`write(|w| ..)` method takes [`c2apb3smenr::W`](W) writer structure
impl crate::Writable for C2APB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB3SMENR to value 0x03
impl crate::Resettable for C2APB3SMENRrs {
    const RESET_VALUE: u32 = 0x03;
}
