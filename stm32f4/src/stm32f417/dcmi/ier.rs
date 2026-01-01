///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `FRAME_IE` reader - Capture complete interrupt enable
pub type FRAME_IE_R = crate::BitReader;
///Field `FRAME_IE` writer - Capture complete interrupt enable
pub type FRAME_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_IE` reader - Overrun interrupt enable
pub type OVR_IE_R = crate::BitReader;
///Field `OVR_IE` writer - Overrun interrupt enable
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_IE` reader - Synchronization error interrupt enable
pub type ERR_IE_R = crate::BitReader;
///Field `ERR_IE` writer - Synchronization error interrupt enable
pub type ERR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSYNC_IE` reader - VSYNC interrupt enable
pub type VSYNC_IE_R = crate::BitReader;
///Field `VSYNC_IE` writer - VSYNC interrupt enable
pub type VSYNC_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_IE` reader - Line interrupt enable
pub type LINE_IE_R = crate::BitReader;
///Field `LINE_IE` writer - Line interrupt enable
pub type LINE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture complete interrupt enable
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line interrupt enable
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("line_ie", &self.line_ie())
            .field("vsync_ie", &self.vsync_ie())
            .field("err_ie", &self.err_ie())
            .field("ovr_ie", &self.ovr_ie())
            .field("frame_ie", &self.frame_ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture complete interrupt enable
    #[inline(always)]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<'_, IERrs> {
        FRAME_IE_W::new(self, 0)
    }
    ///Bit 1 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<'_, IERrs> {
        OVR_IE_W::new(self, 1)
    }
    ///Bit 2 - Synchronization error interrupt enable
    #[inline(always)]
    pub fn err_ie(&mut self) -> ERR_IE_W<'_, IERrs> {
        ERR_IE_W::new(self, 2)
    }
    ///Bit 3 - VSYNC interrupt enable
    #[inline(always)]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<'_, IERrs> {
        VSYNC_IE_W::new(self, 3)
    }
    ///Bit 4 - Line interrupt enable
    #[inline(always)]
    pub fn line_ie(&mut self) -> LINE_IE_W<'_, IERrs> {
        LINE_IE_W::new(self, 4)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#DCMI:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
