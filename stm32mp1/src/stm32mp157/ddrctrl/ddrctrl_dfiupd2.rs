#[doc = "Register `DDRCTRL_DFIUPD2` reader"]
pub type R = crate::R<DDRCTRL_DFIUPD2rs>;
#[doc = "Register `DDRCTRL_DFIUPD2` writer"]
pub type W = crate::W<DDRCTRL_DFIUPD2rs>;
#[doc = "Field `DFI_PHYUPD_EN` reader - DFI_PHYUPD_EN"]
pub type DFI_PHYUPD_EN_R = crate::BitReader;
#[doc = "Field `DFI_PHYUPD_EN` writer - DFI_PHYUPD_EN"]
pub type DFI_PHYUPD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - DFI_PHYUPD_EN"]
    #[inline(always)]
    pub fn dfi_phyupd_en(&self) -> DFI_PHYUPD_EN_R {
        DFI_PHYUPD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - DFI_PHYUPD_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phyupd_en(&mut self) -> DFI_PHYUPD_EN_W<DDRCTRL_DFIUPD2rs> {
        DFI_PHYUPD_EN_W::new(self, 31)
    }
}
#[doc = "DDRCTRL DFI update register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFIUPD2rs;
impl crate::RegisterSpec for DDRCTRL_DFIUPD2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfiupd2::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dfiupd2::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIUPD2 to value 0x8000_0000"]
impl crate::Resettable for DDRCTRL_DFIUPD2rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
