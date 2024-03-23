#[doc = "Register `DBGMCU_DBG_AUTH_HOST` reader"]
pub type R = crate::R<DBGMCU_DBG_AUTH_HOSTrs>;
#[doc = "Field `AUTH_KEY` reader - Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
pub type AUTH_KEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
    #[inline(always)]
    pub fn auth_key(&self) -> AUTH_KEY_R {
        AUTH_KEY_R::new(self.bits)
    }
}
#[doc = "DBGMCU debug host authentication register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_DBG_AUTH_HOSTrs;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_HOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_dbg_auth_host::R`](R) reader structure"]
impl crate::Readable for DBGMCU_DBG_AUTH_HOSTrs {}
#[doc = "`reset()` method sets DBGMCU_DBG_AUTH_HOST to value 0"]
impl crate::Resettable for DBGMCU_DBG_AUTH_HOSTrs {
    const RESET_VALUE: u32 = 0;
}
