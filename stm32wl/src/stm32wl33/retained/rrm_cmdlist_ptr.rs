///Register `RRM_CMDLIST_PTR` reader
pub type R = crate::R<RRM_CMDLIST_PTRrs>;
///Register `RRM_CMDLIST_PTR` writer
pub type W = crate::W<RRM_CMDLIST_PTRrs>;
///Field `CMDLIST_PTR_OFFSET` reader - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point.
pub type CMDLIST_PTR_OFFSET_R = crate::FieldReader<u16>;
///Field `CMDLIST_PTR_OFFSET` writer - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point.
pub type CMDLIST_PTR_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CMDLIST_PTR_VALID` reader - Indicate if a command list has to be executed or not
pub type CMDLIST_PTR_VALID_R = crate::BitReader;
///Field `CMDLIST_PTR_VALID` writer - Indicate if a command list has to be executed or not
pub type CMDLIST_PTR_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point.
    #[inline(always)]
    pub fn cmdlist_ptr_offset(&self) -> CMDLIST_PTR_OFFSET_R {
        CMDLIST_PTR_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - Indicate if a command list has to be executed or not
    #[inline(always)]
    pub fn cmdlist_ptr_valid(&self) -> CMDLIST_PTR_VALID_R {
        CMDLIST_PTR_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRM_CMDLIST_PTR")
            .field("cmdlist_ptr_offset", &self.cmdlist_ptr_offset())
            .field("cmdlist_ptr_valid", &self.cmdlist_ptr_valid())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point.
    #[inline(always)]
    pub fn cmdlist_ptr_offset(&mut self) -> CMDLIST_PTR_OFFSET_W<'_, RRM_CMDLIST_PTRrs> {
        CMDLIST_PTR_OFFSET_W::new(self, 0)
    }
    ///Bit 31 - Indicate if a command list has to be executed or not
    #[inline(always)]
    pub fn cmdlist_ptr_valid(&mut self) -> CMDLIST_PTR_VALID_W<'_, RRM_CMDLIST_PTRrs> {
        CMDLIST_PTR_VALID_W::new(self, 31)
    }
}
/**RRM_CMDLIST_PTR register

You can [`read`](crate::Reg::read) this register and get [`rrm_cmdlist_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_cmdlist_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:RRM_CMDLIST_PTR)*/
pub struct RRM_CMDLIST_PTRrs;
impl crate::RegisterSpec for RRM_CMDLIST_PTRrs {
    type Ux = u32;
}
///`read()` method returns [`rrm_cmdlist_ptr::R`](R) reader structure
impl crate::Readable for RRM_CMDLIST_PTRrs {}
///`write(|w| ..)` method takes [`rrm_cmdlist_ptr::W`](W) writer structure
impl crate::Writable for RRM_CMDLIST_PTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RRM_CMDLIST_PTR to value 0
impl crate::Resettable for RRM_CMDLIST_PTRrs {}
