///Register `FAIL_CONTROL0` reader
pub type R = crate::R<FAIL_CONTROL0rs>;
///Field `PRIVILEGE` reader - PRIVILEGE
pub type PRIVILEGE_R = crate::BitReader;
///Field `NON_SECURE` reader - NON_SECURE
pub type NON_SECURE_R = crate::BitReader;
///Field `DIRECTION` reader - DIRECTION
pub type DIRECTION_R = crate::BitReader;
impl R {
    ///Bit 20 - PRIVILEGE
    #[inline(always)]
    pub fn privilege(&self) -> PRIVILEGE_R {
        PRIVILEGE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - NON_SECURE
    #[inline(always)]
    pub fn non_secure(&self) -> NON_SECURE_R {
        NON_SECURE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - DIRECTION
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAIL_CONTROL0")
            .field("privilege", &self.privilege())
            .field("non_secure", &self.non_secure())
            .field("direction", &self.direction())
            .finish()
    }
}
/**Status information about the first access that failed a region permission check in the associated filter (0 to 1).

You can [`read`](crate::Reg::read) this register and get [`fail_control0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:FAIL_CONTROL0)*/
pub struct FAIL_CONTROL0rs;
impl crate::RegisterSpec for FAIL_CONTROL0rs {
    type Ux = u32;
}
///`read()` method returns [`fail_control0::R`](R) reader structure
impl crate::Readable for FAIL_CONTROL0rs {}
///`reset()` method sets FAIL_CONTROL0 to value 0
impl crate::Resettable for FAIL_CONTROL0rs {}
