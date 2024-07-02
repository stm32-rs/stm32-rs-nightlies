///Register `GPDMA_C7FCR` writer
pub type W = crate::W<GPDMA_C7FCRrs>;
///Field `TCF` writer - transfer complete flag clear
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTF` writer - half transfer flag clear
pub type HTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEF` writer - data transfer error flag clear
pub type DTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULEF` writer - update link transfer error flag clear
pub type ULEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USEF` writer - user setting error flag clear
pub type USEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPF` writer - completed suspension flag clear
pub type SUSPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOF` writer - trigger overrun flag clear
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPDMA_C7FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<GPDMA_C7FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<GPDMA_C7FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<GPDMA_C7FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<GPDMA_C7FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<GPDMA_C7FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<GPDMA_C7FCRrs> {
        SUSPF_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun flag clear
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<GPDMA_C7FCRrs> {
        TOF_W::new(self, 14)
    }
}
/**GPDMA channel 7 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c7fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GPDMA1:GPDMA_C7FCR)*/
pub struct GPDMA_C7FCRrs;
impl crate::RegisterSpec for GPDMA_C7FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpdma_c7fcr::W`](W) writer structure
impl crate::Writable for GPDMA_C7FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPDMA_C7FCR to value 0
impl crate::Resettable for GPDMA_C7FCRrs {
    const RESET_VALUE: u32 = 0;
}
