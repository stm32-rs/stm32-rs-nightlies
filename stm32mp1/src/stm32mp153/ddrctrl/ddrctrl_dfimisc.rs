///Register `DDRCTRL_DFIMISC` reader
pub type R = crate::R<DDRCTRL_DFIMISCrs>;
///Register `DDRCTRL_DFIMISC` writer
pub type W = crate::W<DDRCTRL_DFIMISCrs>;
///Field `DFI_INIT_COMPLETE_EN` reader - DFI_INIT_COMPLETE_EN
pub type DFI_INIT_COMPLETE_EN_R = crate::BitReader;
///Field `DFI_INIT_COMPLETE_EN` writer - DFI_INIT_COMPLETE_EN
pub type DFI_INIT_COMPLETE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTL_IDLE_EN` reader - CTL_IDLE_EN
pub type CTL_IDLE_EN_R = crate::BitReader;
///Field `CTL_IDLE_EN` writer - CTL_IDLE_EN
pub type CTL_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_INIT_START` reader - DFI_INIT_START
pub type DFI_INIT_START_R = crate::BitReader;
///Field `DFI_INIT_START` writer - DFI_INIT_START
pub type DFI_INIT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFI_FREQUENCY` reader - DFI_FREQUENCY
pub type DFI_FREQUENCY_R = crate::FieldReader;
///Field `DFI_FREQUENCY` writer - DFI_FREQUENCY
pub type DFI_FREQUENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - DFI_INIT_COMPLETE_EN
    #[inline(always)]
    pub fn dfi_init_complete_en(&self) -> DFI_INIT_COMPLETE_EN_R {
        DFI_INIT_COMPLETE_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CTL_IDLE_EN
    #[inline(always)]
    pub fn ctl_idle_en(&self) -> CTL_IDLE_EN_R {
        CTL_IDLE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DFI_INIT_START
    #[inline(always)]
    pub fn dfi_init_start(&self) -> DFI_INIT_START_R {
        DFI_INIT_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:12 - DFI_FREQUENCY
    #[inline(always)]
    pub fn dfi_frequency(&self) -> DFI_FREQUENCY_R {
        DFI_FREQUENCY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_DFIMISC")
            .field("dfi_init_complete_en", &self.dfi_init_complete_en())
            .field("ctl_idle_en", &self.ctl_idle_en())
            .field("dfi_init_start", &self.dfi_init_start())
            .field("dfi_frequency", &self.dfi_frequency())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFI_INIT_COMPLETE_EN
    #[inline(always)]
    #[must_use]
    pub fn dfi_init_complete_en(&mut self) -> DFI_INIT_COMPLETE_EN_W<DDRCTRL_DFIMISCrs> {
        DFI_INIT_COMPLETE_EN_W::new(self, 0)
    }
    ///Bit 4 - CTL_IDLE_EN
    #[inline(always)]
    #[must_use]
    pub fn ctl_idle_en(&mut self) -> CTL_IDLE_EN_W<DDRCTRL_DFIMISCrs> {
        CTL_IDLE_EN_W::new(self, 4)
    }
    ///Bit 5 - DFI_INIT_START
    #[inline(always)]
    #[must_use]
    pub fn dfi_init_start(&mut self) -> DFI_INIT_START_W<DDRCTRL_DFIMISCrs> {
        DFI_INIT_START_W::new(self, 5)
    }
    ///Bits 8:12 - DFI_FREQUENCY
    #[inline(always)]
    #[must_use]
    pub fn dfi_frequency(&mut self) -> DFI_FREQUENCY_W<DDRCTRL_DFIMISCrs> {
        DFI_FREQUENCY_W::new(self, 8)
    }
}
/**DDRCTRL DFI miscellaneous control register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_dfimisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_dfimisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_DFIMISC)*/
pub struct DDRCTRL_DFIMISCrs;
impl crate::RegisterSpec for DDRCTRL_DFIMISCrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_dfimisc::R`](R) reader structure
impl crate::Readable for DDRCTRL_DFIMISCrs {}
///`write(|w| ..)` method takes [`ddrctrl_dfimisc::W`](W) writer structure
impl crate::Writable for DDRCTRL_DFIMISCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DFIMISC to value 0x01
impl crate::Resettable for DDRCTRL_DFIMISCrs {
    const RESET_VALUE: u32 = 0x01;
}
