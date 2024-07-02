///Register `DSI_VHSACR` reader
pub type R = crate::R<DSI_VHSACRrs>;
///Register `DSI_VHSACR` writer
pub type W = crate::W<DSI_VHSACRrs>;
///Field `HSA` reader - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
pub type HSA_R = crate::FieldReader<u16>;
///Field `HSA` writer - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
pub type HSA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VHSACR")
            .field("hsa", &self.hsa())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HSA_W<DSI_VHSACRrs> {
        HSA_W::new(self, 0)
    }
}
/**DSI Host video HSA configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vhsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vhsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VHSACR)*/
pub struct DSI_VHSACRrs;
impl crate::RegisterSpec for DSI_VHSACRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vhsacr::R`](R) reader structure
impl crate::Readable for DSI_VHSACRrs {}
///`write(|w| ..)` method takes [`dsi_vhsacr::W`](W) writer structure
impl crate::Writable for DSI_VHSACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VHSACR to value 0
impl crate::Resettable for DSI_VHSACRrs {
    const RESET_VALUE: u32 = 0;
}
