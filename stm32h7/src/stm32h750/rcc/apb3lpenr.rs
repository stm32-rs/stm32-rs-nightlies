///Register `APB3LPENR` reader
pub type R = crate::R<APB3LPENRrs>;
///Register `APB3LPENR` writer
pub type W = crate::W<APB3LPENRrs>;
/**LTDC peripheral clock enable during CSleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<LTDCLPEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode
pub type LTDCLPEN_R = crate::BitReader<LTDCLPEN>;
impl LTDCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCLPEN {
        match self.bits {
            false => LTDCLPEN::Disabled,
            true => LTDCLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCLPEN::Enabled
    }
}
///Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode
pub type LTDCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCLPEN>;
impl<'a, REG> LTDCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCLPEN::Enabled)
    }
}
///Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode
pub use LTDCLPEN_R as WWDG1LPEN_R;
///Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode
pub use LTDCLPEN_W as WWDG1LPEN_W;
impl R {
    ///Bit 3 - LTDC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - WWDG1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> WWDG1LPEN_R {
        WWDG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3LPENR")
            .field("ltdclpen", &self.ltdclpen())
            .field("wwdg1lpen", &self.wwdg1lpen())
            .finish()
    }
}
impl W {
    ///Bit 3 - LTDC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<APB3LPENRrs> {
        LTDCLPEN_W::new(self, 3)
    }
    ///Bit 6 - WWDG1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn wwdg1lpen(&mut self) -> WWDG1LPEN_W<APB3LPENRrs> {
        WWDG1LPEN_W::new(self, 6)
    }
}
/**RCC APB3 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:APB3LPENR)*/
pub struct APB3LPENRrs;
impl crate::RegisterSpec for APB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3lpenr::R`](R) reader structure
impl crate::Readable for APB3LPENRrs {}
///`write(|w| ..)` method takes [`apb3lpenr::W`](W) writer structure
impl crate::Writable for APB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3LPENR to value 0
impl crate::Resettable for APB3LPENRrs {}
