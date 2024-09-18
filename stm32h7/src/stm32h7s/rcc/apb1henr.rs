///Register `APB1HENR` reader
pub type R = crate::R<APB1HENRrs>;
///Register `APB1HENR` writer
pub type W = crate::W<APB1HENRrs>;
/**clock recovery system peripheral clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<CRSEN> for bool {
    #[inline(always)]
    fn from(variant: CRSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSEN` reader - clock recovery system peripheral clock enable Set and reset by software.
pub type CRSEN_R = crate::BitReader<CRSEN>;
impl CRSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSEN {
        match self.bits {
            false => CRSEN::Disabled,
            true => CRSEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSEN::Enabled
    }
}
///Field `CRSEN` writer - clock recovery system peripheral clock enable Set and reset by software.
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSEN>;
impl<'a, REG> CRSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::Enabled)
    }
}
///Field `MDIOSEN` reader - MDIOS peripheral clock enable Set and reset by software.
pub use CRSEN_R as MDIOSEN_R;
///Field `FDCANEN` reader - FDCAN peripheral clock enable Set and reset by software.
pub use CRSEN_R as FDCANEN_R;
///Field `UCPDEN` reader - UCPD peripheral clock enable Set and reset by software.
pub use CRSEN_R as UCPDEN_R;
///Field `MDIOSEN` writer - MDIOS peripheral clock enable Set and reset by software.
pub use CRSEN_W as MDIOSEN_W;
///Field `FDCANEN` writer - FDCAN peripheral clock enable Set and reset by software.
pub use CRSEN_W as FDCANEN_W;
///Field `UCPDEN` writer - UCPD peripheral clock enable Set and reset by software.
pub use CRSEN_W as UCPDEN_W;
impl R {
    ///Bit 1 - clock recovery system peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HENR")
            .field("crsen", &self.crsen())
            .field("mdiosen", &self.mdiosen())
            .field("fdcanen", &self.fdcanen())
            .field("ucpden", &self.ucpden())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APB1HENRrs> {
        CRSEN_W::new(self, 1)
    }
    ///Bit 5 - MDIOS peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<APB1HENRrs> {
        MDIOSEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<APB1HENRrs> {
        FDCANEN_W::new(self, 8)
    }
    ///Bit 27 - UCPD peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<APB1HENRrs> {
        UCPDEN_W::new(self, 27)
    }
}
/**RCC APB1 clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB1HENR)*/
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1henr::R`](R) reader structure
impl crate::Readable for APB1HENRrs {}
///`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB1HENR to value 0
impl crate::Resettable for APB1HENRrs {
    const RESET_VALUE: u32 = 0;
}
