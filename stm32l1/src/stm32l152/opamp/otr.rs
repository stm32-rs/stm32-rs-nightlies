///Register `OTR` reader
pub type R = crate::R<OTRrs>;
///Register `OTR` writer
pub type W = crate::W<OTRrs>;
///Field `AO1_OPT_OFFSET_TRIM` reader - OPAMP1, 10-bit offset trim value for normal mode
pub type AO1_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
///Field `AO1_OPT_OFFSET_TRIM` writer - OPAMP1, 10-bit offset trim value for normal mode
pub type AO1_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `AO2_OPT_OFFSET_TRIM` reader - OPAMP2, 10-bit offset trim value for normal mode
pub type AO2_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
///Field `AO2_OPT_OFFSET_TRIM` writer - OPAMP2, 10-bit offset trim value for normal mode
pub type AO2_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `AO3_OPT_OFFSET_TRIM` reader - OPAMP3, 10-bit offset trim value for normal mode
pub type AO3_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
///Field `AO3_OPT_OFFSET_TRIM` writer - OPAMP3, 10-bit offset trim value for normal mode
pub type AO3_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `OT_USER` reader - Select user or factory trimming value
pub type OT_USER_R = crate::BitReader;
///Field `OT_USER` writer - Select user or factory trimming value
pub type OT_USER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&self) -> AO1_OPT_OFFSET_TRIM_R {
        AO1_OPT_OFFSET_TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&self) -> AO2_OPT_OFFSET_TRIM_R {
        AO2_OPT_OFFSET_TRIM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&self) -> AO3_OPT_OFFSET_TRIM_R {
        AO3_OPT_OFFSET_TRIM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    ///Bit 31 - Select user or factory trimming value
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTR")
            .field("ot_user", &self.ot_user())
            .field("ao3_opt_offset_trim", &self.ao3_opt_offset_trim())
            .field("ao2_opt_offset_trim", &self.ao2_opt_offset_trim())
            .field("ao1_opt_offset_trim", &self.ao1_opt_offset_trim())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&mut self) -> AO1_OPT_OFFSET_TRIM_W<'_, OTRrs> {
        AO1_OPT_OFFSET_TRIM_W::new(self, 0)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&mut self) -> AO2_OPT_OFFSET_TRIM_W<'_, OTRrs> {
        AO2_OPT_OFFSET_TRIM_W::new(self, 10)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&mut self) -> AO3_OPT_OFFSET_TRIM_W<'_, OTRrs> {
        AO3_OPT_OFFSET_TRIM_W::new(self, 20)
    }
    ///Bit 31 - Select user or factory trimming value
    #[inline(always)]
    pub fn ot_user(&mut self) -> OT_USER_W<'_, OTRrs> {
        OT_USER_W::new(self, 31)
    }
}
/**offset trimming register for normal mode

You can [`read`](crate::Reg::read) this register and get [`otr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#OPAMP:OTR)*/
pub struct OTRrs;
impl crate::RegisterSpec for OTRrs {
    type Ux = u32;
}
///`read()` method returns [`otr::R`](R) reader structure
impl crate::Readable for OTRrs {}
///`write(|w| ..)` method takes [`otr::W`](W) writer structure
impl crate::Writable for OTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTR to value 0
impl crate::Resettable for OTRrs {}
