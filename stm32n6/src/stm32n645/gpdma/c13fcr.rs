///Register `C13FCR` writer
pub type W = crate::W<C13FCRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<C13FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<'_, C13FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear
    #[inline(always)]
    pub fn htf(&mut self) -> HTF_W<'_, C13FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear
    #[inline(always)]
    pub fn dtef(&mut self) -> DTEF_W<'_, C13FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear
    #[inline(always)]
    pub fn ulef(&mut self) -> ULEF_W<'_, C13FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear
    #[inline(always)]
    pub fn usef(&mut self) -> USEF_W<'_, C13FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear
    #[inline(always)]
    pub fn suspf(&mut self) -> SUSPF_W<'_, C13FCRrs> {
        SUSPF_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun flag clear
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W<'_, C13FCRrs> {
        TOF_W::new(self, 14)
    }
}
/**GPDMA channel 13 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPDMA:C13FCR)*/
pub struct C13FCRrs;
impl crate::RegisterSpec for C13FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c13fcr::W`](W) writer structure
impl crate::Writable for C13FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C13FCR to value 0
impl crate::Resettable for C13FCRrs {}
