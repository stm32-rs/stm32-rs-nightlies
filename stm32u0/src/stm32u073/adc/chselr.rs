///Register `CHSELR` reader
pub type R = crate::R<CHSELRrs>;
///Register `CHSELR` writer
pub type W = crate::W<CHSELRrs>;
///Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL0_R = crate::BitReader;
///Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL1_R = crate::BitReader;
///Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL2_R = crate::BitReader;
///Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL3_R = crate::BitReader;
///Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL4_R = crate::BitReader;
///Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL5_R = crate::BitReader;
///Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL6_R = crate::BitReader;
///Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL7_R = crate::BitReader;
///Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL8_R = crate::BitReader;
///Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL9_R = crate::BitReader;
///Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL10_R = crate::BitReader;
///Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL11_R = crate::BitReader;
///Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL12_R = crate::BitReader;
///Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL13_R = crate::BitReader;
///Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL14_R = crate::BitReader;
///Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL15_R = crate::BitReader;
///Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL16_R = crate::BitReader;
///Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL17_R = crate::BitReader;
///Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL18_R = crate::BitReader;
///Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL19` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL19_R = crate::BitReader;
///Field `CHSEL19` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel19(&self) -> CHSEL19_R {
        CHSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR")
            .field("chsel0", &self.chsel0())
            .field("chsel1", &self.chsel1())
            .field("chsel2", &self.chsel2())
            .field("chsel3", &self.chsel3())
            .field("chsel4", &self.chsel4())
            .field("chsel5", &self.chsel5())
            .field("chsel6", &self.chsel6())
            .field("chsel7", &self.chsel7())
            .field("chsel8", &self.chsel8())
            .field("chsel9", &self.chsel9())
            .field("chsel10", &self.chsel10())
            .field("chsel11", &self.chsel11())
            .field("chsel12", &self.chsel12())
            .field("chsel13", &self.chsel13())
            .field("chsel14", &self.chsel14())
            .field("chsel15", &self.chsel15())
            .field("chsel16", &self.chsel16())
            .field("chsel17", &self.chsel17())
            .field("chsel18", &self.chsel18())
            .field("chsel19", &self.chsel19())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W<'_, CHSELRrs> {
        CHSEL0_W::new(self, 0)
    }
    ///Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W<'_, CHSELRrs> {
        CHSEL1_W::new(self, 1)
    }
    ///Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W<'_, CHSELRrs> {
        CHSEL2_W::new(self, 2)
    }
    ///Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W<'_, CHSELRrs> {
        CHSEL3_W::new(self, 3)
    }
    ///Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W<'_, CHSELRrs> {
        CHSEL4_W::new(self, 4)
    }
    ///Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W<'_, CHSELRrs> {
        CHSEL5_W::new(self, 5)
    }
    ///Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W<'_, CHSELRrs> {
        CHSEL6_W::new(self, 6)
    }
    ///Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W<'_, CHSELRrs> {
        CHSEL7_W::new(self, 7)
    }
    ///Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W<'_, CHSELRrs> {
        CHSEL8_W::new(self, 8)
    }
    ///Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W<'_, CHSELRrs> {
        CHSEL9_W::new(self, 9)
    }
    ///Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W<'_, CHSELRrs> {
        CHSEL10_W::new(self, 10)
    }
    ///Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL11_W<'_, CHSELRrs> {
        CHSEL11_W::new(self, 11)
    }
    ///Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL12_W<'_, CHSELRrs> {
        CHSEL12_W::new(self, 12)
    }
    ///Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL13_W<'_, CHSELRrs> {
        CHSEL13_W::new(self, 13)
    }
    ///Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL14_W<'_, CHSELRrs> {
        CHSEL14_W::new(self, 14)
    }
    ///Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL15_W<'_, CHSELRrs> {
        CHSEL15_W::new(self, 15)
    }
    ///Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL16_W<'_, CHSELRrs> {
        CHSEL16_W::new(self, 16)
    }
    ///Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL17_W<'_, CHSELRrs> {
        CHSEL17_W::new(self, 17)
    }
    ///Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL18_W<'_, CHSELRrs> {
        CHSEL18_W::new(self, 18)
    }
    ///Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel19(&mut self) -> CHSEL19_W<'_, CHSELRrs> {
        CHSEL19_W::new(self, 19)
    }
}
/**ADC channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#ADC:CHSELR)*/
pub struct CHSELRrs;
impl crate::RegisterSpec for CHSELRrs {
    type Ux = u32;
}
///`read()` method returns [`chselr::R`](R) reader structure
impl crate::Readable for CHSELRrs {}
///`write(|w| ..)` method takes [`chselr::W`](W) writer structure
impl crate::Writable for CHSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CHSELR to value 0
impl crate::Resettable for CHSELRrs {}
