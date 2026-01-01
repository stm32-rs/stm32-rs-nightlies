///Register `FCR` writer
pub type W = crate::W<FCRrs>;
/**transfer complete flag clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFW {
    ///1: Clear flag
    Clear = 1,
}
impl From<TCFW> for bool {
    #[inline(always)]
    fn from(variant: TCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TCF` writer - transfer complete flag clear
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCFW>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCFW::Clear)
    }
}
///Field `HTF` writer - half transfer flag clear
pub use TCF_W as HTF_W;
///Field `DTEF` writer - data transfer error flag clear
pub use TCF_W as DTEF_W;
///Field `ULEF` writer - update link transfer error flag clear
pub use TCF_W as ULEF_W;
///Field `USEF` writer - user setting error flag clear
pub use TCF_W as USEF_W;
///Field `SUSPF` writer - completed suspension flag clear
pub use TCF_W as SUSPF_W;
///Field `TOF` writer - trigger overrun flag clear
pub use TCF_W as TOF_W;
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 8 - transfer complete flag clear
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<'_, FCRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - half transfer flag clear
    #[inline(always)]
    pub fn htf(&mut self) -> HTF_W<'_, FCRrs> {
        HTF_W::new(self, 9)
    }
    ///Bit 10 - data transfer error flag clear
    #[inline(always)]
    pub fn dtef(&mut self) -> DTEF_W<'_, FCRrs> {
        DTEF_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error flag clear
    #[inline(always)]
    pub fn ulef(&mut self) -> ULEF_W<'_, FCRrs> {
        ULEF_W::new(self, 11)
    }
    ///Bit 12 - user setting error flag clear
    #[inline(always)]
    pub fn usef(&mut self) -> USEF_W<'_, FCRrs> {
        USEF_W::new(self, 12)
    }
    ///Bit 13 - completed suspension flag clear
    #[inline(always)]
    pub fn suspf(&mut self) -> SUSPF_W<'_, FCRrs> {
        SUSPF_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun flag clear
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W<'_, FCRrs> {
        TOF_W::new(self, 14)
    }
}
/**GPDMA channel 0 flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
