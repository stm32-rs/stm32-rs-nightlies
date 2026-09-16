///Register `CRCPARCTL0` reader
pub type R = crate::R<CRCPARCTL0rs>;
///Register `CRCPARCTL0` writer
pub type W = crate::W<CRCPARCTL0rs>;
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
        f.debug_struct("CRCPARCTL0")
            .field("dfi_alert_err_int_en", &self.dfi_alert_err_int_en())
            .field("dfi_alert_err_int_clr", &self.dfi_alert_err_int_clr())
            .field("dfi_alert_err_cnt_clr", &self.dfi_alert_err_cnt_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_ALERT_ERR_INT_EN
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&mut self) -> DFI_ALERT_ERR_INT_EN_W<'_, CRCPARCTL0rs> {
        DFI_ALERT_ERR_INT_EN_W::new(self, 0)
    }
    ///Bit 1 - DFI_ALERT_ERR_INT_CLR
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&mut self) -> DFI_ALERT_ERR_INT_CLR_W<'_, CRCPARCTL0rs> {
        DFI_ALERT_ERR_INT_CLR_W::new(self, 1)
    }
    ///Bit 2 - DFI_ALERT_ERR_CNT_CLR
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&mut self) -> DFI_ALERT_ERR_CNT_CLR_W<'_, CRCPARCTL0rs> {
        DFI_ALERT_ERR_CNT_CLR_W::new(self, 2)
    }
}
/**DDRCTRL CRC parity control register 0

You can [`read`](crate::Reg::read) this register and get [`crcparctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcparctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:CRCPARCTL0)*/
pub struct CRCPARCTL0rs;
impl crate::RegisterSpec for CRCPARCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`crcparctl0::R`](R) reader structure
impl crate::Readable for CRCPARCTL0rs {}
///`write(|w| ..)` method takes [`crcparctl0::W`](W) writer structure
impl crate::Writable for CRCPARCTL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCPARCTL0 to value 0
impl crate::Resettable for CRCPARCTL0rs {}
