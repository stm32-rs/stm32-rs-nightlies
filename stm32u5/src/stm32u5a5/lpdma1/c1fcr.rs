///Register `C1FCR` writer
pub type W = crate::W<C1FCRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<C1FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear - 0: no effect - 1: clears the corresponding TCF flag
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<C1FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear - 0: no effect - 1: clears the corresponding HTF flag
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<C1FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear - 0: no effect - 1: clears the corresponding DTEF flag
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<C1FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear - 0: no effect - 1: clears the corresponding ULEF flag
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<C1FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear - 0: no effect - 1: clears the corresponding USEF flag
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<C1FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear - 0: no effect - 1: clears the corresponding SUSPF flag
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<C1FCRrs> {
        SUSPF_W::new(self, 13)
    }
}
/**LPDMA channel x flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#LPDMA1:C1FCR)*/
pub struct C1FCRrs;
impl crate::RegisterSpec for C1FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c1fcr::W`](W) writer structure
impl crate::Writable for C1FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1FCR to value 0
impl crate::Resettable for C1FCRrs {
    const RESET_VALUE: u32 = 0;
}
