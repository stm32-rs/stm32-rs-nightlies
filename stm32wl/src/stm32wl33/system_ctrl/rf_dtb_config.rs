///Register `RF_DTB_CONFIG` reader
pub type R = crate::R<RF_DTB_CONFIGrs>;
///Register `RF_DTB_CONFIG` writer
pub type W = crate::W<RF_DTB_CONFIGrs>;
///Field `RF_DTB_CONFIG` reader - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)
pub type RF_DTB_CONFIG_R = crate::FieldReader;
///Field `RF_DTB_CONFIG` writer - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)
pub type RF_DTB_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)
    #[inline(always)]
    pub fn rf_dtb_config(&self) -> RF_DTB_CONFIG_R {
        RF_DTB_CONFIG_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_DTB_CONFIG")
            .field("rf_dtb_config", &self.rf_dtb_config())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)
    #[inline(always)]
    pub fn rf_dtb_config(&mut self) -> RF_DTB_CONFIG_W<'_, RF_DTB_CONFIGrs> {
        RF_DTB_CONFIG_W::new(self, 0)
    }
}
/**RF_DTB_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rf_dtb_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_dtb_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:RF_DTB_CONFIG)*/
pub struct RF_DTB_CONFIGrs;
impl crate::RegisterSpec for RF_DTB_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`rf_dtb_config::R`](R) reader structure
impl crate::Readable for RF_DTB_CONFIGrs {}
///`write(|w| ..)` method takes [`rf_dtb_config::W`](W) writer structure
impl crate::Writable for RF_DTB_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_DTB_CONFIG to value 0
impl crate::Resettable for RF_DTB_CONFIGrs {}
