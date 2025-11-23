///Register `ICR` writer
pub type W = crate::W<ICRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CMPMCF
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<'_, ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    ///Bit 1 - ARRMCF
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<'_, ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - EXTTRIGCF
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<'_, ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 3 - CMPOKCF
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<'_, ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    ///Bit 4 - ARROKCF
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<'_, ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - UPCF
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<'_, ICRrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - DOWNCF
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<'_, ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
/**LPTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
