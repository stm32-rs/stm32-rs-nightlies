///Register `FCR` reader
pub type R = crate::R<FCRrs>;
///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `HD` reader - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated.
pub type HD_R = crate::BitReader;
///Field `HD` writer - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated.
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFIE` reader - Start of frame interrupt enable This bit is set and cleared by software.
pub type SOFIE_R = crate::BitReader;
///Field `SOFIE` writer - Start of frame interrupt enable This bit is set and cleared by software.
pub type SOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDDIE` reader - Update display done interrupt enable This bit is set and cleared by software.
pub type UDDIE_R = crate::BitReader;
///Field `UDDIE` writer - Update display done interrupt enable This bit is set and cleared by software.
pub type UDDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PON` reader - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:
pub type PON_R = crate::FieldReader;
///Field `PON` writer - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:
pub type PON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DEAD` reader - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......
pub type DEAD_R = crate::FieldReader;
///Field `DEAD` writer - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......
pub type DEAD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CC` reader - Contrast control These bits specify one of the V<sub>LCD </sub>maximum voltages (independent of V<sub>DD</sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V<sub>LCDx</sub> values.
pub type CC_R = crate::FieldReader;
///Field `CC` writer - Contrast control These bits specify one of the V<sub>LCD </sub>maximum voltages (independent of V<sub>DD</sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V<sub>LCDx</sub> values.
pub type CC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BLINKF` reader - Blink frequency selection
pub type BLINKF_R = crate::FieldReader;
///Field `BLINKF` writer - Blink frequency selection
pub type BLINKF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BLINK` reader - Blink mode selection
pub type BLINK_R = crate::FieldReader;
///Field `BLINK` writer - Blink mode selection
pub type BLINK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIV` reader - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ...
pub type DIV_R = crate::FieldReader;
///Field `DIV` writer - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ...
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PS` reader - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2<sup>PS\[3:0\]</sup>). See<sub> </sub>Section118.3.2. ...
pub type PS_R = crate::FieldReader;
///Field `PS` writer - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2<sup>PS\[3:0\]</sup>). See<sub> </sub>Section118.3.2. ...
pub type PS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated.
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of frame interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Update display done interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:12 - Contrast control These bits specify one of the V<sub>LCD </sub>maximum voltages (independent of V<sub>DD</sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V<sub>LCDx</sub> values.
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ...
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2<sup>PS\[3:0\]</sup>). See<sub> </sub>Section118.3.2. ...
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("hd", &self.hd())
            .field("sofie", &self.sofie())
            .field("uddie", &self.uddie())
            .field("pon", &self.pon())
            .field("dead", &self.dead())
            .field("cc", &self.cc())
            .field("blinkf", &self.blinkf())
            .field("blink", &self.blink())
            .field("div", &self.div())
            .field("ps", &self.ps())
            .finish()
    }
}
impl W {
    ///Bit 0 - High drive enable This bit is written by software to enable a low resistance divider. Displays with high internal resistance may need a longer drive time to achieve satisfactory contrast. This bit is useful in this case if some additional power consumption can be tolerated.
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W<'_, FCRrs> {
        HD_W::new(self, 0)
    }
    ///Bit 1 - Start of frame interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W<'_, FCRrs> {
        SOFIE_W::new(self, 1)
    }
    ///Bit 3 - Update display done interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn uddie(&mut self) -> UDDIE_W<'_, FCRrs> {
        UDDIE_W::new(self, 3)
    }
    ///Bits 4:6 - Pulse ON duration These bits are written by software to define the pulse duration in terms of ck_ps pulses. A1short pulse leads to lower power consumption, but displays with high internal resistance may need a longer pulse to achieve satisfactory contrast. Note that the pulse is never longer than one half prescaled LCD clock period. PON duration example with LCDCLK = 32.7681kHz and PS=0x03:
    #[inline(always)]
    pub fn pon(&mut self) -> PON_W<'_, FCRrs> {
        PON_W::new(self, 4)
    }
    ///Bits 7:9 - Dead time duration These bits are written by software to configure the length of the dead time between frames. During the dead time the COM and SEG voltage levels are held at 0 V to reduce the contrast without modifying the frame rate. ......
    #[inline(always)]
    pub fn dead(&mut self) -> DEAD_W<'_, FCRrs> {
        DEAD_W::new(self, 7)
    }
    ///Bits 10:12 - Contrast control These bits specify one of the V<sub>LCD </sub>maximum voltages (independent of V<sub>DD</sub>). It ranges from12.60 V to 3.51V. Note: Refer to the datasheet for the V<sub>LCDx</sub> values.
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W<'_, FCRrs> {
        CC_W::new(self, 10)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&mut self) -> BLINKF_W<'_, FCRrs> {
        BLINKF_W::new(self, 13)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&mut self) -> BLINK_W<'_, FCRrs> {
        BLINK_W::new(self, 16)
    }
    ///Bits 18:21 - DIV clock divider These bits are written by software to define the division factor of the DIV divider (see1Section118.3.2.) ...
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<'_, FCRrs> {
        DIV_W::new(self, 18)
    }
    ///Bits 22:25 - PS 16-bit prescaler These bits are written by software to define the division factor of the PS 16-bit prescaler. ck_ps = LCDCLK/(2<sup>PS\[3:0\]</sup>). See<sub> </sub>Section118.3.2. ...
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, FCRrs> {
        PS_W::new(self, 22)
    }
}
/**LCD frame control register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LCD:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FCRrs {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
