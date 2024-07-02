///Register `DSI_VVBPCR` reader
pub type R = crate::R<DSI_VVBPCRrs>;
///Register `DSI_VVBPCR` writer
pub type W = crate::W<DSI_VVBPCRrs>;
///Field `VBP` reader - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
pub type VBP_R = crate::FieldReader<u16>;
///Field `VBP` writer - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVBPCR")
            .field("vbp", &self.vbp())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<DSI_VVBPCRrs> {
        VBP_W::new(self, 0)
    }
}
/**DSI Host video VBP configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VVBPCR)*/
pub struct DSI_VVBPCRrs;
impl crate::RegisterSpec for DSI_VVBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvbpcr::R`](R) reader structure
impl crate::Readable for DSI_VVBPCRrs {}
///`write(|w| ..)` method takes [`dsi_vvbpcr::W`](W) writer structure
impl crate::Writable for DSI_VVBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VVBPCR to value 0
impl crate::Resettable for DSI_VVBPCRrs {
    const RESET_VALUE: u32 = 0;
}
