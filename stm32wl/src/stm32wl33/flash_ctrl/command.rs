///Register `COMMAND` reader
pub type R = crate::R<COMMANDrs>;
///Register `COMMAND` writer
pub type W = crate::W<COMMANDrs>;
///Field `COMMAND` reader - Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE
pub type COMMAND_R = crate::FieldReader;
///Field `COMMAND` writer - Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("command", &self.command())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W<'_, COMMANDrs> {
        COMMAND_W::new(self, 0)
    }
}
/**COMMAND register

You can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#FLASH_CTRL:COMMAND)*/
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
