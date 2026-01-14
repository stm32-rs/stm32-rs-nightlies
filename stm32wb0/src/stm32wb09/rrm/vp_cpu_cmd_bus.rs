///Register `VP_CPU_CMD_BUS` reader
pub type R = crate::R<VP_CPU_CMD_BUSrs>;
///Register `VP_CPU_CMD_BUS` writer
pub type W = crate::W<VP_CPU_CMD_BUSrs>;
///Field `COMMAND` reader - command number
pub type COMMAND_R = crate::FieldReader;
///Field `COMMAND` writer - command number
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMMAND_REQ` reader - CPU Virtual port command request:
pub type COMMAND_REQ_R = crate::BitReader;
///Field `COMMAND_REQ` writer - CPU Virtual port command request:
pub type COMMAND_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - command number
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - CPU Virtual port command request:
    #[inline(always)]
    pub fn command_req(&self) -> COMMAND_REQ_R {
        COMMAND_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VP_CPU_CMD_BUS")
            .field("command", &self.command())
            .field("command_req", &self.command_req())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - command number
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W<'_, VP_CPU_CMD_BUSrs> {
        COMMAND_W::new(self, 0)
    }
    ///Bit 3 - CPU Virtual port command request:
    #[inline(always)]
    pub fn command_req(&mut self) -> COMMAND_REQ_W<'_, VP_CPU_CMD_BUSrs> {
        COMMAND_REQ_W::new(self, 3)
    }
}
/**VP_CPU_CMD_BUS register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_cmd_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_cmd_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RRM:VP_CPU_CMD_BUS)*/
pub struct VP_CPU_CMD_BUSrs;
impl crate::RegisterSpec for VP_CPU_CMD_BUSrs {
    type Ux = u32;
}
///`read()` method returns [`vp_cpu_cmd_bus::R`](R) reader structure
impl crate::Readable for VP_CPU_CMD_BUSrs {}
///`write(|w| ..)` method takes [`vp_cpu_cmd_bus::W`](W) writer structure
impl crate::Writable for VP_CPU_CMD_BUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VP_CPU_CMD_BUS to value 0
impl crate::Resettable for VP_CPU_CMD_BUSrs {}
