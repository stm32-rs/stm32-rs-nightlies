///Register `MICR` writer
pub type W = crate::W<MICRrs>;
/**Master Compare 1 Interrupt flag clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1C {
    ///1: Clears flag in MISR register
    Clear = 1,
}
impl From<MCMP1C> for bool {
    #[inline(always)]
    fn from(variant: MCMP1C) -> Self {
        variant as u8 != 0
    }
}
///Field `MCMP1C` writer - Master Compare 1 Interrupt flag clear
pub type MCMP1C_W<'a, REG> = crate::BitWriter<'a, REG, MCMP1C>;
impl<'a, REG> MCMP1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears flag in MISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MCMP1C::Clear)
    }
}
///Field `MCMP2C` writer - Master Compare 2 Interrupt flag clear
pub use MCMP1C_W as MCMP2C_W;
///Field `MCMP3C` writer - Master Compare 3 Interrupt flag clear
pub use MCMP1C_W as MCMP3C_W;
///Field `MCMP4C` writer - Master Compare 4 Interrupt flag clear
pub use MCMP1C_W as MCMP4C_W;
///Field `MREPC` writer - Repetition Interrupt flag clear
pub use MCMP1C_W as MREPC_W;
///Field `SYNCC` writer - Sync Input Interrupt flag clear
pub use MCMP1C_W as SYNCC_W;
///Field `MUPDC` writer - Master update Interrupt flag clear
pub use MCMP1C_W as MUPDC_W;
impl core::fmt::Debug for crate::generic::Reg<MICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master Compare 1 Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mcmp1c(&mut self) -> MCMP1C_W<MICRrs> {
        MCMP1C_W::new(self, 0)
    }
    ///Bit 1 - Master Compare 2 Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mcmp2c(&mut self) -> MCMP2C_W<MICRrs> {
        MCMP2C_W::new(self, 1)
    }
    ///Bit 2 - Master Compare 3 Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mcmp3c(&mut self) -> MCMP3C_W<MICRrs> {
        MCMP3C_W::new(self, 2)
    }
    ///Bit 3 - Master Compare 4 Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mcmp4c(&mut self) -> MCMP4C_W<MICRrs> {
        MCMP4C_W::new(self, 3)
    }
    ///Bit 4 - Repetition Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mrepc(&mut self) -> MREPC_W<MICRrs> {
        MREPC_W::new(self, 4)
    }
    ///Bit 5 - Sync Input Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn syncc(&mut self) -> SYNCC_W<MICRrs> {
        SYNCC_W::new(self, 5)
    }
    ///Bit 6 - Master update Interrupt flag clear
    #[inline(always)]
    #[must_use]
    pub fn mupdc(&mut self) -> MUPDC_W<MICRrs> {
        MUPDC_W::new(self, 6)
    }
}
/**Master Timer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`micr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Master:MICR)*/
pub struct MICRrs;
impl crate::RegisterSpec for MICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`micr::W`](W) writer structure
impl crate::Writable for MICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MICR to value 0
impl crate::Resettable for MICRrs {
    const RESET_VALUE: u32 = 0;
}
