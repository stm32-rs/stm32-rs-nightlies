#[doc = "Register `CHSELR0` reader"]
pub type R = crate::R<CHSELR0rs>;
#[doc = "Register `CHSELR0` writer"]
pub type W = crate::W<CHSELR0rs>;
#[doc = "Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_R = crate::BitReader;
#[doc = "Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL1_R = crate::BitReader;
#[doc = "Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL2_R = crate::BitReader;
#[doc = "Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL3_R = crate::BitReader;
#[doc = "Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL4_R = crate::BitReader;
#[doc = "Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL5_R = crate::BitReader;
#[doc = "Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL6_R = crate::BitReader;
#[doc = "Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL7_R = crate::BitReader;
#[doc = "Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL8_R = crate::BitReader;
#[doc = "Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL9_R = crate::BitReader;
#[doc = "Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL10_R = crate::BitReader;
#[doc = "Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL11_R = crate::BitReader;
#[doc = "Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL12_R = crate::BitReader;
#[doc = "Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL13_R = crate::BitReader;
#[doc = "Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL14_R = crate::BitReader;
#[doc = "Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL15_R = crate::BitReader;
#[doc = "Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL16_R = crate::BitReader;
#[doc = "Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL17_R = crate::BitReader;
#[doc = "Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL18_R = crate::BitReader;
#[doc = "Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL19` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL19_R = crate::BitReader;
#[doc = "Field `CHSEL19` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL20` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL20_R = crate::BitReader;
#[doc = "Field `CHSEL20` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL21` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL21_R = crate::BitReader;
#[doc = "Field `CHSEL21` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL22` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL22_R = crate::BitReader;
#[doc = "Field `CHSEL22` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel19(&self) -> CHSEL19_R {
        CHSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel20(&self) -> CHSEL20_R {
        CHSEL20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel21(&self) -> CHSEL21_R {
        CHSEL21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel22(&self) -> CHSEL22_R {
        CHSEL22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<CHSELR0rs> {
        CHSEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<CHSELR0rs> {
        CHSEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<CHSELR0rs> {
        CHSEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<CHSELR0rs> {
        CHSEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<CHSELR0rs> {
        CHSEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<CHSELR0rs> {
        CHSEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<CHSELR0rs> {
        CHSEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<CHSELR0rs> {
        CHSEL7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<CHSELR0rs> {
        CHSEL8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> CHSEL9_W<CHSELR0rs> {
        CHSEL9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel10(&mut self) -> CHSEL10_W<CHSELR0rs> {
        CHSEL10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> CHSEL11_W<CHSELR0rs> {
        CHSEL11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> CHSEL12_W<CHSELR0rs> {
        CHSEL12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel13(&mut self) -> CHSEL13_W<CHSELR0rs> {
        CHSEL13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel14(&mut self) -> CHSEL14_W<CHSELR0rs> {
        CHSEL14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel15(&mut self) -> CHSEL15_W<CHSELR0rs> {
        CHSEL15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel16(&mut self) -> CHSEL16_W<CHSELR0rs> {
        CHSEL16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel17(&mut self) -> CHSEL17_W<CHSELR0rs> {
        CHSEL17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel18(&mut self) -> CHSEL18_W<CHSELR0rs> {
        CHSEL18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel19(&mut self) -> CHSEL19_W<CHSELR0rs> {
        CHSEL19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel20(&mut self) -> CHSEL20_W<CHSELR0rs> {
        CHSEL20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel21(&mut self) -> CHSEL21_W<CHSELR0rs> {
        CHSEL21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel22(&mut self) -> CHSEL22_W<CHSELR0rs> {
        CHSEL22_W::new(self, 22)
    }
}
#[doc = "ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSELR0rs;
impl crate::RegisterSpec for CHSELR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr0::R`](R) reader structure"]
impl crate::Readable for CHSELR0rs {}
#[doc = "`write(|w| ..)` method takes [`chselr0::W`](W) writer structure"]
impl crate::Writable for CHSELR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR0 to value 0"]
impl crate::Resettable for CHSELR0rs {
    const RESET_VALUE: u32 = 0;
}
