///Register `C12FCR` writer
pub type W = crate::W<C12FCRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<C12FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<C12FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear
    #[inline(always)]
    pub fn htf(&mut self) -> HTF_W<C12FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear
    #[inline(always)]
    pub fn dtef(&mut self) -> DTEF_W<C12FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear
    #[inline(always)]
    pub fn ulef(&mut self) -> ULEF_W<C12FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear
    #[inline(always)]
    pub fn usef(&mut self) -> USEF_W<C12FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear
    #[inline(always)]
    pub fn suspf(&mut self) -> SUSPF_W<C12FCRrs> {
        SUSPF_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun flag clear
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W<C12FCRrs> {
        TOF_W::new(self, 14)
    }
}
/**GPDMA channel 12 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPDMA:C12FCR)*/
pub struct C12FCRrs;
impl crate::RegisterSpec for C12FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c12fcr::W`](W) writer structure
impl crate::Writable for C12FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C12FCR to value 0
impl crate::Resettable for C12FCRrs {}
