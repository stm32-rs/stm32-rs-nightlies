#[doc = "Register `GPIOH_HWCFGR9` reader"]
pub type R = crate::R<GPIOH_HWCFGR9rs>;
#[doc = "Field `EN_IO` reader - EN_IO"]
pub type EN_IO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - EN_IO"]
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioh_hwcfgr9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOH_HWCFGR9rs;
impl crate::RegisterSpec for GPIOH_HWCFGR9rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioh_hwcfgr9::R`](R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR9rs {}
#[doc = "`reset()` method sets GPIOH_HWCFGR9 to value 0xff"]
impl crate::Resettable for GPIOH_HWCFGR9rs {
    const RESET_VALUE: u32 = 0xff;
}
