///Register `DBG_AUTH_DEVICE` reader
pub type R = crate::R<DBG_AUTH_DEVICErs>;
///Field `MESSAGE` reader - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register.
pub type MESSAGE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register.
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
/**DBGMCU debug authentication mailbox device register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#DBGMCU:DBG_AUTH_DEVICE)*/
pub struct DBG_AUTH_DEVICErs;
impl crate::RegisterSpec for DBG_AUTH_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_device::R`](R) reader structure
impl crate::Readable for DBG_AUTH_DEVICErs {}
///`reset()` method sets DBG_AUTH_DEVICE to value 0
impl crate::Resettable for DBG_AUTH_DEVICErs {}
