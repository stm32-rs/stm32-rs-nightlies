///Register `DFIUPD0` reader
pub type R = crate::R<DFIUPD0rs>;
///Register `DFIUPD0` writer
pub type W = crate::W<DFIUPD0rs>;
///Field `DFI_T_CTRLUP_MIN` reader - DFI_T_CTRLUP_MIN
pub type DFI_T_CTRLUP_MIN_R = crate::FieldReader<u16>;
///Field `DFI_T_CTRLUP_MIN` writer - DFI_T_CTRLUP_MIN
pub type DFI_T_CTRLUP_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DFI_T_CTRLUP_MAX` reader - DFI_T_CTRLUP_MAX
pub type DFI_T_CTRLUP_MAX_R = crate::FieldReader<u16>;
///Field `DFI_T_CTRLUP_MAX` writer - DFI_T_CTRLUP_MAX
pub type DFI_T_CTRLUP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CTRLUPD_PRE_SRX` reader - CTRLUPD_PRE_SRX
pub type CTRLUPD_PRE_SRX_R = crate::BitReader;
///Field `CTRLUPD_PRE_SRX` writer - CTRLUPD_PRE_SRX
pub type CTRLUPD_PRE_SRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_AUTO_CTRLUPD_SRX` reader - DIS_AUTO_CTRLUPD_SRX
pub type DIS_AUTO_CTRLUPD_SRX_R = crate::BitReader;
///Field `DIS_AUTO_CTRLUPD_SRX` writer - DIS_AUTO_CTRLUPD_SRX
pub type DIS_AUTO_CTRLUPD_SRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_AUTO_CTRLUPD` reader - DIS_AUTO_CTRLUPD
pub type DIS_AUTO_CTRLUPD_R = crate::BitReader;
///Field `DIS_AUTO_CTRLUPD` writer - DIS_AUTO_CTRLUPD
pub type DIS_AUTO_CTRLUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - DFI_T_CTRLUP_MIN
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&self) -> DFI_T_CTRLUP_MIN_R {
        DFI_T_CTRLUP_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - DFI_T_CTRLUP_MAX
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&self) -> DFI_T_CTRLUP_MAX_R {
        DFI_T_CTRLUP_MAX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 29 - CTRLUPD_PRE_SRX
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&self) -> CTRLUPD_PRE_SRX_R {
        CTRLUPD_PRE_SRX_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DIS_AUTO_CTRLUPD_SRX
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&self) -> DIS_AUTO_CTRLUPD_SRX_R {
        DIS_AUTO_CTRLUPD_SRX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DIS_AUTO_CTRLUPD
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&self) -> DIS_AUTO_CTRLUPD_R {
        DIS_AUTO_CTRLUPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFIUPD0")
            .field("dfi_t_ctrlup_min", &self.dfi_t_ctrlup_min())
            .field("dfi_t_ctrlup_max", &self.dfi_t_ctrlup_max())
            .field("ctrlupd_pre_srx", &self.ctrlupd_pre_srx())
            .field("dis_auto_ctrlupd_srx", &self.dis_auto_ctrlupd_srx())
            .field("dis_auto_ctrlupd", &self.dis_auto_ctrlupd())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - DFI_T_CTRLUP_MIN
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&mut self) -> DFI_T_CTRLUP_MIN_W<'_, DFIUPD0rs> {
        DFI_T_CTRLUP_MIN_W::new(self, 0)
    }
    ///Bits 16:25 - DFI_T_CTRLUP_MAX
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&mut self) -> DFI_T_CTRLUP_MAX_W<'_, DFIUPD0rs> {
        DFI_T_CTRLUP_MAX_W::new(self, 16)
    }
    ///Bit 29 - CTRLUPD_PRE_SRX
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&mut self) -> CTRLUPD_PRE_SRX_W<'_, DFIUPD0rs> {
        CTRLUPD_PRE_SRX_W::new(self, 29)
    }
    ///Bit 30 - DIS_AUTO_CTRLUPD_SRX
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&mut self) -> DIS_AUTO_CTRLUPD_SRX_W<'_, DFIUPD0rs> {
        DIS_AUTO_CTRLUPD_SRX_W::new(self, 30)
    }
    ///Bit 31 - DIS_AUTO_CTRLUPD
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&mut self) -> DIS_AUTO_CTRLUPD_W<'_, DFIUPD0rs> {
        DIS_AUTO_CTRLUPD_W::new(self, 31)
    }
}
/**DDRCTRL DFI update register 0

You can [`read`](crate::Reg::read) this register and get [`dfiupd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfiupd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DFIUPD0)*/
pub struct DFIUPD0rs;
impl crate::RegisterSpec for DFIUPD0rs {
    type Ux = u32;
}
///`read()` method returns [`dfiupd0::R`](R) reader structure
impl crate::Readable for DFIUPD0rs {}
///`write(|w| ..)` method takes [`dfiupd0::W`](W) writer structure
impl crate::Writable for DFIUPD0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFIUPD0 to value 0x0040_0003
impl crate::Resettable for DFIUPD0rs {
    const RESET_VALUE: u32 = 0x0040_0003;
}
