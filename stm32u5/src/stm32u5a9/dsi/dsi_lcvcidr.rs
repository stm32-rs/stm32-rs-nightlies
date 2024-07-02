///Register `DSI_LCVCIDR` reader
pub type R = crate::R<DSI_LCVCIDRrs>;
///Register `DSI_LCVCIDR` writer
pub type W = crate::W<DSI_LCVCIDRrs>;
///Field `VCID` reader - Virtual channel ID This field returns the virtual channel ID for the LTDC interface.
pub type VCID_R = crate::FieldReader;
///Field `VCID` writer - Virtual channel ID This field returns the virtual channel ID for the LTDC interface.
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Virtual channel ID This field returns the virtual channel ID for the LTDC interface.
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_LCVCIDR")
            .field("vcid", &self.vcid())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Virtual channel ID This field returns the virtual channel ID for the LTDC interface.
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<DSI_LCVCIDRrs> {
        VCID_W::new(self, 0)
    }
}
/**DSI Host LTDC current VCID register

You can [`read`](crate::Reg::read) this register and get [`dsi_lcvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_LCVCIDR)*/
pub struct DSI_LCVCIDRrs;
impl crate::RegisterSpec for DSI_LCVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_lcvcidr::R`](R) reader structure
impl crate::Readable for DSI_LCVCIDRrs {}
///`write(|w| ..)` method takes [`dsi_lcvcidr::W`](W) writer structure
impl crate::Writable for DSI_LCVCIDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_LCVCIDR to value 0
impl crate::Resettable for DSI_LCVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
