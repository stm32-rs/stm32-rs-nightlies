#[doc = "Register `UR15` reader"]
pub type R = crate::R<UR15rs>;
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub type FZIWDGSTB_R = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR15rs;
impl crate::RegisterSpec for UR15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur15::R`](R) reader structure"]
impl crate::Readable for UR15rs {}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for UR15rs {
    const RESET_VALUE: u32 = 0;
}
