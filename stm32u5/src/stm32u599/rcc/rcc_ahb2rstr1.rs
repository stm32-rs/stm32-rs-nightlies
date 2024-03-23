#[doc = "Register `RCC_AHB2RSTR1` reader"]
pub type R = crate::R<RCC_AHB2RSTR1rs>;
#[doc = "Register `RCC_AHB2RSTR1` writer"]
pub type W = crate::W<RCC_AHB2RSTR1rs>;
#[doc = "Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software."]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software."]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software."]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software."]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software."]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software."]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software."]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software."]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - I/O port E reset This bit is set and cleared by software."]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - I/O port E reset This bit is set and cleared by software."]
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOFRST_R = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - I/O port G reset This bit is set and cleared by software."]
pub type GPIOGRST_R = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - I/O port G reset This bit is set and cleared by software."]
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - I/O port H reset This bit is set and cleared by software."]
pub type GPIOHRST_R = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - I/O port H reset This bit is set and cleared by software."]
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIRST` reader - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOIRST_R = crate::BitReader;
#[doc = "Field `GPIOIRST` writer - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJRST` reader - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOJRST_R = crate::BitReader;
#[doc = "Field `GPIOJRST` writer - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOJRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12RST` reader - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
pub type ADC12RST_R = crate::BitReader;
#[doc = "Field `ADC12RST` writer - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
pub type ADC12RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMI_PSSIRST` reader - DCMI and PSSI reset This bit is set and cleared by software."]
pub type DCMI_PSSIRST_R = crate::BitReader;
#[doc = "Field `DCMI_PSSIRST` writer - DCMI and PSSI reset This bit is set and cleared by software."]
pub type DCMI_PSSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGRST` reader - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGRST_R = crate::BitReader;
#[doc = "Field `OTGRST` writer - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESRST` reader - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type AESRST_R = crate::BitReader;
#[doc = "Field `AESRST` writer - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHRST` reader - HASH reset This bit is set and cleared by software."]
pub type HASHRST_R = crate::BitReader;
#[doc = "Field `HASHRST` writer - HASH reset This bit is set and cleared by software."]
pub type HASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - RNG reset This bit is set and cleared by software."]
pub type RNGRST_R = crate::BitReader;
#[doc = "Field `RNGRST` writer - RNG reset This bit is set and cleared by software."]
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKARST` reader - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type PKARST_R = crate::BitReader;
#[doc = "Field `PKARST` writer - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAESRST` reader - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAESRST_R = crate::BitReader;
#[doc = "Field `SAESRST` writer - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAESRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPIMRST` reader - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPIMRST_R = crate::BitReader;
#[doc = "Field `OCTOSPIMRST` writer - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTFDEC1RST` reader - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC1RST_R = crate::BitReader;
#[doc = "Field `OTFDEC1RST` writer - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTFDEC2RST` reader - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC2RST_R = crate::BitReader;
#[doc = "Field `OTFDEC2RST` writer - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 reset This bit is set and cleared by software."]
pub type SDMMC1RST_R = crate::BitReader;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 reset This bit is set and cleared by software."]
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SDMMC2RST_R = crate::BitReader;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port G reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port H reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DCMI and PSSI reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otgrst(&self) -> OTGRST_R {
        OTGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn octospimrst(&self) -> OCTOSPIMRST_R {
        OCTOSPIMRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otfdec1rst(&self) -> OTFDEC1RST_R {
        OTFDEC1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otfdec2rst(&self) -> OTFDEC2RST_R {
        OTFDEC2RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - SDMMC1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<RCC_AHB2RSTR1rs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<RCC_AHB2RSTR1rs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<RCC_AHB2RSTR1rs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<RCC_AHB2RSTR1rs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<RCC_AHB2RSTR1rs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F reset This bit is set and cleared by software. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<RCC_AHB2RSTR1rs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - I/O port G reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<RCC_AHB2RSTR1rs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - I/O port H reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<RCC_AHB2RSTR1rs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - I/O port I reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<RCC_AHB2RSTR1rs> {
        GPIOIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<RCC_AHB2RSTR1rs> {
        GPIOJRST_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<RCC_AHB2RSTR1rs> {
        ADC12RST_W::new(self, 10)
    }
    #[doc = "Bit 12 - DCMI and PSSI reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<RCC_AHB2RSTR1rs> {
        DCMI_PSSIRST_W::new(self, 12)
    }
    #[doc = "Bit 14 - OTG_FS or OTG_HS reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otgrst(&mut self) -> OTGRST_W<RCC_AHB2RSTR1rs> {
        OTGRST_W::new(self, 14)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<RCC_AHB2RSTR1rs> {
        AESRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<RCC_AHB2RSTR1rs> {
        HASHRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<RCC_AHB2RSTR1rs> {
        RNGRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - PKA reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<RCC_AHB2RSTR1rs> {
        PKARST_W::new(self, 19)
    }
    #[doc = "Bit 20 - SAES hardware accelerator reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn saesrst(&mut self) -> SAESRST_W<RCC_AHB2RSTR1rs> {
        SAESRST_W::new(self, 20)
    }
    #[doc = "Bit 21 - OCTOSPIM reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn octospimrst(&mut self) -> OCTOSPIMRST_W<RCC_AHB2RSTR1rs> {
        OCTOSPIMRST_W::new(self, 21)
    }
    #[doc = "Bit 23 - OTFDEC1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1rst(&mut self) -> OTFDEC1RST_W<RCC_AHB2RSTR1rs> {
        OTFDEC1RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - OTFDEC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otfdec2rst(&mut self) -> OTFDEC2RST_W<RCC_AHB2RSTR1rs> {
        OTFDEC2RST_W::new(self, 24)
    }
    #[doc = "Bit 27 - SDMMC1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<RCC_AHB2RSTR1rs> {
        SDMMC1RST_W::new(self, 27)
    }
    #[doc = "Bit 28 - SDMMC2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<RCC_AHB2RSTR1rs> {
        SDMMC2RST_W::new(self, 28)
    }
}
#[doc = "RCC AHB2 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB2RSTR1rs;
impl crate::RegisterSpec for RCC_AHB2RSTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2rstr1::R`](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTR1rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2rstr1::W`](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB2RSTR1 to value 0"]
impl crate::Resettable for RCC_AHB2RSTR1rs {
    const RESET_VALUE: u32 = 0;
}
