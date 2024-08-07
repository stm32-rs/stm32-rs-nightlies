///Register `DDRCTRL_CRCPARCTL0` reader
pub type R = crate::R<DDRCTRL_CRCPARCTL0rs>;
///Register `DDRCTRL_CRCPARCTL0` writer
pub type W = crate::W<DDRCTRL_CRCPARCTL0rs>;
///Field `DFI_ALERT_ERR_INT_EN` reader - DFI_ALERT_ERR_INT_EN
pub type DFI_ALERT_ERR_INT_EN_R = crate::BitReader;
///Field `DFI_ALERT_ERR_INT_EN` writer - DFI_ALERT_ERR_INT_EN
pub type DFI_ALERT_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_ALERT_ERR_INT_CLR` reader - DFI_ALERT_ERR_INT_CLR
pub type DFI_ALERT_ERR_INT_CLR_R = crate::BitReader;
///Field `DFI_ALERT_ERR_INT_CLR` writer - DFI_ALERT_ERR_INT_CLR
pub type DFI_ALERT_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_ALERT_ERR_CNT_CLR` reader - DFI_ALERT_ERR_CNT_CLR
pub type DFI_ALERT_ERR_CNT_CLR_R = crate::BitReader;
///Field `DFI_ALERT_ERR_CNT_CLR` writer - DFI_ALERT_ERR_CNT_CLR
pub type DFI_ALERT_ERR_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFI_ALERT_ERR_INT_EN
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&self) -> DFI_ALERT_ERR_INT_EN_R {
        DFI_ALERT_ERR_INT_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DFI_ALERT_ERR_INT_CLR
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&self) -> DFI_ALERT_ERR_INT_CLR_R {
        DFI_ALERT_ERR_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DFI_ALERT_ERR_CNT_CLR
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&self) -> DFI_ALERT_ERR_CNT_CLR_R {
        DFI_ALERT_ERR_CNT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_CRCPARCTL0")
            .field("dfi_alert_err_int_en", &self.dfi_alert_err_int_en())
            .field("dfi_alert_err_int_clr", &self.dfi_alert_err_int_clr())
            .field("dfi_alert_err_cnt_clr", &self.dfi_alert_err_cnt_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_ALERT_ERR_INT_EN
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_int_en(&mut self) -> DFI_ALERT_ERR_INT_EN_W<DDRCTRL_CRCPARCTL0rs> {
        DFI_ALERT_ERR_INT_EN_W::new(self, 0)
    }
    ///Bit 1 - DFI_ALERT_ERR_INT_CLR
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_int_clr(&mut self) -> DFI_ALERT_ERR_INT_CLR_W<DDRCTRL_CRCPARCTL0rs> {
        DFI_ALERT_ERR_INT_CLR_W::new(self, 1)
    }
    ///Bit 2 - DFI_ALERT_ERR_CNT_CLR
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_cnt_clr(&mut self) -> DFI_ALERT_ERR_CNT_CLR_W<DDRCTRL_CRCPARCTL0rs> {
        DFI_ALERT_ERR_CNT_CLR_W::new(self, 2)
    }
}
/**DDRCTRL CRC parity control register 0

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_crcparctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_crcparctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_CRCPARCTL0)*/
pub struct DDRCTRL_CRCPARCTL0rs;
impl crate::RegisterSpec for DDRCTRL_CRCPARCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_crcparctl0::R`](R) reader structure
impl crate::Readable for DDRCTRL_CRCPARCTL0rs {}
///`write(|w| ..)` method takes [`ddrctrl_crcparctl0::W`](W) writer structure
impl crate::Writable for DDRCTRL_CRCPARCTL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_CRCPARCTL0 to value 0
impl crate::Resettable for DDRCTRL_CRCPARCTL0rs {
    const RESET_VALUE: u32 = 0;
}
