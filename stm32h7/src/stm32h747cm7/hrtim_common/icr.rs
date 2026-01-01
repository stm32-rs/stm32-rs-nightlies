///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Fault 1 Interrupt Flag Clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1CW {
    ///1: Clears associated flag in ISR register
    Clear = 1,
}
impl From<FLT1CW> for bool {
    #[inline(always)]
    fn from(variant: FLT1CW) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1C` writer - Fault 1 Interrupt Flag Clear
pub type FLT1C_W<'a, REG> = crate::BitWriter1C<'a, REG, FLT1CW>;
impl<'a, REG> FLT1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears associated flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1CW::Clear)
    }
}
///Field `FLT2C` writer - Fault 2 Interrupt Flag Clear
pub use FLT1C_W as FLT2C_W;
///Field `FLT3C` writer - Fault 3 Interrupt Flag Clear
pub use FLT1C_W as FLT3C_W;
///Field `FLT4C` writer - Fault 4 Interrupt Flag Clear
pub use FLT1C_W as FLT4C_W;
///Field `FLT5C` writer - Fault 5 Interrupt Flag Clear
pub use FLT1C_W as FLT5C_W;
///Field `SYSFLTC` writer - System Fault Interrupt Flag Clear
pub use FLT1C_W as SYSFLTC_W;
///Field `BMPERC` writer - Burst mode period flag Clear
pub use FLT1C_W as BMPERC_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fault 1 Interrupt Flag Clear
    #[inline(always)]
    pub fn flt1c(&mut self) -> FLT1C_W<'_, ICRrs> {
        FLT1C_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 Interrupt Flag Clear
    #[inline(always)]
    pub fn flt2c(&mut self) -> FLT2C_W<'_, ICRrs> {
        FLT2C_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 Interrupt Flag Clear
    #[inline(always)]
    pub fn flt3c(&mut self) -> FLT3C_W<'_, ICRrs> {
        FLT3C_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 Interrupt Flag Clear
    #[inline(always)]
    pub fn flt4c(&mut self) -> FLT4C_W<'_, ICRrs> {
        FLT4C_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 Interrupt Flag Clear
    #[inline(always)]
    pub fn flt5c(&mut self) -> FLT5C_W<'_, ICRrs> {
        FLT5C_W::new(self, 4)
    }
    ///Bit 5 - System Fault Interrupt Flag Clear
    #[inline(always)]
    pub fn sysfltc(&mut self) -> SYSFLTC_W<'_, ICRrs> {
        SYSFLTC_W::new(self, 5)
    }
    ///Bit 17 - Burst mode period flag Clear
    #[inline(always)]
    pub fn bmperc(&mut self) -> BMPERC_W<'_, ICRrs> {
        BMPERC_W::new(self, 17)
    }
}
/**Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#HRTIM_Common:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0002_003f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
