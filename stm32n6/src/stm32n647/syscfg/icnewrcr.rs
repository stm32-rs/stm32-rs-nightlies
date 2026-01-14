///Register `ICNEWRCR` reader
pub type R = crate::R<ICNEWRCRrs>;
///Register `ICNEWRCR` writer
pub type W = crate::W<ICNEWRCRrs>;
///Field `SDMMC1_EARLY_WR_RSP_ENABLE` reader - None
pub type SDMMC1_EARLY_WR_RSP_ENABLE_R = crate::BitReader;
///Field `SDMMC1_EARLY_WR_RSP_ENABLE` writer - None
pub type SDMMC1_EARLY_WR_RSP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2_EARLY_WR_RSP_ENABLE` reader - None
pub type SDMMC2_EARLY_WR_RSP_ENABLE_R = crate::BitReader;
///Field `SDMMC2_EARLY_WR_RSP_ENABLE` writer - None
pub type SDMMC2_EARLY_WR_RSP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB1_EARLY_WR_RSP_ENABLE` reader - None
pub type USB1_EARLY_WR_RSP_ENABLE_R = crate::BitReader;
///Field `USB1_EARLY_WR_RSP_ENABLE` writer - None
pub type USB1_EARLY_WR_RSP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB2_EARLY_WR_RSP_ENABLE` reader - None
pub type USB2_EARLY_WR_RSP_ENABLE_R = crate::BitReader;
///Field `USB2_EARLY_WR_RSP_ENABLE` writer - None
pub type USB2_EARLY_WR_RSP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn sdmmc1_early_wr_rsp_enable(&self) -> SDMMC1_EARLY_WR_RSP_ENABLE_R {
        SDMMC1_EARLY_WR_RSP_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - None
    #[inline(always)]
    pub fn sdmmc2_early_wr_rsp_enable(&self) -> SDMMC2_EARLY_WR_RSP_ENABLE_R {
        SDMMC2_EARLY_WR_RSP_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - None
    #[inline(always)]
    pub fn usb1_early_wr_rsp_enable(&self) -> USB1_EARLY_WR_RSP_ENABLE_R {
        USB1_EARLY_WR_RSP_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - None
    #[inline(always)]
    pub fn usb2_early_wr_rsp_enable(&self) -> USB2_EARLY_WR_RSP_ENABLE_R {
        USB2_EARLY_WR_RSP_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICNEWRCR")
            .field(
                "sdmmc1_early_wr_rsp_enable",
                &self.sdmmc1_early_wr_rsp_enable(),
            )
            .field(
                "sdmmc2_early_wr_rsp_enable",
                &self.sdmmc2_early_wr_rsp_enable(),
            )
            .field("usb1_early_wr_rsp_enable", &self.usb1_early_wr_rsp_enable())
            .field("usb2_early_wr_rsp_enable", &self.usb2_early_wr_rsp_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn sdmmc1_early_wr_rsp_enable(&mut self) -> SDMMC1_EARLY_WR_RSP_ENABLE_W<'_, ICNEWRCRrs> {
        SDMMC1_EARLY_WR_RSP_ENABLE_W::new(self, 0)
    }
    ///Bit 1 - None
    #[inline(always)]
    pub fn sdmmc2_early_wr_rsp_enable(&mut self) -> SDMMC2_EARLY_WR_RSP_ENABLE_W<'_, ICNEWRCRrs> {
        SDMMC2_EARLY_WR_RSP_ENABLE_W::new(self, 1)
    }
    ///Bit 2 - None
    #[inline(always)]
    pub fn usb1_early_wr_rsp_enable(&mut self) -> USB1_EARLY_WR_RSP_ENABLE_W<'_, ICNEWRCRrs> {
        USB1_EARLY_WR_RSP_ENABLE_W::new(self, 2)
    }
    ///Bit 3 - None
    #[inline(always)]
    pub fn usb2_early_wr_rsp_enable(&mut self) -> USB2_EARLY_WR_RSP_ENABLE_W<'_, ICNEWRCRrs> {
        USB2_EARLY_WR_RSP_ENABLE_W::new(self, 3)
    }
}
/**SYSCFG AHB-AXI bridge early write response control register

You can [`read`](crate::Reg::read) this register and get [`icnewrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnewrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:ICNEWRCR)*/
pub struct ICNEWRCRrs;
impl crate::RegisterSpec for ICNEWRCRrs {
    type Ux = u32;
}
///`read()` method returns [`icnewrcr::R`](R) reader structure
impl crate::Readable for ICNEWRCRrs {}
///`write(|w| ..)` method takes [`icnewrcr::W`](W) writer structure
impl crate::Writable for ICNEWRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICNEWRCR to value 0
impl crate::Resettable for ICNEWRCRrs {}
