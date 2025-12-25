///Register `APB3ENR` reader
pub type R = crate::R<APB3ENRrs>;
///Register `APB3ENR` writer
pub type W = crate::W<APB3ENRrs>;
/**LTDC peripheral clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCEN` reader - LTDC peripheral clock enable
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
///Field `LTDCEN` writer - LTDC peripheral clock enable
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
///Field `DSIEN` reader - DSI Peripheral clocks enable
pub use LTDCEN_R as DSIEN_R;
///Field `WWDG1EN` reader - WWDG1 Clock Enable
pub use LTDCEN_R as WWDG1EN_R;
///Field `DSIEN` writer - DSI Peripheral clocks enable
pub use LTDCEN_W as DSIEN_W;
///Field `WWDG1EN` writer - WWDG1 Clock Enable
pub use LTDCEN_W as WWDG1EN_W;
impl R {
    ///Bit 3 - LTDC peripheral clock enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DSI Peripheral clocks enable
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - WWDG1 Clock Enable
    #[inline(always)]
    pub fn wwdg1en(&self) -> WWDG1EN_R {
        WWDG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3ENR")
            .field("ltdcen", &self.ltdcen())
            .field("wwdg1en", &self.wwdg1en())
            .field("dsien", &self.dsien())
            .finish()
    }
}
impl W {
    ///Bit 3 - LTDC peripheral clock enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, APB3ENRrs> {
        LTDCEN_W::new(self, 3)
    }
    ///Bit 4 - DSI Peripheral clocks enable
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<'_, APB3ENRrs> {
        DSIEN_W::new(self, 4)
    }
    ///Bit 6 - WWDG1 Clock Enable
    #[inline(always)]
    pub fn wwdg1en(&mut self) -> WWDG1EN_W<'_, APB3ENRrs> {
        WWDG1EN_W::new(self, 6)
    }
}
/**RCC APB3 Clock Register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RCC:APB3ENR)*/
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3enr::R`](R) reader structure
impl crate::Readable for APB3ENRrs {}
///`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENRrs {}
