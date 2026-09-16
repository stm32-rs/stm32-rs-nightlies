///Register `WKUPEPR` reader
pub type R = crate::R<WKUPEPRrs>;
///Register `WKUPEPR` writer
pub type W = crate::W<WKUPEPRrs>;
///Field `WKUPEN1` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN1_R = crate::BitReader;
///Field `WKUPEN1` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN2` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN2_R = crate::BitReader;
///Field `WKUPEN2` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN3` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN3_R = crate::BitReader;
///Field `WKUPEN3` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN4` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN4_R = crate::BitReader;
///Field `WKUPEN4` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN5` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN5_R = crate::BitReader;
///Field `WKUPEN5` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN6` reader - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN6_R = crate::BitReader;
///Field `WKUPEN6` writer - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
pub type WKUPEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP1` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP1_R = crate::BitReader;
///Field `WKUPP1` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP2` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP2_R = crate::BitReader;
///Field `WKUPP2` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP3` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP3_R = crate::BitReader;
///Field `WKUPP3` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP4` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP4_R = crate::BitReader;
///Field `WKUPP4` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP5` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP5_R = crate::BitReader;
///Field `WKUPP5` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP6` reader - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP6_R = crate::BitReader;
///Field `WKUPP6` writer - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
pub type WKUPP6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPUPD1` reader - Wakeup pin pull configuration
pub type WKUPPUPD1_R = crate::FieldReader;
///Field `WKUPPUPD1` writer - Wakeup pin pull configuration
pub type WKUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD2` reader - Wakeup pin pull configuration
pub type WKUPPUPD2_R = crate::FieldReader;
///Field `WKUPPUPD2` writer - Wakeup pin pull configuration
pub type WKUPPUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD3` reader - Wakeup pin pull configuration
pub type WKUPPUPD3_R = crate::FieldReader;
///Field `WKUPPUPD3` writer - Wakeup pin pull configuration
pub type WKUPPUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD4` reader - Wakeup pin pull configuration
pub type WKUPPUPD4_R = crate::FieldReader;
///Field `WKUPPUPD4` writer - Wakeup pin pull configuration
pub type WKUPPUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD5` reader - Wakeup pin pull configuration
pub type WKUPPUPD5_R = crate::FieldReader;
///Field `WKUPPUPD5` writer - Wakeup pin pull configuration
pub type WKUPPUPD5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD6` reader - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode.
pub type WKUPPUPD6_R = crate::FieldReader;
///Field `WKUPPUPD6` writer - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode.
pub type WKUPPUPD6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPEPR")
            .field("wkupen1", &self.wkupen1())
            .field("wkupen2", &self.wkupen2())
            .field("wkupen3", &self.wkupen3())
            .field("wkupen4", &self.wkupen4())
            .field("wkupen5", &self.wkupen5())
            .field("wkupen6", &self.wkupen6())
            .field("wkupp1", &self.wkupp1())
            .field("wkupp2", &self.wkupp2())
            .field("wkupp3", &self.wkupp3())
            .field("wkupp4", &self.wkupp4())
            .field("wkupp5", &self.wkupp5())
            .field("wkupp6", &self.wkupp6())
            .field("wkuppupd1", &self.wkuppupd1())
            .field("wkuppupd2", &self.wkuppupd2())
            .field("wkuppupd3", &self.wkuppupd3())
            .field("wkuppupd4", &self.wkuppupd4())
            .field("wkuppupd5", &self.wkuppupd5())
            .field("wkuppupd6", &self.wkuppupd6())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<'_, WKUPEPRrs> {
        WKUPEN1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<'_, WKUPEPRrs> {
        WKUPEN2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<'_, WKUPEPRrs> {
        WKUPEN3_W::new(self, 2)
    }
    ///Bit 3 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<'_, WKUPEPRrs> {
        WKUPEN4_W::new(self, 3)
    }
    ///Bit 4 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<'_, WKUPEPRrs> {
        WKUPEN5_W::new(self, 4)
    }
    ///Bit 5 - Enable Wakeup Pin WKUPn+1 Each bit is set and cleared by software. Note: An additional wakeup event is detected if WKUPn+1 pin is enabled (by setting the WKUPENn+1 bit) when WKUPn+1 pin level is already high when WKUPPn+1 selects rising edge, or low when WKUPPn+1 selects falling edge.
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<'_, WKUPEPRrs> {
        WKUPEN6_W::new(self, 5)
    }
    ///Bit 8 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W<'_, WKUPEPRrs> {
        WKUPP1_W::new(self, 8)
    }
    ///Bit 9 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W<'_, WKUPEPRrs> {
        WKUPP2_W::new(self, 9)
    }
    ///Bit 10 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W<'_, WKUPEPRrs> {
        WKUPP3_W::new(self, 10)
    }
    ///Bit 11 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W<'_, WKUPEPRrs> {
        WKUPP4_W::new(self, 11)
    }
    ///Bit 12 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W<'_, WKUPEPRrs> {
        WKUPP5_W::new(self, 12)
    }
    ///Bit 13 - Wakeup pin polarity bit for WKUPn-7 These bits define the polarity used for event detection on WKUPn-7 external wakeup pin.
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W<'_, WKUPEPRrs> {
        WKUPP6_W::new(self, 13)
    }
    ///Bits 16:17 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W<'_, WKUPEPRrs> {
        WKUPPUPD1_W::new(self, 16)
    }
    ///Bits 18:19 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W<'_, WKUPEPRrs> {
        WKUPPUPD2_W::new(self, 18)
    }
    ///Bits 20:21 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W<'_, WKUPEPRrs> {
        WKUPPUPD3_W::new(self, 20)
    }
    ///Bits 22:23 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W<'_, WKUPEPRrs> {
        WKUPPUPD4_W::new(self, 22)
    }
    ///Bits 24:25 - Wakeup pin pull configuration
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W<'_, WKUPEPRrs> {
        WKUPPUPD5_W::new(self, 24)
    }
    ///Bits 26:27 - Wakeup pin pull configuration for WKUP(truncate(n/2)-7) These bits define the I/O pad pull configuration used when WKUPEN(truncate(n/2)-7) = 1. The associated GPIO port pull configuration shall be set to the same value or to 00. The Wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W<'_, WKUPEPRrs> {
        WKUPPUPD6_W::new(self, 26)
    }
}
/**Reset only by system reset, not reset by wakeup from Standby mode

You can [`read`](crate::Reg::read) this register and get [`wkupepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#PWR:WKUPEPR)*/
pub struct WKUPEPRrs;
impl crate::RegisterSpec for WKUPEPRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupepr::R`](R) reader structure
impl crate::Readable for WKUPEPRrs {}
///`write(|w| ..)` method takes [`wkupepr::W`](W) writer structure
impl crate::Writable for WKUPEPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPEPR to value 0
impl crate::Resettable for WKUPEPRrs {}
