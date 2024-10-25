///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CRXBFF` writer - Clear receive buffer full flag
pub type CRXBFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTXBEF` writer - Clear transmit buffer empty flag
pub type CTXBEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRXBERF` writer - Clear receive CRC error flag
pub type CRXBERF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRXOVRF` writer - Clear receive overrun error flag
pub type CRXOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTXUNRF` writer - Clear transmit underrun error flag
pub type CTXUNRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCF` writer - Clear transfer complete flag
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRF` writer - Clear slave resume flag
pub type CSRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear receive buffer full flag
    #[inline(always)]
    #[must_use]
    pub fn crxbff(&mut self) -> CRXBFF_W<ICRrs> {
        CRXBFF_W::new(self, 0)
    }
    ///Bit 1 - Clear transmit buffer empty flag
    #[inline(always)]
    #[must_use]
    pub fn ctxbef(&mut self) -> CTXBEF_W<ICRrs> {
        CTXBEF_W::new(self, 1)
    }
    ///Bit 2 - Clear receive CRC error flag
    #[inline(always)]
    #[must_use]
    pub fn crxberf(&mut self) -> CRXBERF_W<ICRrs> {
        CRXBERF_W::new(self, 2)
    }
    ///Bit 3 - Clear receive overrun error flag
    #[inline(always)]
    #[must_use]
    pub fn crxovrf(&mut self) -> CRXOVRF_W<ICRrs> {
        CRXOVRF_W::new(self, 3)
    }
    ///Bit 4 - Clear transmit underrun error flag
    #[inline(always)]
    #[must_use]
    pub fn ctxunrf(&mut self) -> CTXUNRF_W<ICRrs> {
        CTXUNRF_W::new(self, 4)
    }
    ///Bit 7 - Clear transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<ICRrs> {
        CTCF_W::new(self, 7)
    }
    ///Bit 8 - Clear slave resume flag
    #[inline(always)]
    #[must_use]
    pub fn csrf(&mut self) -> CSRF_W<ICRrs> {
        CSRF_W::new(self, 8)
    }
}
/**SWPMI Interrupt Flag Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SWPMI1:ICR)*/
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
