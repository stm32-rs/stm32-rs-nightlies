///Register `DDRCTRL_DFIPHYMSTR` reader
pub type R = crate::R<DDRCTRL_DFIPHYMSTRrs>;
///Register `DDRCTRL_DFIPHYMSTR` writer
pub type W = crate::W<DDRCTRL_DFIPHYMSTRrs>;
///Field `DFI_PHYMSTR_EN` reader - DFI_PHYMSTR_EN
pub type DFI_PHYMSTR_EN_R = crate::BitReader;
///Field `DFI_PHYMSTR_EN` writer - DFI_PHYMSTR_EN
pub type DFI_PHYMSTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFI_PHYMSTR_EN
    #[inline(always)]
    pub fn dfi_phymstr_en(&self) -> DFI_PHYMSTR_EN_R {
        DFI_PHYMSTR_EN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_DFIPHYMSTR")
            .field("dfi_phymstr_en", &self.dfi_phymstr_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_PHYMSTR_EN
    #[inline(always)]
    #[must_use]
    pub fn dfi_phymstr_en(&mut self) -> DFI_PHYMSTR_EN_W<DDRCTRL_DFIPHYMSTRrs> {
        DFI_PHYMSTR_EN_W::new(self, 0)
    }
}
/**DDRCTRL DFI PHY master register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_dfiphymstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_dfiphymstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_DFIPHYMSTR)*/
pub struct DDRCTRL_DFIPHYMSTRrs;
impl crate::RegisterSpec for DDRCTRL_DFIPHYMSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_dfiphymstr::R`](R) reader structure
impl crate::Readable for DDRCTRL_DFIPHYMSTRrs {}
///`write(|w| ..)` method takes [`ddrctrl_dfiphymstr::W`](W) writer structure
impl crate::Writable for DDRCTRL_DFIPHYMSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DFIPHYMSTR to value 0x01
impl crate::Resettable for DDRCTRL_DFIPHYMSTRrs {
    const RESET_VALUE: u32 = 0x01;
}
