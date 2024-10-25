///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CLIF` writer - CLIF
pub type CLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFUIF` writer - CFUIF
pub type CFUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTERRIF` writer - CTERRIF
pub type CTERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRRIF` writer - CRRIF
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CLIF
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<ICRrs> {
        CLIF_W::new(self, 0)
    }
    ///Bit 1 - CFUIF
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<ICRrs> {
        CFUIF_W::new(self, 1)
    }
    ///Bit 2 - CTERRIF
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    ///Bit 3 - CRRIF
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
/**LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:ICR)*/
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
