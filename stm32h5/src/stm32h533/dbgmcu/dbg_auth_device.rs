///Register `DBG_AUTH_DEVICE` reader
pub type R = crate::R<DBG_AUTH_DEVICErs>;
///Register `DBG_AUTH_DEVICE` writer
pub type W = crate::W<DBG_AUTH_DEVICErs>;
///Field `MESSAGE` reader - Device to debug host mailbox message.
pub type MESSAGE_R = crate::FieldReader<u32>;
///Field `MESSAGE` writer - Device to debug host mailbox message.
pub type MESSAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Device to debug host mailbox message.
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_DEVICE")
            .field("message", &self.message())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Device to debug host mailbox message.
    #[inline(always)]
    pub fn message(&mut self) -> MESSAGE_W<'_, DBG_AUTH_DEVICErs> {
        MESSAGE_W::new(self, 0)
    }
}
/**DBGMCU debug authentication mailbox device register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_device::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#DBGMCU:DBG_AUTH_DEVICE)*/
pub struct DBG_AUTH_DEVICErs;
impl crate::RegisterSpec for DBG_AUTH_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_device::R`](R) reader structure
impl crate::Readable for DBG_AUTH_DEVICErs {}
///`write(|w| ..)` method takes [`dbg_auth_device::W`](W) writer structure
impl crate::Writable for DBG_AUTH_DEVICErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_AUTH_DEVICE to value 0
impl crate::Resettable for DBG_AUTH_DEVICErs {}
