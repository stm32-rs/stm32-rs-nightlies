#[doc = "Register `DBG_AUTH_DEVICE` reader"]
pub type R = crate::R<DBG_AUTH_DEVICErs>;
#[doc = "Field `MESSAGE` reader - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
pub type MESSAGE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
#[doc = "DBGMCU debug authentication mailbox device register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_auth_device::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_AUTH_DEVICErs;
impl crate::RegisterSpec for DBG_AUTH_DEVICErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_auth_device::R`](R) reader structure"]
impl crate::Readable for DBG_AUTH_DEVICErs {}
#[doc = "`reset()` method sets DBG_AUTH_DEVICE to value 0"]
impl crate::Resettable for DBG_AUTH_DEVICErs {
    const RESET_VALUE: u32 = 0;
}
