#[doc = "Register `RCC_AHB2ENR1` reader"]
pub type R = crate::R<RCC_AHB2ENR1rs>;
#[doc = "Register `RCC_AHB2ENR1` writer"]
pub type W = crate::W<RCC_AHB2ENR1rs>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software."]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software."]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software."]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software."]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software."]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software."]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software."]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software."]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - I/O port E clock enable This bit is set and cleared by software."]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - I/O port E clock enable This bit is set and cleared by software."]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - I/O port G clock enable This bit is set and cleared by software."]
pub type GPIOGEN_R = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - I/O port G clock enable This bit is set and cleared by software."]
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - I/O port H clock enable This bit is set and cleared by software."]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - I/O port H clock enable This bit is set and cleared by software."]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIEN` reader - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOIEN_R = crate::BitReader;
#[doc = "Field `GPIOIEN` writer - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJEN` reader - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOJEN_R = crate::BitReader;
#[doc = "Field `GPIOJEN` writer - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GPIOJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12EN` reader - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
pub type ADC12EN_R = crate::BitReader;
#[doc = "Field `ADC12EN` writer - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
pub type ADC12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMI_PSSIEN` reader - DCMI and PSSI clock enable This bit is set and cleared by software."]
pub type DCMI_PSSIEN_R = crate::BitReader;
#[doc = "Field `DCMI_PSSIEN` writer - DCMI and PSSI clock enable This bit is set and cleared by software."]
pub type DCMI_PSSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGEN` reader - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGEN_R = crate::BitReader;
#[doc = "Field `OTGEN` writer - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGHSPHYEN` reader - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGHSPHYEN_R = crate::BitReader;
#[doc = "Field `OTGHSPHYEN` writer - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTGHSPHYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type AESEN_R = crate::BitReader;
#[doc = "Field `AESEN` writer - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHEN` reader - HASH clock enable This bit is set and cleared by software"]
pub type HASHEN_R = crate::BitReader;
#[doc = "Field `HASHEN` writer - HASH clock enable This bit is set and cleared by software"]
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - RNG clock enable This bit is set and cleared by software."]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - RNG clock enable This bit is set and cleared by software."]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAEN` reader - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type PKAEN_R = crate::BitReader;
#[doc = "Field `PKAEN` writer - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAESEN` reader - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAESEN_R = crate::BitReader;
#[doc = "Field `SAESEN` writer - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPIMEN` reader - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPIMEN_R = crate::BitReader;
#[doc = "Field `OCTOSPIMEN` writer - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OCTOSPIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTFDEC1EN` reader - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC1EN_R = crate::BitReader;
#[doc = "Field `OTFDEC1EN` writer - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTFDEC2EN` reader - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC2EN_R = crate::BitReader;
#[doc = "Field `OTFDEC2EN` writer - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type OTFDEC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 clock enable This bit is set and cleared by software."]
pub type SDMMC1EN_R = crate::BitReader;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 clock enable This bit is set and cleared by software."]
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SDMMC2EN_R = crate::BitReader;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SDMMC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2EN` reader - SRAM2 clock enable This bit is set and reset by software."]
pub type SRAM2EN_R = crate::BitReader;
#[doc = "Field `SRAM2EN` writer - SRAM2 clock enable This bit is set and reset by software."]
pub type SRAM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3EN` reader - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM3EN_R = crate::BitReader;
#[doc = "Field `SRAM3EN` writer - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SRAM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port G clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port H clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DCMI and PSSI clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otghsphyen(&self) -> OTGHSPHYEN_R {
        OTGHSPHYEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable This bit is set and cleared by software"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn saesen(&self) -> SAESEN_R {
        SAESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn octospimen(&self) -> OCTOSPIMEN_R {
        OCTOSPIMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otfdec1en(&self) -> OTFDEC1EN_R {
        OTFDEC1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn otfdec2en(&self) -> OTFDEC2EN_R {
        OTFDEC2EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - SDMMC1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable This bit is set and reset by software."]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<RCC_AHB2ENR1rs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<RCC_AHB2ENR1rs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<RCC_AHB2ENR1rs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<RCC_AHB2ENR1rs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<RCC_AHB2ENR1rs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<RCC_AHB2ENR1rs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - I/O port G clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<RCC_AHB2ENR1rs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I/O port H clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<RCC_AHB2ENR1rs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - I/O port I clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<RCC_AHB2ENR1rs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<RCC_AHB2ENR1rs> {
        GPIOJEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<RCC_AHB2ENR1rs> {
        ADC12EN_W::new(self, 10)
    }
    #[doc = "Bit 12 - DCMI and PSSI clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<RCC_AHB2ENR1rs> {
        DCMI_PSSIEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - OTG_FS or OTG_HS clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otgen(&mut self) -> OTGEN_W<RCC_AHB2ENR1rs> {
        OTGEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otghsphyen(&mut self) -> OTGHSPHYEN_W<RCC_AHB2ENR1rs> {
        OTGHSPHYEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - AES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<RCC_AHB2ENR1rs> {
        AESEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable This bit is set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<RCC_AHB2ENR1rs> {
        HASHEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<RCC_AHB2ENR1rs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - PKA clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<RCC_AHB2ENR1rs> {
        PKAEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SAES clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn saesen(&mut self) -> SAESEN_W<RCC_AHB2ENR1rs> {
        SAESEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - OCTOSPIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn octospimen(&mut self) -> OCTOSPIMEN_W<RCC_AHB2ENR1rs> {
        OCTOSPIMEN_W::new(self, 21)
    }
    #[doc = "Bit 23 - OTFDEC1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1en(&mut self) -> OTFDEC1EN_W<RCC_AHB2ENR1rs> {
        OTFDEC1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - OTFDEC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn otfdec2en(&mut self) -> OTFDEC2EN_W<RCC_AHB2ENR1rs> {
        OTFDEC2EN_W::new(self, 24)
    }
    #[doc = "Bit 27 - SDMMC1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<RCC_AHB2ENR1rs> {
        SDMMC1EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - SDMMC2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<RCC_AHB2ENR1rs> {
        SDMMC2EN_W::new(self, 28)
    }
    #[doc = "Bit 30 - SRAM2 clock enable This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<RCC_AHB2ENR1rs> {
        SRAM2EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM3 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<RCC_AHB2ENR1rs> {
        SRAM3EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB2 peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2enr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2enr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB2ENR1rs;
impl crate::RegisterSpec for RCC_AHB2ENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2enr1::R`](R) reader structure"]
impl crate::Readable for RCC_AHB2ENR1rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2enr1::W`](W) writer structure"]
impl crate::Writable for RCC_AHB2ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB2ENR1 to value 0xc000_0000"]
impl crate::Resettable for RCC_AHB2ENR1rs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
