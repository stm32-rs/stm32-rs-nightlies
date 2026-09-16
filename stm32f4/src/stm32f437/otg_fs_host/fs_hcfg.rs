///Register `FS_HCFG` reader
pub type R = crate::R<FS_HCFGrs>;
///Register `FS_HCFG` writer
pub type W = crate::W<FS_HCFGrs>;
///Field `FSLSPCS` reader - FS/LS PHY clock select
pub type FSLSPCS_R = crate::FieldReader;
///Field `FSLSPCS` writer - FS/LS PHY clock select
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FSLSS` reader - FS- and LS-only support
pub type FSLSS_R = crate::BitReader;
impl R {
    ///Bits 0:1 - FS/LS PHY clock select
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - FS- and LS-only support
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HCFG")
            .field("fslspcs", &self.fslspcs())
            .field("fslss", &self.fslss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FS/LS PHY clock select
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<'_, FS_HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
}
/**OTG_FS host configuration register (OTG_FS_HCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_FS_HOST:FS_HCFG)*/
pub struct FS_HCFGrs;
impl crate::RegisterSpec for FS_HCFGrs {
    type Ux = u32;
}
///`read()` method returns [`fs_hcfg::R`](R) reader structure
impl crate::Readable for FS_HCFGrs {}
///`write(|w| ..)` method takes [`fs_hcfg::W`](W) writer structure
impl crate::Writable for FS_HCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_HCFG to value 0
impl crate::Resettable for FS_HCFGrs {}
