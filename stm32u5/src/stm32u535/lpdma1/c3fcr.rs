///Register `C3FCR` writer
pub type W = crate::W<C3FCRrs>;
///Field `TCF` writer - transfer complete flag clear - 0: no effect - 1: clears the corresponding TCF flag
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTF` writer - half transfer flag clear - 0: no effect - 1: clears the corresponding HTF flag
pub type HTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEF` writer - data transfer error flag clear - 0: no effect - 1: clears the corresponding DTEF flag
pub type DTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULEF` writer - update link transfer error flag clear - 0: no effect - 1: clears the corresponding ULEF flag
pub type ULEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USEF` writer - user setting error flag clear - 0: no effect - 1: clears the corresponding USEF flag
pub type USEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPF` writer - completed suspension flag clear - 0: no effect - 1: clears the corresponding SUSPF flag
pub type SUSPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<C3FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear - 0: no effect - 1: clears the corresponding TCF flag
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<C3FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear - 0: no effect - 1: clears the corresponding HTF flag
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<C3FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear - 0: no effect - 1: clears the corresponding DTEF flag
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<C3FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear - 0: no effect - 1: clears the corresponding ULEF flag
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<C3FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear - 0: no effect - 1: clears the corresponding USEF flag
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<C3FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear - 0: no effect - 1: clears the corresponding SUSPF flag
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<C3FCRrs> {
        SUSPF_W::new(self, 13)
    }
}
/**LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPDMA1:C3FCR)*/
pub struct C3FCRrs;
impl crate::RegisterSpec for C3FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c3fcr::W`](W) writer structure
impl crate::Writable for C3FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C3FCR to value 0
impl crate::Resettable for C3FCRrs {
    const RESET_VALUE: u32 = 0;
}
