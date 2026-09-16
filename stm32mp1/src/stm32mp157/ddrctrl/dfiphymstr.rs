///Register `DFIPHYMSTR` reader
pub type R = crate::R<DFIPHYMSTRrs>;
///Register `DFIPHYMSTR` writer
pub type W = crate::W<DFIPHYMSTRrs>;
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
        f.debug_struct("DFIPHYMSTR")
            .field("dfi_phymstr_en", &self.dfi_phymstr_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_PHYMSTR_EN
    #[inline(always)]
    pub fn dfi_phymstr_en(&mut self) -> DFI_PHYMSTR_EN_W<'_, DFIPHYMSTRrs> {
        DFI_PHYMSTR_EN_W::new(self, 0)
    }
}
/**DDRCTRL DFI PHY master register

You can [`read`](crate::Reg::read) this register and get [`dfiphymstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiphymstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIPHYMSTR)*/
pub struct DFIPHYMSTRrs;
impl crate::RegisterSpec for DFIPHYMSTRrs {
    type Ux = u32;
}
///`read()` method returns [`dfiphymstr::R`](R) reader structure
impl crate::Readable for DFIPHYMSTRrs {}
///`write(|w| ..)` method takes [`dfiphymstr::W`](W) writer structure
impl crate::Writable for DFIPHYMSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFIPHYMSTR to value 0x01
impl crate::Resettable for DFIPHYMSTRrs {
    const RESET_VALUE: u32 = 0x01;
}
