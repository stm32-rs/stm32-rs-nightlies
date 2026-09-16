///Register `APB5ENR` reader
pub type R = crate::R<APB5ENRrs>;
///Register `APB5ENR` writer
pub type W = crate::W<APB5ENRrs>;
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
///Field `DCMIPPEN` reader - DCMIPP peripheral clock enable
pub use LTDCEN_R as DCMIPPEN_R;
///Field `GFXTIMEN` reader - GFXTIM peripheral clock enable
pub use LTDCEN_R as GFXTIMEN_R;
///Field `DCMIPPEN` writer - DCMIPP peripheral clock enable
pub use LTDCEN_W as DCMIPPEN_W;
///Field `GFXTIMEN` writer - GFXTIM peripheral clock enable
pub use LTDCEN_W as GFXTIMEN_W;
impl R {
    ///Bit 1 - LTDC peripheral clock enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP peripheral clock enable
    #[inline(always)]
    pub fn dcmippen(&self) -> DCMIPPEN_R {
        DCMIPPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM peripheral clock enable
    #[inline(always)]
    pub fn gfxtimen(&self) -> GFXTIMEN_R {
        GFXTIMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5ENR")
            .field("ltdcen", &self.ltdcen())
            .field("dcmippen", &self.dcmippen())
            .field("gfxtimen", &self.gfxtimen())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC peripheral clock enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, APB5ENRrs> {
        LTDCEN_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP peripheral clock enable
    #[inline(always)]
    pub fn dcmippen(&mut self) -> DCMIPPEN_W<'_, APB5ENRrs> {
        DCMIPPEN_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM peripheral clock enable
    #[inline(always)]
    pub fn gfxtimen(&mut self) -> GFXTIMEN_W<'_, APB5ENRrs> {
        GFXTIMEN_W::new(self, 4)
    }
}
/**RCC APB5 clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb5enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB5ENR)*/
pub struct APB5ENRrs;
impl crate::RegisterSpec for APB5ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5enr::R`](R) reader structure
impl crate::Readable for APB5ENRrs {}
///`write(|w| ..)` method takes [`apb5enr::W`](W) writer structure
impl crate::Writable for APB5ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5ENR to value 0
impl crate::Resettable for APB5ENRrs {}
