///Register `DSI_VPCR` reader
pub type R = crate::R<DSI_VPCRrs>;
///Register `DSI_VPCR` writer
pub type W = crate::W<DSI_VPCRrs>;
///Field `VPSIZE` reader - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
pub type VPSIZE_R = crate::FieldReader<u16>;
///Field `VPSIZE` writer - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
pub type VPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VPCR")
            .field("vpsize", &self.vpsize())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification.
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<DSI_VPCRrs> {
        VPSIZE_W::new(self, 0)
    }
}
/**DSI Host video packet configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VPCR)*/
pub struct DSI_VPCRrs;
impl crate::RegisterSpec for DSI_VPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vpcr::R`](R) reader structure
impl crate::Readable for DSI_VPCRrs {}
///`write(|w| ..)` method takes [`dsi_vpcr::W`](W) writer structure
impl crate::Writable for DSI_VPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VPCR to value 0
impl crate::Resettable for DSI_VPCRrs {
    const RESET_VALUE: u32 = 0;
}
