///Register `DEMOD_DIG_OUT` reader
pub type R = crate::R<DEMOD_DIG_OUTrs>;
///Field `CI_FIELD` reader - CI field
pub type CI_FIELD_R = crate::FieldReader;
///Field `AAC_FOUND` reader - aac_found
pub type AAC_FOUND_R = crate::BitReader;
///Field `PD_FOUND` reader - pd_found
pub type PD_FOUND_R = crate::BitReader;
///Field `RX_END` reader - rx_end
pub type RX_END_R = crate::BitReader;
impl R {
    ///Bits 0:1 - CI field
    #[inline(always)]
    pub fn ci_field(&self) -> CI_FIELD_R {
        CI_FIELD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - aac_found
    #[inline(always)]
    pub fn aac_found(&self) -> AAC_FOUND_R {
        AAC_FOUND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - pd_found
    #[inline(always)]
    pub fn pd_found(&self) -> PD_FOUND_R {
        PD_FOUND_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - rx_end
    #[inline(always)]
    pub fn rx_end(&self) -> RX_END_R {
        RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMOD_DIG_OUT")
            .field("ci_field", &self.ci_field())
            .field("aac_found", &self.aac_found())
            .field("pd_found", &self.pd_found())
            .field("rx_end", &self.rx_end())
            .finish()
    }
}
/**DEMOD_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`demod_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:DEMOD_DIG_OUT)*/
pub struct DEMOD_DIG_OUTrs;
impl crate::RegisterSpec for DEMOD_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`demod_dig_out::R`](R) reader structure
impl crate::Readable for DEMOD_DIG_OUTrs {}
///`reset()` method sets DEMOD_DIG_OUT to value 0
impl crate::Resettable for DEMOD_DIG_OUTrs {}
