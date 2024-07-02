///Register `DSI_VSCR` reader
pub type R = crate::R<DSI_VSCRrs>;
///Register `DSI_VSCR` writer
pub type W = crate::W<DSI_VSCRrs>;
///Field `EN` reader - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UR` reader - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
pub type UR_R = crate::BitReader;
///Field `UR` writer - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
pub type UR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VSCR")
            .field("en", &self.en())
            .field("ur", &self.ur())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DSI_VSCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<DSI_VSCRrs> {
        UR_W::new(self, 8)
    }
}
/**DSI Host video shadow control register

You can [`read`](crate::Reg::read) this register and get [`dsi_vscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VSCR)*/
pub struct DSI_VSCRrs;
impl crate::RegisterSpec for DSI_VSCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vscr::R`](R) reader structure
impl crate::Readable for DSI_VSCRrs {}
///`write(|w| ..)` method takes [`dsi_vscr::W`](W) writer structure
impl crate::Writable for DSI_VSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_VSCR to value 0
impl crate::Resettable for DSI_VSCRrs {
    const RESET_VALUE: u32 = 0;
}
