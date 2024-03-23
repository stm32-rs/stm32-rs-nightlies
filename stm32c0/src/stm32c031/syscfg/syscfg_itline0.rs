#[doc = "Register `SYSCFG_ITLINE0` reader"]
pub type R = crate::R<SYSCFG_ITLINE0rs>;
#[doc = "Field `WWDG` reader - Window watchdog interrupt pending flag"]
pub type WWDG_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Window watchdog interrupt pending flag"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE0rs;
impl crate::RegisterSpec for SYSCFG_ITLINE0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline0::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE0rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE0 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE0rs {
    const RESET_VALUE: u32 = 0;
}
