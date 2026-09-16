///Register `APB5LPENR` reader
pub type R = crate::R<APB5LPENRrs>;
///Register `APB5LPENR` writer
pub type W = crate::W<APB5LPENRrs>;
/**LTDC peripheral clock enable in low-power mode

Value on reset: 1*/
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
///Field `LTDCLPEN` reader - LTDC peripheral clock enable in low-power mode
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
///Field `LTDCLPEN` writer - LTDC peripheral clock enable in low-power mode
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
///Field `DCMIPPLPEN` reader - DCMIPP peripheral clock enable in low-power mode
pub use LTDCLPEN_R as DCMIPPLPEN_R;
///Field `GFXTIMLPEN` reader - GFXTIM peripheral clock enable in low-power mode
pub use LTDCLPEN_R as GFXTIMLPEN_R;
///Field `DCMIPPLPEN` writer - DCMIPP peripheral clock enable in low-power mode
pub use LTDCLPEN_W as DCMIPPLPEN_W;
///Field `GFXTIMLPEN` writer - GFXTIM peripheral clock enable in low-power mode
pub use LTDCLPEN_W as GFXTIMLPEN_W;
impl R {
    ///Bit 1 - LTDC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn dcmipplpen(&self) -> DCMIPPLPEN_R {
        DCMIPPLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn gfxtimlpen(&self) -> GFXTIMLPEN_R {
        GFXTIMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5LPENR")
            .field("ltdclpen", &self.ltdclpen())
            .field("dcmipplpen", &self.dcmipplpen())
            .field("gfxtimlpen", &self.gfxtimlpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<'_, APB5LPENRrs> {
        LTDCLPEN_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn dcmipplpen(&mut self) -> DCMIPPLPEN_W<'_, APB5LPENRrs> {
        DCMIPPLPEN_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn gfxtimlpen(&mut self) -> GFXTIMLPEN_W<'_, APB5LPENRrs> {
        GFXTIMLPEN_W::new(self, 4)
    }
}
/**RCC APB5 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb5lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB5LPENR)*/
pub struct APB5LPENRrs;
impl crate::RegisterSpec for APB5LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5lpenr::R`](R) reader structure
impl crate::Readable for APB5LPENRrs {}
///`write(|w| ..)` method takes [`apb5lpenr::W`](W) writer structure
impl crate::Writable for APB5LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5LPENR to value 0x16
impl crate::Resettable for APB5LPENRrs {
    const RESET_VALUE: u32 = 0x16;
}
