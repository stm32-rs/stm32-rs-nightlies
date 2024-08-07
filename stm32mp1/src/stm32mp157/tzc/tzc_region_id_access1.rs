///Register `TZC_REGION_ID_ACCESS1` reader
pub type R = crate::R<TZC_REGION_ID_ACCESS1rs>;
///Register `TZC_REGION_ID_ACCESS1` writer
pub type W = crate::W<TZC_REGION_ID_ACCESS1rs>;
///Field `NSAID_RD_EN` reader - NSAID_RD_EN
pub type NSAID_RD_EN_R = crate::FieldReader<u16>;
///Field `NSAID_RD_EN` writer - NSAID_RD_EN
pub type NSAID_RD_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `NSAID_WR_EN` reader - NSAID_WR_EN
pub type NSAID_WR_EN_R = crate::FieldReader<u16>;
///Field `NSAID_WR_EN` writer - NSAID_WR_EN
pub type NSAID_WR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NSAID_RD_EN
    #[inline(always)]
    pub fn nsaid_rd_en(&self) -> NSAID_RD_EN_R {
        NSAID_RD_EN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - NSAID_WR_EN
    #[inline(always)]
    pub fn nsaid_wr_en(&self) -> NSAID_WR_EN_R {
        NSAID_WR_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_REGION_ID_ACCESS1")
            .field("nsaid_rd_en", &self.nsaid_rd_en())
            .field("nsaid_wr_en", &self.nsaid_wr_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NSAID_RD_EN
    #[inline(always)]
    #[must_use]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W<TZC_REGION_ID_ACCESS1rs> {
        NSAID_RD_EN_W::new(self, 0)
    }
    ///Bits 16:31 - NSAID_WR_EN
    #[inline(always)]
    #[must_use]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W<TZC_REGION_ID_ACCESS1rs> {
        NSAID_WR_EN_W::new(self, 16)
    }
}
/**Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`tzc_region_id_access1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_region_id_access1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:TZC_REGION_ID_ACCESS1)*/
pub struct TZC_REGION_ID_ACCESS1rs;
impl crate::RegisterSpec for TZC_REGION_ID_ACCESS1rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_region_id_access1::R`](R) reader structure
impl crate::Readable for TZC_REGION_ID_ACCESS1rs {}
///`write(|w| ..)` method takes [`tzc_region_id_access1::W`](W) writer structure
impl crate::Writable for TZC_REGION_ID_ACCESS1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TZC_REGION_ID_ACCESS1 to value 0
impl crate::Resettable for TZC_REGION_ID_ACCESS1rs {
    const RESET_VALUE: u32 = 0;
}
