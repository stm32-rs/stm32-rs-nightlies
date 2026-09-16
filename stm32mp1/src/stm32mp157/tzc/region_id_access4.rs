///Register `REGION_ID_ACCESS4` reader
pub type R = crate::R<REGION_ID_ACCESS4rs>;
///Register `REGION_ID_ACCESS4` writer
pub type W = crate::W<REGION_ID_ACCESS4rs>;
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
        f.debug_struct("REGION_ID_ACCESS4")
            .field("nsaid_rd_en", &self.nsaid_rd_en())
            .field("nsaid_wr_en", &self.nsaid_wr_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NSAID_RD_EN
    #[inline(always)]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W<'_, REGION_ID_ACCESS4rs> {
        NSAID_RD_EN_W::new(self, 0)
    }
    ///Bits 16:31 - NSAID_WR_EN
    #[inline(always)]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W<'_, REGION_ID_ACCESS4rs> {
        NSAID_WR_EN_W::new(self, 16)
    }
}
/**Region non-secure access based on NSAID.

You can [`read`](crate::Reg::read) this register and get [`region_id_access4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_id_access4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:REGION_ID_ACCESS4)*/
pub struct REGION_ID_ACCESS4rs;
impl crate::RegisterSpec for REGION_ID_ACCESS4rs {
    type Ux = u32;
}
///`read()` method returns [`region_id_access4::R`](R) reader structure
impl crate::Readable for REGION_ID_ACCESS4rs {}
///`write(|w| ..)` method takes [`region_id_access4::W`](W) writer structure
impl crate::Writable for REGION_ID_ACCESS4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGION_ID_ACCESS4 to value 0
impl crate::Resettable for REGION_ID_ACCESS4rs {}
