///Register `VP_CPU_IRQ_ENABLE` reader
pub type R = crate::R<VP_CPU_IRQ_ENABLErs>;
///Register `VP_CPU_IRQ_ENABLE` writer
pub type W = crate::W<VP_CPU_IRQ_ENABLErs>;
///Field `PORT_GRANT` reader - CPU virtual port grant interrupt enable
pub type PORT_GRANT_R = crate::BitReader;
///Field `PORT_GRANT` writer - CPU virtual port grant interrupt enable
pub type PORT_GRANT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_RELEASE` reader - CPU virtual port release interrupt enable
pub type PORT_RELEASE_R = crate::BitReader;
///Field `PORT_RELEASE` writer - CPU virtual port release interrupt enable
pub type PORT_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_CMD_START` reader - CPU virtual port command start interrup enable
pub type PORT_CMD_START_R = crate::BitReader;
///Field `PORT_CMD_START` writer - CPU virtual port command start interrup enable
pub type PORT_CMD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_CMD_END` reader - CPU virtual port command end interrup enable
pub type PORT_CMD_END_R = crate::BitReader;
///Field `PORT_CMD_END` writer - CPU virtual port command end interrup enable
pub type PORT_CMD_END_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU virtual port grant interrupt enable
    #[inline(always)]
    pub fn port_grant(&self) -> PORT_GRANT_R {
        PORT_GRANT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU virtual port release interrupt enable
    #[inline(always)]
    pub fn port_release(&self) -> PORT_RELEASE_R {
        PORT_RELEASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - CPU virtual port command start interrup enable
    #[inline(always)]
    pub fn port_cmd_start(&self) -> PORT_CMD_START_R {
        PORT_CMD_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU virtual port command end interrup enable
    #[inline(always)]
    pub fn port_cmd_end(&self) -> PORT_CMD_END_R {
        PORT_CMD_END_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VP_CPU_IRQ_ENABLE")
            .field("port_grant", &self.port_grant())
            .field("port_release", &self.port_release())
            .field("port_cmd_start", &self.port_cmd_start())
            .field("port_cmd_end", &self.port_cmd_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU virtual port grant interrupt enable
    #[inline(always)]
    pub fn port_grant(&mut self) -> PORT_GRANT_W<'_, VP_CPU_IRQ_ENABLErs> {
        PORT_GRANT_W::new(self, 0)
    }
    ///Bit 1 - CPU virtual port release interrupt enable
    #[inline(always)]
    pub fn port_release(&mut self) -> PORT_RELEASE_W<'_, VP_CPU_IRQ_ENABLErs> {
        PORT_RELEASE_W::new(self, 1)
    }
    ///Bit 3 - CPU virtual port command start interrup enable
    #[inline(always)]
    pub fn port_cmd_start(&mut self) -> PORT_CMD_START_W<'_, VP_CPU_IRQ_ENABLErs> {
        PORT_CMD_START_W::new(self, 3)
    }
    ///Bit 4 - CPU virtual port command end interrup enable
    #[inline(always)]
    pub fn port_cmd_end(&mut self) -> PORT_CMD_END_W<'_, VP_CPU_IRQ_ENABLErs> {
        PORT_CMD_END_W::new(self, 4)
    }
}
/**VP_CPU_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RRM:VP_CPU_IRQ_ENABLE)*/
pub struct VP_CPU_IRQ_ENABLErs;
impl crate::RegisterSpec for VP_CPU_IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`vp_cpu_irq_enable::R`](R) reader structure
impl crate::Readable for VP_CPU_IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`vp_cpu_irq_enable::W`](W) writer structure
impl crate::Writable for VP_CPU_IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VP_CPU_IRQ_ENABLE to value 0
impl crate::Resettable for VP_CPU_IRQ_ENABLErs {}
