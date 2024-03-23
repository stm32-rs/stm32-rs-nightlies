#[doc = "Register `DBG_AUTH_DEVICE` reader"]
pub type R = crate::R<DBG_AUTH_DEVICErs>;
#[doc = "Field `AUTH_ID` reader - AUTH_ID"]
pub type AUTH_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - AUTH_ID"]
    #[inline(always)]
    pub fn auth_id(&self) -> AUTH_ID_R {
        AUTH_ID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DBGMCU debug device authentication register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_auth_device::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
