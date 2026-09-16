///Register `UDRA_IRQ_ENABLE` reader
pub type R = crate::R<UDRA_IRQ_ENABLErs>;
///Register `UDRA_IRQ_ENABLE` writer
pub type W = crate::W<UDRA_IRQ_ENABLErs>;
///Field `RADIO_CFG_PTR_RELOADED` reader - UDRA interrupt enable (reload radio config pointer)
pub type RADIO_CFG_PTR_RELOADED_R = crate::BitReader;
///Field `RADIO_CFG_PTR_RELOADED` writer - UDRA interrupt enable (reload radio config pointer)
pub type RADIO_CFG_PTR_RELOADED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_START` reader - UDRA interrupt enable (command start)
pub type CMD_START_R = crate::BitReader;
///Field `CMD_START` writer - UDRA interrupt enable (command start)
pub type CMD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_END` reader - UDRA interrupt enable (command end)
pub type CMD_END_R = crate::BitReader;
///Field `CMD_END` writer - UDRA interrupt enable (command end)
pub type CMD_END_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UDRA interrupt enable (reload radio config pointer)
    #[inline(always)]
    pub fn radio_cfg_ptr_reloaded(&self) -> RADIO_CFG_PTR_RELOADED_R {
        RADIO_CFG_PTR_RELOADED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UDRA interrupt enable (command start)
    #[inline(always)]
    pub fn cmd_start(&self) -> CMD_START_R {
        CMD_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - UDRA interrupt enable (command end)
    #[inline(always)]
    pub fn cmd_end(&self) -> CMD_END_R {
        CMD_END_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDRA_IRQ_ENABLE")
            .field("radio_cfg_ptr_reloaded", &self.radio_cfg_ptr_reloaded())
            .field("cmd_start", &self.cmd_start())
            .field("cmd_end", &self.cmd_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - UDRA interrupt enable (reload radio config pointer)
    #[inline(always)]
    pub fn radio_cfg_ptr_reloaded(&mut self) -> RADIO_CFG_PTR_RELOADED_W<'_, UDRA_IRQ_ENABLErs> {
        RADIO_CFG_PTR_RELOADED_W::new(self, 0)
    }
    ///Bit 1 - UDRA interrupt enable (command start)
    #[inline(always)]
    pub fn cmd_start(&mut self) -> CMD_START_W<'_, UDRA_IRQ_ENABLErs> {
        CMD_START_W::new(self, 1)
    }
    ///Bit 2 - UDRA interrupt enable (command end)
    #[inline(always)]
    pub fn cmd_end(&mut self) -> CMD_END_W<'_, UDRA_IRQ_ENABLErs> {
        CMD_END_W::new(self, 2)
    }
}
/**UDRA_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`udra_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RRM:UDRA_IRQ_ENABLE)*/
pub struct UDRA_IRQ_ENABLErs;
impl crate::RegisterSpec for UDRA_IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`udra_irq_enable::R`](R) reader structure
impl crate::Readable for UDRA_IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`udra_irq_enable::W`](W) writer structure
impl crate::Writable for UDRA_IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UDRA_IRQ_ENABLE to value 0
impl crate::Resettable for UDRA_IRQ_ENABLErs {}
