#[doc = "Register `DBGMCU_DBG_AUTH_DEVICE` reader"]
pub type R = crate::R<DBGMCU_DBG_AUTH_DEVICErs>;
#[doc = "Field `AUTH_ID` reader - Device specific ID Device specific ID used for RDP regression."]
pub type AUTH_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device specific ID Device specific ID used for RDP regression."]
    #[inline(always)]
    pub fn auth_id(&self) -> AUTH_ID_R {
        AUTH_ID_R::new(self.bits)
    }
}
#[doc = "DBGMCU debug device authentication register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_device::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_DBG_AUTH_DEVICErs;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_DEVICErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_dbg_auth_device::R`](R) reader structure"]
impl crate::Readable for DBGMCU_DBG_AUTH_DEVICErs {}
#[doc = "`reset()` method sets DBGMCU_DBG_AUTH_DEVICE to value 0"]
impl crate::Resettable for DBGMCU_DBG_AUTH_DEVICErs {
    const RESET_VALUE: u32 = 0;
}
