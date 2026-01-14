///Register `OTG_HS_HCFG` reader
pub type R = crate::R<OTG_HS_HCFGrs>;
///Register `OTG_HS_HCFG` writer
pub type W = crate::W<OTG_HS_HCFGrs>;
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
        f.debug_struct("OTG_HS_HCFG")
            .field("fslspcs", &self.fslspcs())
            .field("fslss", &self.fslss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FS/LS PHY clock select
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<'_, OTG_HS_HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
}
/**OTG_HS host configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCFG)*/
pub struct OTG_HS_HCFGrs;
impl crate::RegisterSpec for OTG_HS_HCFGrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_hcfg::R`](R) reader structure
impl crate::Readable for OTG_HS_HCFGrs {}
///`write(|w| ..)` method takes [`otg_hs_hcfg::W`](W) writer structure
impl crate::Writable for OTG_HS_HCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_HCFG to value 0
impl crate::Resettable for OTG_HS_HCFGrs {}
