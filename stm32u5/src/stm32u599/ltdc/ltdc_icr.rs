///Register `LTDC_ICR` writer
pub type W = crate::W<LTDC_ICRrs>;
///Field `CLIF` writer - clears the line interrupt flag
pub type CLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFUIF` writer - clears the FIFO underrun interrupt flag
pub type CFUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTERRIF` writer - clears the transfer error interrupt flag
pub type CTERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRRIF` writer - clears register reload interrupt flag
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LTDC_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clears the line interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<LTDC_ICRrs> {
        CLIF_W::new(self, 0)
    }
    ///Bit 1 - clears the FIFO underrun interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<LTDC_ICRrs> {
        CFUIF_W::new(self, 1)
    }
    ///Bit 2 - clears the transfer error interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<LTDC_ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    ///Bit 3 - clears register reload interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<LTDC_ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_ICR)*/
pub struct LTDC_ICRrs;
impl crate::RegisterSpec for LTDC_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ltdc_icr::W`](W) writer structure
impl crate::Writable for LTDC_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_ICR to value 0
impl crate::Resettable for LTDC_ICRrs {
    const RESET_VALUE: u32 = 0;
}
