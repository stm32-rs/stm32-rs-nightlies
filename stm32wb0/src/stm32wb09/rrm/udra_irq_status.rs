///Register `UDRA_IRQ_STATUS` reader
pub type R = crate::R<UDRA_IRQ_STATUSrs>;
///Register `UDRA_IRQ_STATUS` writer
pub type W = crate::W<UDRA_IRQ_STATUSrs>;
///Field `RADIO_CFG_PTR_RELOADED` reader - On read, returns the UDRA reload radio configuration pointer interrupt status.
pub type RADIO_CFG_PTR_RELOADED_R = crate::BitReader;
///Field `RADIO_CFG_PTR_RELOADED` writer - On read, returns the UDRA reload radio configuration pointer interrupt status.
pub type RADIO_CFG_PTR_RELOADED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_STARD` reader - On read, returns the UDRA command start interrupt status.
pub type CMD_STARD_R = crate::BitReader;
///Field `CMD_STARD` writer - On read, returns the UDRA command start interrupt status.
pub type CMD_STARD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_END` reader - On read, returns the UDRA command end interrupt status
pub type CMD_END_R = crate::BitReader;
///Field `CMD_END` writer - On read, returns the UDRA command end interrupt status
pub type CMD_END_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - On read, returns the UDRA reload radio configuration pointer interrupt status.
    #[inline(always)]
    pub fn radio_cfg_ptr_reloaded(&self) -> RADIO_CFG_PTR_RELOADED_R {
        RADIO_CFG_PTR_RELOADED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - On read, returns the UDRA command start interrupt status.
    #[inline(always)]
    pub fn cmd_stard(&self) -> CMD_STARD_R {
        CMD_STARD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - On read, returns the UDRA command end interrupt status
    #[inline(always)]
    pub fn cmd_end(&self) -> CMD_END_R {
        CMD_END_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDRA_IRQ_STATUS")
            .field("radio_cfg_ptr_reloaded", &self.radio_cfg_ptr_reloaded())
            .field("cmd_stard", &self.cmd_stard())
            .field("cmd_end", &self.cmd_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - On read, returns the UDRA reload radio configuration pointer interrupt status.
    #[inline(always)]
    pub fn radio_cfg_ptr_reloaded(&mut self) -> RADIO_CFG_PTR_RELOADED_W<'_, UDRA_IRQ_STATUSrs> {
        RADIO_CFG_PTR_RELOADED_W::new(self, 0)
    }
    ///Bit 1 - On read, returns the UDRA command start interrupt status.
    #[inline(always)]
    pub fn cmd_stard(&mut self) -> CMD_STARD_W<'_, UDRA_IRQ_STATUSrs> {
        CMD_STARD_W::new(self, 1)
    }
    ///Bit 2 - On read, returns the UDRA command end interrupt status
    #[inline(always)]
    pub fn cmd_end(&mut self) -> CMD_END_W<'_, UDRA_IRQ_STATUSrs> {
        CMD_END_W::new(self, 2)
    }
}
/**UDRA_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`udra_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:UDRA_IRQ_STATUS)*/
pub struct UDRA_IRQ_STATUSrs;
impl crate::RegisterSpec for UDRA_IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`udra_irq_status::R`](R) reader structure
impl crate::Readable for UDRA_IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`udra_irq_status::W`](W) writer structure
impl crate::Writable for UDRA_IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UDRA_IRQ_STATUS to value 0
impl crate::Resettable for UDRA_IRQ_STATUSrs {}
