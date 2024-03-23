#[doc = "Register `UR18` reader"]
pub type R = crate::R<UR18rs>;
#[doc = "Field `CPU_FREQ_BOOST` reader - CPU maximum frequency boost"]
pub type CPU_FREQ_BOOST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CPU maximum frequency boost"]
    #[inline(always)]
    pub fn cpu_freq_boost(&self) -> CPU_FREQ_BOOST_R {
        CPU_FREQ_BOOST_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG user register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR18rs;
impl crate::RegisterSpec for UR18rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur18::R`](R) reader structure"]
impl crate::Readable for UR18rs {}
#[doc = "`reset()` method sets UR18 to value 0"]
impl crate::Resettable for UR18rs {
    const RESET_VALUE: u32 = 0;
}
