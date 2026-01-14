///Register `APB1HLPENR` reader
pub type R = crate::R<APB1HLPENRrs>;
///Register `APB1HLPENR` writer
pub type W = crate::W<APB1HLPENRrs>;
/**UART9 clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<UART9LPEN> for bool {
    #[inline(always)]
    fn from(variant: UART9LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `UART9LPEN` reader - UART9 clock enable during Sleep mode
pub type UART9LPEN_R = crate::BitReader<UART9LPEN>;
impl UART9LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UART9LPEN {
        match self.bits {
            false => UART9LPEN::Disabled,
            true => UART9LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART9LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART9LPEN::Enabled
    }
}
///Field `UART9LPEN` writer - UART9 clock enable during Sleep mode
pub type UART9LPEN_W<'a, REG> = crate::BitWriter<'a, REG, UART9LPEN>;
impl<'a, REG> UART9LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9LPEN::Enabled)
    }
}
///Field `UART12LPEN` reader - UART12 clock enable during Sleep mode
pub use UART9LPEN_R as UART12LPEN_R;
///Field `DTSLPEN` reader - DTS clock enable during Sleep mode
pub use UART9LPEN_R as DTSLPEN_R;
///Field `LPTIM2LPEN` reader - LPTIM2 clock enable during Sleep mode
pub use UART9LPEN_R as LPTIM2LPEN_R;
///Field `FDCANLPEN` reader - FDCAN1 and FDCAN2 peripheral clock enable during Sleep mode
pub use UART9LPEN_R as FDCANLPEN_R;
///Field `UCPD1LPEN` reader - UCPD1 clock enable during Sleep mode
pub use UART9LPEN_R as UCPD1LPEN_R;
///Field `UART12LPEN` writer - UART12 clock enable during Sleep mode
pub use UART9LPEN_W as UART12LPEN_W;
///Field `DTSLPEN` writer - DTS clock enable during Sleep mode
pub use UART9LPEN_W as DTSLPEN_W;
///Field `LPTIM2LPEN` writer - LPTIM2 clock enable during Sleep mode
pub use UART9LPEN_W as LPTIM2LPEN_W;
///Field `FDCANLPEN` writer - FDCAN1 and FDCAN2 peripheral clock enable during Sleep mode
pub use UART9LPEN_W as FDCANLPEN_W;
///Field `UCPD1LPEN` writer - UCPD1 clock enable during Sleep mode
pub use UART9LPEN_W as UCPD1LPEN_W;
impl R {
    ///Bit 0 - UART9 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UART12 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart12lpen(&self) -> UART12LPEN_R {
        UART12LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - DTS clock enable during Sleep mode
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 23 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1lpen(&self) -> UCPD1LPEN_R {
        UCPD1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HLPENR")
            .field("uart9lpen", &self.uart9lpen())
            .field("uart12lpen", &self.uart12lpen())
            .field("dtslpen", &self.dtslpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .field("ucpd1lpen", &self.ucpd1lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - UART9 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<'_, APB1HLPENRrs> {
        UART9LPEN_W::new(self, 0)
    }
    ///Bit 1 - UART12 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart12lpen(&mut self) -> UART12LPEN_W<'_, APB1HLPENRrs> {
        UART12LPEN_W::new(self, 1)
    }
    ///Bit 3 - DTS clock enable during Sleep mode
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, APB1HLPENRrs> {
        DTSLPEN_W::new(self, 3)
    }
    ///Bit 5 - LPTIM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, APB1HLPENRrs> {
        LPTIM2LPEN_W::new(self, 5)
    }
    ///Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 9)
    }
    ///Bit 23 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1lpen(&mut self) -> UCPD1LPEN_W<'_, APB1HLPENRrs> {
        UCPD1LPEN_W::new(self, 23)
    }
}
/**RCC APB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:APB1HLPENR)*/
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
///`reset()` method sets APB1HLPENR to value 0x4080_0228
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0x4080_0228;
}
