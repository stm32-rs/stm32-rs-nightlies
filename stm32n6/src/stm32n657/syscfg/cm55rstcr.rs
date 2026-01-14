///Register `CM55RSTCR` reader
pub type R = crate::R<CM55RSTCRrs>;
///Register `CM55RSTCR` writer
pub type W = crate::W<CM55RSTCRrs>;
///Field `CORE_RESET_TYPE` reader - Select reset to apply on core upon SYSRESETREQ
pub type CORE_RESET_TYPE_R = crate::BitReader;
///Field `CORE_RESET_TYPE` writer - Select reset to apply on core upon SYSRESETREQ
pub type CORE_RESET_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKUP_RST_EN` reader - Select action to perform on a lockup state on the core
pub type LOCKUP_RST_EN_R = crate::BitReader;
///Field `LOCKUP_RST_EN` writer - Select action to perform on a lockup state on the core
pub type LOCKUP_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKUP_NMI_EN` reader - Select action to perform on a lockup state on the core
pub type LOCKUP_NMI_EN_R = crate::BitReader;
///Field `LOCKUP_NMI_EN` writer - Select action to perform on a lockup state on the core
pub type LOCKUP_NMI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Select reset to apply on core upon SYSRESETREQ
    #[inline(always)]
    pub fn core_reset_type(&self) -> CORE_RESET_TYPE_R {
        CORE_RESET_TYPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Select action to perform on a lockup state on the core
    #[inline(always)]
    pub fn lockup_rst_en(&self) -> LOCKUP_RST_EN_R {
        LOCKUP_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Select action to perform on a lockup state on the core
    #[inline(always)]
    pub fn lockup_nmi_en(&self) -> LOCKUP_NMI_EN_R {
        LOCKUP_NMI_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM55RSTCR")
            .field("core_reset_type", &self.core_reset_type())
            .field("lockup_rst_en", &self.lockup_rst_en())
            .field("lockup_nmi_en", &self.lockup_nmi_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Select reset to apply on core upon SYSRESETREQ
    #[inline(always)]
    pub fn core_reset_type(&mut self) -> CORE_RESET_TYPE_W<'_, CM55RSTCRrs> {
        CORE_RESET_TYPE_W::new(self, 0)
    }
    ///Bit 1 - Select action to perform on a lockup state on the core
    #[inline(always)]
    pub fn lockup_rst_en(&mut self) -> LOCKUP_RST_EN_W<'_, CM55RSTCRrs> {
        LOCKUP_RST_EN_W::new(self, 1)
    }
    ///Bit 2 - Select action to perform on a lockup state on the core
    #[inline(always)]
    pub fn lockup_nmi_en(&mut self) -> LOCKUP_NMI_EN_W<'_, CM55RSTCRrs> {
        LOCKUP_NMI_EN_W::new(self, 2)
    }
}
/**SYSCFG Cortex-M55 reset type control register

You can [`read`](crate::Reg::read) this register and get [`cm55rstcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55rstcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:CM55RSTCR)*/
pub struct CM55RSTCRrs;
impl crate::RegisterSpec for CM55RSTCRrs {
    type Ux = u32;
}
///`read()` method returns [`cm55rstcr::R`](R) reader structure
impl crate::Readable for CM55RSTCRrs {}
///`write(|w| ..)` method takes [`cm55rstcr::W`](W) writer structure
impl crate::Writable for CM55RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM55RSTCR to value 0
impl crate::Resettable for CM55RSTCRrs {}
