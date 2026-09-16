///Register `DBG_AUTH_DEVICE` reader
pub type R = crate::R<DBG_AUTH_DEVICErs>;
///Field `AUTH_ID` reader - Device specific ID Device specific ID used for RDP regression.
pub type AUTH_ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Device specific ID Device specific ID used for RDP regression.
    #[inline(always)]
    pub fn auth_id(&self) -> AUTH_ID_R {
        AUTH_ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_DEVICE")
            .field("auth_id", &self.auth_id())
            .finish()
    }
}
/**DBGMCU debug device authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#DBGMCU:DBG_AUTH_DEVICE)*/
pub struct DBG_AUTH_DEVICErs;
impl crate::RegisterSpec for DBG_AUTH_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_device::R`](R) reader structure
impl crate::Readable for DBG_AUTH_DEVICErs {}
///`reset()` method sets DBG_AUTH_DEVICE to value 0
impl crate::Resettable for DBG_AUTH_DEVICErs {}
