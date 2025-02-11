///Register `APB1HLPENR` reader
pub type R = crate::R<APB1HLPENRrs>;
///Register `APB1HLPENR` writer
pub type W = crate::W<APB1HLPENRrs>;
/**clock recovery system peripheral clock enable in low-power mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<CRSLPEN> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSLPEN` reader - clock recovery system peripheral clock enable in low-power mode Set and reset by software.
pub type CRSLPEN_R = crate::BitReader<CRSLPEN>;
impl CRSLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSLPEN {
        match self.bits {
            false => CRSLPEN::Disabled,
            true => CRSLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSLPEN::Enabled
    }
}
///Field `CRSLPEN` writer - clock recovery system peripheral clock enable in low-power mode Set and reset by software.
pub type CRSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSLPEN>;
impl<'a, REG> CRSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Enabled)
    }
}
///Field `MDIOSLPEN` reader - MDIOS peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_R as MDIOSLPEN_R;
///Field `FDCANLPEN` reader - FDCAN peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_R as FDCANLPEN_R;
///Field `UCPDLPEN` reader - UCPD peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_R as UCPDLPEN_R;
///Field `MDIOSLPEN` writer - MDIOS peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_W as MDIOSLPEN_W;
///Field `FDCANLPEN` writer - FDCAN peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_W as FDCANLPEN_W;
///Field `UCPDLPEN` writer - UCPD peripheral clock enable in low-power mode Set and reset by software.
pub use CRSLPEN_W as UCPDLPEN_W;
impl R {
    ///Bit 1 - clock recovery system peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn ucpdlpen(&self) -> UCPDLPEN_R {
        UCPDLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HLPENR")
            .field("crslpen", &self.crslpen())
            .field("mdioslpen", &self.mdioslpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .field("ucpdlpen", &self.ucpdlpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W<APB1HLPENRrs> {
        CRSLPEN_W::new(self, 1)
    }
    ///Bit 5 - MDIOS peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<APB1HLPENRrs> {
        MDIOSLPEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 8)
    }
    ///Bit 27 - UCPD peripheral clock enable in low-power mode Set and reset by software.
    #[inline(always)]
    pub fn ucpdlpen(&mut self) -> UCPDLPEN_W<APB1HLPENRrs> {
        UCPDLPEN_W::new(self, 27)
    }
}
/**RCC APB1 low-power clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1HLPENR)*/
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hlpenr::R`](R) reader structure
impl crate::Readable for APB1HLPENRrs {}
///`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB1HLPENR to value 0x0800_0122
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0x0800_0122;
}
