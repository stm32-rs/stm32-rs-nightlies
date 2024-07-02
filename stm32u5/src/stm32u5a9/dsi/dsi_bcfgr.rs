///Register `DSI_BCFGR` reader
pub type R = crate::R<DSI_BCFGRrs>;
///Register `DSI_BCFGR` writer
pub type W = crate::W<DSI_BCFGRrs>;
///Field `PWRUP` reader - Power-up This bit powers-up the reference bias for the MIPI D-PHY
pub type PWRUP_R = crate::BitReader;
///Field `PWRUP` writer - Power-up This bit powers-up the reference bias for the MIPI D-PHY
pub type PWRUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Power-up This bit powers-up the reference bias for the MIPI D-PHY
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_BCFGR")
            .field("pwrup", &self.pwrup())
            .finish()
    }
}
impl W {
    ///Bit 6 - Power-up This bit powers-up the reference bias for the MIPI D-PHY
    #[inline(always)]
    #[must_use]
    pub fn pwrup(&mut self) -> PWRUP_W<DSI_BCFGRrs> {
        PWRUP_W::new(self, 6)
    }
}
/**DSI bias configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_bcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_bcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_BCFGR)*/
pub struct DSI_BCFGRrs;
impl crate::RegisterSpec for DSI_BCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_bcfgr::R`](R) reader structure
impl crate::Readable for DSI_BCFGRrs {}
///`write(|w| ..)` method takes [`dsi_bcfgr::W`](W) writer structure
impl crate::Writable for DSI_BCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_BCFGR to value 0
impl crate::Resettable for DSI_BCFGRrs {
    const RESET_VALUE: u32 = 0;
}
