///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `FLT1C` writer - Fault 1 Interrupt Flag Clear
pub type FLT1C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2C` writer - Fault 2 Interrupt Flag Clear
pub type FLT2C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3C` writer - Fault 3 Interrupt Flag Clear
pub type FLT3C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4C` writer - Fault 4 Interrupt Flag Clear
pub type FLT4C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5C` writer - Fault 5 Interrupt Flag Clear
pub type FLT5C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSFLTC` writer - System Fault Interrupt Flag Clear
pub type SYSFLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT6C` writer - Fault 6 Interrupt Flag Clear
pub type FLT6C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLRDYC` writer - DLL Ready Interrupt flag Clear
pub type DLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMPERC` writer - Burst mode period flag Clear
pub type BMPERC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fault 1 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt1c(&mut self) -> FLT1C_W<ICRrs> {
        FLT1C_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt2c(&mut self) -> FLT2C_W<ICRrs> {
        FLT2C_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt3c(&mut self) -> FLT3C_W<ICRrs> {
        FLT3C_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt4c(&mut self) -> FLT4C_W<ICRrs> {
        FLT4C_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt5c(&mut self) -> FLT5C_W<ICRrs> {
        FLT5C_W::new(self, 4)
    }
    ///Bit 5 - System Fault Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn sysfltc(&mut self) -> SYSFLTC_W<ICRrs> {
        SYSFLTC_W::new(self, 5)
    }
    ///Bit 6 - Fault 6 Interrupt Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn flt6c(&mut self) -> FLT6C_W<ICRrs> {
        FLT6C_W::new(self, 6)
    }
    ///Bit 16 - DLL Ready Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn dllrdyc(&mut self) -> DLLRDYC_W<ICRrs> {
        DLLRDYC_W::new(self, 16)
    }
    ///Bit 17 - Burst mode period flag Clear
    #[inline(always)]
    #[must_use]
    pub fn bmperc(&mut self) -> BMPERC_W<ICRrs> {
        BMPERC_W::new(self, 17)
    }
}
/**Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Common:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
