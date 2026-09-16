///Register `COMMAND` reader
pub type R = crate::R<COMMANDrs>;
///Register `COMMAND` writer
pub type W = crate::W<COMMANDrs>;
///Field `COMMAND_ID` reader - Opcode coresponding to a command:
pub type COMMAND_ID_R = crate::FieldReader;
///Field `COMMAND_ID` writer - Opcode coresponding to a command:
pub type COMMAND_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BACK2ACTIVE` reader - Select the default/return state for the Radio FSM to be ACTIVE2
pub type BACK2ACTIVE_R = crate::BitReader;
///Field `BACK2ACTIVE` writer - Select the default/return state for the Radio FSM to be ACTIVE2
pub type BACK2ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BACK2LOCKON` reader - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX
pub type BACK2LOCKON_R = crate::BitReader;
///Field `BACK2LOCKON` writer - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX
pub type BACK2LOCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Opcode coresponding to a command:
    #[inline(always)]
    pub fn command_id(&self) -> COMMAND_ID_R {
        COMMAND_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 25 - Select the default/return state for the Radio FSM to be ACTIVE2
    #[inline(always)]
    pub fn back2active(&self) -> BACK2ACTIVE_R {
        BACK2ACTIVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX
    #[inline(always)]
    pub fn back2lockon(&self) -> BACK2LOCKON_R {
        BACK2LOCKON_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("command_id", &self.command_id())
            .field("back2active", &self.back2active())
            .field("back2lockon", &self.back2lockon())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Opcode coresponding to a command:
    #[inline(always)]
    pub fn command_id(&mut self) -> COMMAND_ID_W<'_, COMMANDrs> {
        COMMAND_ID_W::new(self, 0)
    }
    ///Bit 25 - Select the default/return state for the Radio FSM to be ACTIVE2
    #[inline(always)]
    pub fn back2active(&mut self) -> BACK2ACTIVE_W<'_, COMMANDrs> {
        BACK2ACTIVE_W::new(self, 25)
    }
    ///Bit 26 - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX
    #[inline(always)]
    pub fn back2lockon(&mut self) -> BACK2LOCKON_W<'_, COMMANDrs> {
        BACK2LOCKON_W::new(self, 26)
    }
}
/**COMMAND register

You can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:COMMAND)*/
pub struct COMMANDrs;
impl crate::RegisterSpec for COMMANDrs {
    type Ux = u32;
}
///`read()` method returns [`command::R`](R) reader structure
impl crate::Readable for COMMANDrs {}
///`write(|w| ..)` method takes [`command::W`](W) writer structure
impl crate::Writable for COMMANDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMMAND to value 0
impl crate::Resettable for COMMANDrs {}
