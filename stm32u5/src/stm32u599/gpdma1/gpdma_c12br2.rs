#[doc = "Register `GPDMA_C12BR2` reader"]
pub type R = crate::R<GPDMA_C12BR2rs>;
#[doc = "Register `GPDMA_C12BR2` writer"]
pub type W = crate::W<GPDMA_C12BR2rs>;
#[doc = "Field `BRSAO` reader - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\\[2:0\\]
vs GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
pub type BRSAO_R = crate::FieldReader<u16>;
#[doc = "Field `BRSAO` writer - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\\[2:0\\]
vs GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
pub type BRSAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BRDAO` reader - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\\[2:0\\]
vs GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
pub type BRDAO_R = crate::FieldReader<u16>;
#[doc = "Field `BRDAO` writer - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\\[2:0\\]
vs GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
pub type BRDAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\\[2:0\\]
vs GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn brsao(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\\[2:0\\]
vs GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn brdao(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field BRSAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (i.e. GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (c.f. BRSAO\\[2:0\\]
vs GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn brsao(&mut self) -> BRSAO_W<GPDMA_C12BR2rs> {
        BRSAO_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field BRDAO\\[15:0\\]
is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (i.e. GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (c.f. BRDAO\\[2:0\\]
vs GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn brdao(&mut self) -> BRDAO_W<GPDMA_C12BR2rs> {
        BRDAO_W::new(self, 16)
    }
}
#[doc = "GPDMA channel x block register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c12br2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c12br2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C12BR2rs;
impl crate::RegisterSpec for GPDMA_C12BR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c12br2::R`](R) reader structure"]
impl crate::Readable for GPDMA_C12BR2rs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c12br2::W`](W) writer structure"]
impl crate::Writable for GPDMA_C12BR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C12BR2 to value 0"]
impl crate::Resettable for GPDMA_C12BR2rs {
    const RESET_VALUE: u32 = 0;
}
