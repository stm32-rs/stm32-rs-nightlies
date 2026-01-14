///Register `DFIUPD2` reader
pub type R = crate::R<DFIUPD2rs>;
///Register `DFIUPD2` writer
pub type W = crate::W<DFIUPD2rs>;
///Field `DFI_PHYUPD_EN` reader - DFI_PHYUPD_EN
pub type DFI_PHYUPD_EN_R = crate::BitReader;
///Field `DFI_PHYUPD_EN` writer - DFI_PHYUPD_EN
pub type DFI_PHYUPD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - DFI_PHYUPD_EN
    #[inline(always)]
    pub fn dfi_phyupd_en(&self) -> DFI_PHYUPD_EN_R {
        DFI_PHYUPD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFIUPD2")
            .field("dfi_phyupd_en", &self.dfi_phyupd_en())
            .finish()
    }
}
impl W {
    ///Bit 31 - DFI_PHYUPD_EN
    #[inline(always)]
    pub fn dfi_phyupd_en(&mut self) -> DFI_PHYUPD_EN_W<'_, DFIUPD2rs> {
        DFI_PHYUPD_EN_W::new(self, 31)
    }
}
/**DDRCTRL DFI update register 2

You can [`read`](crate::Reg::read) this register and get [`dfiupd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiupd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIUPD2)*/
pub struct DFIUPD2rs;
impl crate::RegisterSpec for DFIUPD2rs {
    type Ux = u32;
}
///`read()` method returns [`dfiupd2::R`](R) reader structure
impl crate::Readable for DFIUPD2rs {}
///`write(|w| ..)` method takes [`dfiupd2::W`](W) writer structure
impl crate::Writable for DFIUPD2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFIUPD2 to value 0x8000_0000
impl crate::Resettable for DFIUPD2rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
