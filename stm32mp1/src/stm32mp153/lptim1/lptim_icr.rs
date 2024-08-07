///Register `LPTIM_ICR` writer
pub type W = crate::W<LPTIM_ICRrs>;
///Field `CMPMCF` writer - CMPMCF
pub type CMPMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMCF` writer - ARRMCF
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGCF` writer - EXTTRIGCF
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPOKCF` writer - CMPOKCF
pub type CMPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKCF` writer - ARROKCF
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPCF` writer - UPCF
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNCF` writer - DOWNCF
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LPTIM_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CMPMCF
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<LPTIM_ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    ///Bit 1 - ARRMCF
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<LPTIM_ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - EXTTRIGCF
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<LPTIM_ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 3 - CMPOKCF
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<LPTIM_ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    ///Bit 4 - ARROKCF
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<LPTIM_ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - UPCF
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<LPTIM_ICRrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - DOWNCF
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<LPTIM_ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
/**LPTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LPTIM1:LPTIM_ICR)*/
pub struct LPTIM_ICRrs;
impl crate::RegisterSpec for LPTIM_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lptim_icr::W`](W) writer structure
impl crate::Writable for LPTIM_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM_ICR to value 0
impl crate::Resettable for LPTIM_ICRrs {
    const RESET_VALUE: u32 = 0;
}
