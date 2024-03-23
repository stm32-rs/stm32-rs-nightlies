#[doc = "Register `MDF_DFLT5RSFR` reader"]
pub type R = crate::R<MDF_DFLT5RSFRrs>;
#[doc = "Register `MDF_DFLT5RSFR` writer"]
pub type W = crate::W<MDF_DFLT5RSFRrs>;
#[doc = "Field `RSFLTBYP` reader - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type RSFLTBYP_R = crate::BitReader;
#[doc = "Field `RSFLTBYP` writer - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type RSFLTBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSFLTD` reader - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type RSFLTD_R = crate::BitReader;
#[doc = "Field `RSFLTD` writer - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type RSFLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPFBYP` reader - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type HPFBYP_R = crate::BitReader;
#[doc = "Field `HPFBYP` writer - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type HPFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPFC` reader - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type HPFC_R = crate::FieldReader;
#[doc = "Field `HPFC` writer - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type HPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn rsfltbyp(&self) -> RSFLTBYP_R {
        RSFLTBYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn rsfltd(&self) -> RSFLTD_R {
        RSFLTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn hpfc(&self) -> HPFC_R {
        HPFC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn rsfltbyp(&mut self) -> RSFLTBYP_W<MDF_DFLT5RSFRrs> {
        RSFLTBYP_W::new(self, 0)
    }
    #[doc = "Bit 4 - Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn rsfltd(&mut self) -> RSFLTD_W<MDF_DFLT5RSFRrs> {
        RSFLTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn hpfbyp(&mut self) -> HPFBYP_W<MDF_DFLT5RSFRrs> {
        HPFBYP_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn hpfc(&mut self) -> HPFC_W<MDF_DFLT5RSFRrs> {
        HPFC_W::new(self, 8)
    }
}
#[doc = "This register is used to control the reshape and HPF filters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_dflt5rsfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_dflt5rsfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_DFLT5RSFRrs;
impl crate::RegisterSpec for MDF_DFLT5RSFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_dflt5rsfr::R`](R) reader structure"]
impl crate::Readable for MDF_DFLT5RSFRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_dflt5rsfr::W`](W) writer structure"]
impl crate::Writable for MDF_DFLT5RSFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_DFLT5RSFR to value 0"]
impl crate::Resettable for MDF_DFLT5RSFRrs {
    const RESET_VALUE: u32 = 0;
}
