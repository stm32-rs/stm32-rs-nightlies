///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `FRAME_ISC` writer - Capture complete interrupt status clear
pub type FRAME_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_ISC` writer - Overrun interrupt status clear
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_ISC` writer - Synchronization error interrupt status clear
pub type ERR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSYNC_ISC` writer - Vertical Synchronization interrupt status clear
pub type VSYNC_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_ISC` writer - line interrupt status clear
pub type LINE_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Capture complete interrupt status clear
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<'_, ICRrs> {
        FRAME_ISC_W::new(self, 0)
    }
    ///Bit 1 - Overrun interrupt status clear
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<'_, ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
    ///Bit 2 - Synchronization error interrupt status clear
    #[inline(always)]
    pub fn err_isc(&mut self) -> ERR_ISC_W<'_, ICRrs> {
        ERR_ISC_W::new(self, 2)
    }
    ///Bit 3 - Vertical Synchronization interrupt status clear
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<'_, ICRrs> {
        VSYNC_ISC_W::new(self, 3)
    }
    ///Bit 4 - line interrupt status clear
    #[inline(always)]
    pub fn line_isc(&mut self) -> LINE_ISC_W<'_, ICRrs> {
        LINE_ISC_W::new(self, 4)
    }
}
/**DCMI interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMI:ICR)*/
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
