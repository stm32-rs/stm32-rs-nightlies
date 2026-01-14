///Register `WUSCR` writer
pub type W = crate::W<WUSCRrs>;
/**clear wake-up pin flag for WUFx (x = 8 to 1)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1W {
    ///1: Writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)
    Clear = 1,
}
impl From<CWUF1W> for bool {
    #[inline(always)]
    fn from(variant: CWUF1W) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF1` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1W>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1W::Clear)
    }
}
///Field `CWUF2` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF2_W;
///Field `CWUF3` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF3_W;
///Field `CWUF4` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF4_W;
///Field `CWUF5` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF5_W;
///Field `CWUF6` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF6_W;
///Field `CWUF7` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF7_W;
///Field `CWUF8` writer - clear wake-up pin flag for WUFx (x = 8 to 1)
pub use CWUF1_W as CWUF8_W;
impl core::fmt::Debug for crate::generic::Reg<WUSCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<'_, WUSCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<'_, WUSCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<'_, WUSCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<'_, WUSCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<'_, WUSCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 5 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf6(&mut self) -> CWUF6_W<'_, WUSCRrs> {
        CWUF6_W::new(self, 5)
    }
    ///Bit 6 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf7(&mut self) -> CWUF7_W<'_, WUSCRrs> {
        CWUF7_W::new(self, 6)
    }
    ///Bit 7 - clear wake-up pin flag for WUFx (x = 8 to 1)
    #[inline(always)]
    pub fn cwuf8(&mut self) -> CWUF8_W<'_, WUSCRrs> {
        CWUF8_W::new(self, 7)
    }
}
/**PWR wake-up status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#PWR:WUSCR)*/
pub struct WUSCRrs;
impl crate::RegisterSpec for WUSCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wuscr::W`](W) writer structure
impl crate::Writable for WUSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUSCR to value 0
impl crate::Resettable for WUSCRrs {}
