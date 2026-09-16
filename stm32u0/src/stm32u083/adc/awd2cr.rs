///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
///Field `AWD2CH0` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH0_R = crate::BitReader;
///Field `AWD2CH0` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH1` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH1_R = crate::BitReader;
///Field `AWD2CH1` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH2` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH2_R = crate::BitReader;
///Field `AWD2CH2` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH3` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH3_R = crate::BitReader;
///Field `AWD2CH3` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH4` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH4_R = crate::BitReader;
///Field `AWD2CH4` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH5` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH5_R = crate::BitReader;
///Field `AWD2CH5` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH6` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH6_R = crate::BitReader;
///Field `AWD2CH6` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH7` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH7_R = crate::BitReader;
///Field `AWD2CH7` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH8` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH8_R = crate::BitReader;
///Field `AWD2CH8` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH9` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH9_R = crate::BitReader;
///Field `AWD2CH9` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH10` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH10_R = crate::BitReader;
///Field `AWD2CH10` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH11` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH11_R = crate::BitReader;
///Field `AWD2CH11` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH12` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH12_R = crate::BitReader;
///Field `AWD2CH12` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH13` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH13_R = crate::BitReader;
///Field `AWD2CH13` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH14` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH14_R = crate::BitReader;
///Field `AWD2CH14` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH15` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH15_R = crate::BitReader;
///Field `AWD2CH15` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH16` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH16_R = crate::BitReader;
///Field `AWD2CH16` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH17` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH17_R = crate::BitReader;
///Field `AWD2CH17` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH18` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH18_R = crate::BitReader;
///Field `AWD2CH18` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2CH19` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH19_R = crate::BitReader;
///Field `AWD2CH19` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type AWD2CH19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch18(&self) -> AWD2CH18_R {
        AWD2CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch19(&self) -> AWD2CH19_R {
        AWD2CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch0", &self.awd2ch0())
            .field("awd2ch1", &self.awd2ch1())
            .field("awd2ch2", &self.awd2ch2())
            .field("awd2ch3", &self.awd2ch3())
            .field("awd2ch4", &self.awd2ch4())
            .field("awd2ch5", &self.awd2ch5())
            .field("awd2ch6", &self.awd2ch6())
            .field("awd2ch7", &self.awd2ch7())
            .field("awd2ch8", &self.awd2ch8())
            .field("awd2ch9", &self.awd2ch9())
            .field("awd2ch10", &self.awd2ch10())
            .field("awd2ch11", &self.awd2ch11())
            .field("awd2ch12", &self.awd2ch12())
            .field("awd2ch13", &self.awd2ch13())
            .field("awd2ch14", &self.awd2ch14())
            .field("awd2ch15", &self.awd2ch15())
            .field("awd2ch16", &self.awd2ch16())
            .field("awd2ch17", &self.awd2ch17())
            .field("awd2ch18", &self.awd2ch18())
            .field("awd2ch19", &self.awd2ch19())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W<'_, AWD2CRrs> {
        AWD2CH0_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W<'_, AWD2CRrs> {
        AWD2CH1_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W<'_, AWD2CRrs> {
        AWD2CH2_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W<'_, AWD2CRrs> {
        AWD2CH3_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W<'_, AWD2CRrs> {
        AWD2CH4_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W<'_, AWD2CRrs> {
        AWD2CH5_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W<'_, AWD2CRrs> {
        AWD2CH6_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W<'_, AWD2CRrs> {
        AWD2CH7_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W<'_, AWD2CRrs> {
        AWD2CH8_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W<'_, AWD2CRrs> {
        AWD2CH9_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W<'_, AWD2CRrs> {
        AWD2CH10_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W<'_, AWD2CRrs> {
        AWD2CH11_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W<'_, AWD2CRrs> {
        AWD2CH12_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W<'_, AWD2CRrs> {
        AWD2CH13_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W<'_, AWD2CRrs> {
        AWD2CH14_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W<'_, AWD2CRrs> {
        AWD2CH15_W::new(self, 15)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W<'_, AWD2CRrs> {
        AWD2CH16_W::new(self, 16)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W<'_, AWD2CRrs> {
        AWD2CH17_W::new(self, 17)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch18(&mut self) -> AWD2CH18_W<'_, AWD2CRrs> {
        AWD2CH18_W::new(self, 18)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch19(&mut self) -> AWD2CH19_W<'_, AWD2CRrs> {
        AWD2CH19_W::new(self, 19)
    }
}
/**ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#ADC:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {}
