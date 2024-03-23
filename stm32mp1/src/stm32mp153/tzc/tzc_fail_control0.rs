#[doc = "Register `TZC_FAIL_CONTROL0` reader"]
pub type R = crate::R<TZC_FAIL_CONTROL0rs>;
#[doc = "Field `PRIVILEGE` reader - PRIVILEGE"]
pub type PRIVILEGE_R = crate::BitReader;
#[doc = "Field `NON_SECURE` reader - NON_SECURE"]
pub type NON_SECURE_R = crate::BitReader;
#[doc = "Field `DIRECTION` reader - DIRECTION"]
pub type DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - PRIVILEGE"]
    #[inline(always)]
    pub fn privilege(&self) -> PRIVILEGE_R {
        PRIVILEGE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NON_SECURE"]
    #[inline(always)]
    pub fn non_secure(&self) -> NON_SECURE_R {
        NON_SECURE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DIRECTION"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_control0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_FAIL_CONTROL0rs;
impl crate::RegisterSpec for TZC_FAIL_CONTROL0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_fail_control0::R`](R) reader structure"]
impl crate::Readable for TZC_FAIL_CONTROL0rs {}
#[doc = "`reset()` method sets TZC_FAIL_CONTROL0 to value 0"]
impl crate::Resettable for TZC_FAIL_CONTROL0rs {
    const RESET_VALUE: u32 = 0;
}
