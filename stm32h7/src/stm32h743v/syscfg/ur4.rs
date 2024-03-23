#[doc = "Register `UR4` reader"]
pub type R = crate::R<UR4rs>;
#[doc = "Field `MEPAD_1` reader - Mass Erase Protected Area Disabled for bank 1"]
pub type MEPAD_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR4rs;
impl crate::RegisterSpec for UR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur4::R`](R) reader structure"]
impl crate::Readable for UR4rs {}
#[doc = "`reset()` method sets UR4 to value 0"]
impl crate::Resettable for UR4rs {
    const RESET_VALUE: u32 = 0;
}
