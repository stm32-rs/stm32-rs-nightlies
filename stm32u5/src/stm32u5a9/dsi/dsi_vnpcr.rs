///Register `DSI_VNPCR` reader
pub type R = crate::R<DSI_VNPCRrs>;
///Register `DSI_VNPCR` writer
pub type W = crate::W<DSI_VNPCRrs>;
///Field `NPSIZE` reader - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
pub type NPSIZE_R = crate::FieldReader<u16>;
///Field `NPSIZE` writer - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
pub type NPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VNPCR")
            .field("npsize", &self.npsize())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
    #[inline(always)]
    #[must_use]
    pub fn npsize(&mut self) -> NPSIZE_W<DSI_VNPCRrs> {
        NPSIZE_W::new(self, 0)
    }
}
/**DSI Host video null packet configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vnpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vnpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VNPCR)*/
pub struct DSI_VNPCRrs;
impl crate::RegisterSpec for DSI_VNPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vnpcr::R`](R) reader structure
impl crate::Readable for DSI_VNPCRrs {}
///`write(|w| ..)` method takes [`dsi_vnpcr::W`](W) writer structure
impl crate::Writable for DSI_VNPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VNPCR to value 0
impl crate::Resettable for DSI_VNPCRrs {
    const RESET_VALUE: u32 = 0;
}
