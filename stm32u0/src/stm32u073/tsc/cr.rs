///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `TSCE` reader - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect.
pub type TSCE_R = crate::BitReader;
///Field `TSCE` writer - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect.
pub type TSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition.
pub type START_R = crate::BitReader;
///Field `START` writer - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AM` reader - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing.
pub type AM_R = crate::BitReader;
///Field `AM` writer - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing.
pub type AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCPOL` reader - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin.
pub type SYNCPOL_R = crate::BitReader;
///Field `SYNCPOL` writer - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin.
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IODEF` reader - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing.
pub type IODEF_R = crate::BitReader;
///Field `IODEF` writer - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing.
pub type IODEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCV` reader - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing.
pub type MCV_R = crate::FieldReader;
///Field `MCV` writer - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing.
pub type MCV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PGPSC` reader - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
pub type PGPSC_R = crate::FieldReader;
///Field `PGPSC` writer - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
pub type PGPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SSPSC` reader - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing.
pub type SSPSC_R = crate::BitReader;
///Field `SSPSC` writer - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing.
pub type SSPSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSE` reader - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing.
pub type SSE_R = crate::BitReader;
///Field `SSE` writer - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing.
pub type SSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSD` reader - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing.
pub type SSD_R = crate::FieldReader;
///Field `SSD` writer - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing.
pub type SSD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CTPL` reader - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C<sub>X</sub> to C<sub>S</sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
pub type CTPL_R = crate::FieldReader;
///Field `CTPL` writer - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C<sub>X</sub> to C<sub>S</sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
pub type CTPL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CTPH` reader - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C<sub>X</sub>). ... Note: These bits must not be modified when an acquisition is ongoing.
pub type CTPH_R = crate::FieldReader;
///Field `CTPH` writer - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C<sub>X</sub>). ... Note: These bits must not be modified when an acquisition is ongoing.
pub type CTPH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect.
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin.
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 12:14 - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bits 24:27 - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C<sub>X</sub> to C<sub>S</sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C<sub>X</sub>). ... Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("tsce", &self.tsce())
            .field("start", &self.start())
            .field("am", &self.am())
            .field("syncpol", &self.syncpol())
            .field("iodef", &self.iodef())
            .field("mcv", &self.mcv())
            .field("pgpsc", &self.pgpsc())
            .field("sspsc", &self.sspsc())
            .field("sse", &self.sse())
            .field("ssd", &self.ssd())
            .field("ctpl", &self.ctpl())
            .field("ctph", &self.ctph())
            .finish()
    }
}
impl W {
    ///Bit 0 - Touch sensing controller enable This bit is set and cleared by software to enable/disable the touch sensing controller. Note: When the touch sensing controller is disabled, TSC registers settings have no effect.
    #[inline(always)]
    pub fn tsce(&mut self) -> TSCE_W<CRrs> {
        TSCE_W::new(self, 0)
    }
    ///Bit 1 - Start a new acquisition This bit is set by software to start a new acquisition. It is cleared by hardware as soon as the acquisition is complete or by software to cancel the ongoing acquisition.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    ///Bit 2 - Acquisition mode This bit is set and cleared by software to select the acquisition mode. Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<CRrs> {
        AM_W::new(self, 2)
    }
    ///Bit 3 - Synchronization pin polarity This bit is set and cleared by software to select the polarity of the synchronization input pin.
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CRrs> {
        SYNCPOL_W::new(self, 3)
    }
    ///Bit 4 - I/O Default mode This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when there is no ongoing acquisition. When there is an ongoing acquisition, it defines the configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O). Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn iodef(&mut self) -> IODEF_W<CRrs> {
        IODEF_W::new(self, 4)
    }
    ///Bits 5:7 - Max count value These bits are set and cleared by software. They define the maximum number of charge transfer pulses that can be generated before a max count error is generated. Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn mcv(&mut self) -> MCV_W<CRrs> {
        MCV_W::new(self, 5)
    }
    ///Bits 12:14 - Pulse generator prescaler These bits are set and cleared by software.They select the AHB clock divider used to generate the pulse generator clock (PGCLK). Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
    #[inline(always)]
    pub fn pgpsc(&mut self) -> PGPSC_W<CRrs> {
        PGPSC_W::new(self, 12)
    }
    ///Bit 15 - Spread spectrum prescaler This bit is set and cleared by software. It selects the AHB clock divider used to generate the spread spectrum clock (SSCLK). Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn sspsc(&mut self) -> SSPSC_W<CRrs> {
        SSPSC_W::new(self, 15)
    }
    ///Bit 16 - Spread spectrum enable This bit is set and cleared by software to enable/disable the spread spectrum feature. Note: This bit must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W<CRrs> {
        SSE_W::new(self, 16)
    }
    ///Bits 17:23 - Spread spectrum deviation These bits are set and cleared by software. They define the spread spectrum deviation which consists in adding a variable number of periods of the SSCLK clock to the charge transfer pulse high state. ... Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W<CRrs> {
        SSD_W::new(self, 17)
    }
    ///Bits 24:27 - Charge transfer pulse low These bits are set and cleared by software. They define the duration of the low state of the charge transfer pulse (transfer of charge from C<sub>X</sub> to C<sub>S</sub>). ... Note: These bits must not be modified when an acquisition is ongoing. Note: Some configurations are forbidden. Refer to the Section119.4.4: Charge transfer acquisition sequence for details.
    #[inline(always)]
    pub fn ctpl(&mut self) -> CTPL_W<CRrs> {
        CTPL_W::new(self, 24)
    }
    ///Bits 28:31 - Charge transfer pulse high These bits are set and cleared by software. They define the duration of the high state of the charge transfer pulse (charge of C<sub>X</sub>). ... Note: These bits must not be modified when an acquisition is ongoing.
    #[inline(always)]
    pub fn ctph(&mut self) -> CTPH_W<CRrs> {
        CTPH_W::new(self, 28)
    }
}
/**TSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TSC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
