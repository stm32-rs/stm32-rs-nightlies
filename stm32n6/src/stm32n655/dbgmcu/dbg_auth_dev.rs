///Register `DBG_AUTH_DEV` reader
pub type R = crate::R<DBG_AUTH_DEVrs>;
///Register `DBG_AUTH_DEV` writer
pub type W = crate::W<DBG_AUTH_DEVrs>;
///Field `MESSAGE` reader - Mailbox between debugger and processor
pub type MESSAGE_R = crate::FieldReader<u32>;
///Field `MESSAGE` writer - Mailbox between debugger and processor
pub type MESSAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Mailbox between debugger and processor
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_DEV")
            .field("message", &self.message())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Mailbox between debugger and processor
    #[inline(always)]
    pub fn message(&mut self) -> MESSAGE_W<'_, DBG_AUTH_DEVrs> {
        MESSAGE_W::new(self, 0)
    }
}
/**DBGMCU device authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_dev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_dev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:DBG_AUTH_DEV)*/
pub struct DBG_AUTH_DEVrs;
impl crate::RegisterSpec for DBG_AUTH_DEVrs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_dev::R`](R) reader structure
impl crate::Readable for DBG_AUTH_DEVrs {}
///`write(|w| ..)` method takes [`dbg_auth_dev::W`](W) writer structure
impl crate::Writable for DBG_AUTH_DEVrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_AUTH_DEV to value 0
impl crate::Resettable for DBG_AUTH_DEVrs {}
