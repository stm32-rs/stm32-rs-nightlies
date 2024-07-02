///Register `IPCC_C2MR` reader
pub type R = crate::R<IPCC_C2MRrs>;
///Register `IPCC_C2MR` writer
pub type W = crate::W<IPCC_C2MRrs>;
///Field `CHxOM` reader - CHxOM
pub type CHX_OM_R = crate::FieldReader;
///Field `CHxOM` writer - CHxOM
pub type CHX_OM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CHxFM` reader - CHxFM
pub type CHX_FM_R = crate::FieldReader;
///Field `CHxFM` writer - CHxFM
pub type CHX_FM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - CHxOM
    #[inline(always)]
    pub fn chx_om(&self) -> CHX_OM_R {
        CHX_OM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:21 - CHxFM
    #[inline(always)]
    pub fn chx_fm(&self) -> CHX_FM_R {
        CHX_FM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC_C2MR")
            .field("chx_om", &self.chx_om())
            .field("chx_fm", &self.chx_fm())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - CHxOM
    #[inline(always)]
    #[must_use]
    pub fn chx_om(&mut self) -> CHX_OM_W<IPCC_C2MRrs> {
        CHX_OM_W::new(self, 0)
    }
    ///Bits 16:21 - CHxFM
    #[inline(always)]
    #[must_use]
    pub fn chx_fm(&mut self) -> CHX_FM_W<IPCC_C2MRrs> {
        CHX_FM_W::new(self, 16)
    }
}
/**IPCC Processor 2 mask register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2MR)*/
pub struct IPCC_C2MRrs;
impl crate::RegisterSpec for IPCC_C2MRrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_c2mr::R`](R) reader structure
impl crate::Readable for IPCC_C2MRrs {}
///`write(|w| ..)` method takes [`ipcc_c2mr::W`](W) writer structure
impl crate::Writable for IPCC_C2MRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPCC_C2MR to value 0xffff_ffff
impl crate::Resettable for IPCC_C2MRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
