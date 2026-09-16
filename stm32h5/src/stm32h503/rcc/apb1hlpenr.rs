///Register `APB1HLPENR` reader
pub type R = crate::R<APB1HLPENRrs>;
///Register `APB1HLPENR` writer
pub type W = crate::W<APB1HLPENRrs>;
/**DTS clock enable during sleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DTSLPEN> for bool {
    #[inline(always)]
    fn from(variant: DTSLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software.
pub type DTSLPEN_R = crate::BitReader<DTSLPEN>;
impl DTSLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTSLPEN {
        match self.bits {
            false => DTSLPEN::Disabled,
            true => DTSLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTSLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTSLPEN::Enabled
    }
}
///Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software.
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSLPEN>;
impl<'a, REG> DTSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN::Enabled)
    }
}
///Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software.
pub use DTSLPEN_R as LPTIM2LPEN_R;
///Field `FDCANLPEN` reader - FDCAN peripheral clock enable during sleep mode
pub use DTSLPEN_R as FDCANLPEN_R;
///Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software.
pub use DTSLPEN_W as LPTIM2LPEN_W;
///Field `FDCANLPEN` writer - FDCAN peripheral clock enable during sleep mode
pub use DTSLPEN_W as FDCANLPEN_W;
impl R {
    ///Bit 3 - DTS clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN peripheral clock enable during sleep mode
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HLPENR")
            .field("dtslpen", &self.dtslpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .finish()
    }
}
impl W {
    ///Bit 3 - DTS clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, APB1HLPENRrs> {
        DTSLPEN_W::new(self, 3)
    }
    ///Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, APB1HLPENRrs> {
        LPTIM2LPEN_W::new(self, 5)
    }
    ///Bit 9 - FDCAN peripheral clock enable during sleep mode
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 9)
    }
}
/**RCC APB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB1HLPENR)*/
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hlpenr::R`](R) reader structure
impl crate::Readable for APB1HLPENRrs {}
///`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HLPENR to value 0xffff_ffff
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
